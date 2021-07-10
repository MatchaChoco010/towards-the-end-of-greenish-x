use futures::task::ArcWake;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::future::Future;
use std::ops::{Add, AddAssign};
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub struct JoinHandle<T> {
    value: Rc<RefCell<Option<T>>>,
    register_handle_waker: Box<dyn Fn(Waker) -> ()>,
}
impl<T> Future for JoinHandle<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(val) = self.value.borrow_mut().take() {
            Poll::Ready(val)
        } else {
            let waker = cx.waker().clone();
            (self.register_handle_waker)(waker);
            Poll::Pending
        }
    }
}

pub struct Task {
    id: Uuid,
    future: Pin<Box<dyn Future<Output = ()>>>,
    handle_waker: Rc<RefCell<Option<Waker>>>,
}
impl Task {
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        Future::poll(self.future.as_mut(), cx)
    }
}

fn joinable<F>(future: F) -> (Task, JoinHandle<F::Output>)
where
    F: Future + 'static,
    F::Output: 'static,
{
    let value = Rc::new(RefCell::new(None));

    let task = {
        let value = Rc::clone(&value);
        Task {
            future: Box::pin(async move {
                let output = future.await;
                value.borrow_mut().replace(output);
            }),
            handle_waker: Rc::new(RefCell::new(None)),
            id: Uuid::new_v4(),
        }
    };

    let register_handle_waker = Box::new({
        let handle_waker = Rc::clone(&task.handle_waker);
        move |waker| {
            handle_waker.borrow_mut().replace(waker);
        }
    });
    let handle = JoinHandle {
        value,
        register_handle_waker,
    };

    (task, handle)
}

#[derive(PartialEq, Eq, Debug)]
pub enum StepState {
    RemainTasks,
    Completed,
}

struct InnerExecutor {
    running_tasks: RefCell<VecDeque<Task>>,
    wait_tasks: RefCell<HashMap<Uuid, Task>>,
    waker_sender: Sender<Uuid>,
    waker_receiver: Receiver<Uuid>,
    next_frame_waker_list: RefCell<Vec<Waker>>,
    current_time: RefCell<Instant>,
    delay_waker_heap: RefCell<BinaryHeap<Timeout>>,
}
impl InnerExecutor {
    fn new() -> Self {
        let (waker_sender, waker_receiver) = channel();
        Self {
            running_tasks: Default::default(),
            wait_tasks: Default::default(),
            waker_sender,
            waker_receiver,
            next_frame_waker_list: RefCell::new(vec![]),
            current_time: RefCell::new(Instant::now()),
            delay_waker_heap: RefCell::new(BinaryHeap::new()),
        }
    }
}

#[derive(Clone)]
pub struct Executor {
    inner: Rc<InnerExecutor>,
}
impl Executor {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(InnerExecutor::new()),
        }
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let (task, handle) = joinable(future);
        self.inner.running_tasks.borrow_mut().push_back(task);
        handle
    }

    pub fn step(&self, delta_time: Duration) -> StepState {
        // Update time
        self.inner.current_time.borrow_mut().add_assign(delta_time);

        // Frame start event
        self.inner
            .next_frame_waker_list
            .borrow_mut()
            .drain(0..)
            .for_each(|w| w.wake());
        // Delta time Event
        {
            let mut heap = self.inner.delay_waker_heap.borrow_mut();
            while let Some(timeout) = heap.pop() {
                if timeout.instant >= *self.inner.current_time.borrow() {
                    heap.push(timeout);
                    break;
                }
                timeout.waker.wake();
            }
        }

        // poll loop
        'current_frame: loop {
            for id in self.inner.waker_receiver.try_iter() {
                if let Some(task) = self.inner.wait_tasks.borrow_mut().remove(&id) {
                    self.inner.running_tasks.borrow_mut().push_back(task);
                }
            }

            let task = self.inner.running_tasks.borrow_mut().pop_front();
            match task {
                None => break 'current_frame,
                Some(mut task) => {
                    let unpark = Box::new({
                        let id = task.id;
                        let sender = Mutex::new(self.inner.waker_sender.clone());
                        move || {
                            if let Ok(s) = sender.lock() {
                                s.send(id).unwrap();
                            }
                        }
                    });
                    let waker = TaskWaker::waker(unpark);
                    let mut cx = Context::from_waker(&waker);

                    match task.poll(&mut cx) {
                        Poll::Ready(_) => {
                            // Send a notification to handle
                            if let Some(handle_waker) = task.handle_waker.borrow_mut().take() {
                                handle_waker.wake_by_ref();
                            }
                        }
                        Poll::Pending => {
                            // I'm sure this will never happen, but if a UUid is duplicated,
                            // re-generate the UUid.
                            while let Some(_) = self.inner.wait_tasks.borrow_mut().get(&task.id) {
                                task.id = Uuid::new_v4();
                            }
                            // park the task
                            self.inner.wait_tasks.borrow_mut().insert(task.id, task);
                        }
                    }
                }
            }
        }

        if self.inner.wait_tasks.borrow().is_empty() {
            StepState::Completed
        } else {
            StepState::RemainTasks
        }
    }

    pub fn next_frame(&self) -> WaitNextFrameFuture {
        WaitNextFrameFuture::new(&self.inner)
    }

    pub fn delay(&self, delay: Duration) -> DelayFuture {
        DelayFuture::new(&self.inner, delay)
    }
}

struct TaskWaker {
    unpark: Box<dyn Fn() -> () + Send + Sync>,
}
impl TaskWaker {
    fn waker(unpark: Box<dyn Fn() -> () + Send + Sync>) -> Waker {
        futures::task::waker(Arc::new(TaskWaker { unpark }))
    }
}
impl ArcWake for TaskWaker {
    fn wake_by_ref(arc_self: &std::sync::Arc<Self>) {
        (arc_self.unpark)();
    }
}

pub struct WaitNextFrameFuture {
    flag: bool,
    executor: Weak<InnerExecutor>,
}
impl WaitNextFrameFuture {
    fn new(executor: &Rc<InnerExecutor>) -> Self {
        Self {
            flag: false,
            executor: Rc::downgrade(executor),
        }
    }
}
impl Future for WaitNextFrameFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(executor) = self.executor.upgrade() {
            if self.flag {
                Poll::Ready(())
            } else {
                self.get_mut().flag = true;
                let waker = cx.waker().clone();
                executor.next_frame_waker_list.borrow_mut().push(waker);
                Poll::Pending
            }
        } else {
            unreachable!()
        }
    }
}

struct Timeout {
    instant: Instant,
    waker: Waker,
}
impl PartialEq for Timeout {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}
impl Eq for Timeout {}
impl Ord for Timeout {
    fn cmp(&self, other: &Timeout) -> Ordering {
        self.instant.cmp(&other.instant).reverse() // for max-heap to min-heap
    }
}
impl PartialOrd for Timeout {
    fn partial_cmp(&self, other: &Timeout) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

pub struct DelayFuture {
    instant: Instant,
    executor: Weak<InnerExecutor>,
}
impl DelayFuture {
    fn new(executor: &Rc<InnerExecutor>, delay: Duration) -> Self {
        Self {
            instant: executor.current_time.borrow().add(delay),
            executor: Rc::downgrade(executor),
        }
    }
}
impl Future for DelayFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(executor) = self.executor.upgrade() {
            if self.instant < *executor.current_time.borrow() {
                Poll::Ready(())
            } else {
                executor.delay_waker_heap.borrow_mut().push(Timeout {
                    instant: self.instant,
                    waker: cx.waker().clone(),
                });
                Poll::Pending
            }
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::FutureExt;
    use futures::{join, select};

    #[test]
    fn it_should_run_async_block_in_single_step() {
        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            async move {
                *flag.borrow_mut() = true;
            }
        });
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_run_nested_async_block_and_await_in_single_step() {
        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            async move {
                let result = async { 1 + async { 2 + 3 }.await }.await;
                assert_eq!(result, 6);
                *flag.borrow_mut() = true;
            }
        });
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_run_multiple_await_in_single_step() {
        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            async move {
                let result = async { 1 + 2 }.await;
                assert_eq!(result, 3);
                let result = async { 3 + 4 }.await;
                assert_eq!(result, 7);
                *flag.borrow_mut() = true;
            }
        });
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_run_nested_spawn_in_single_step() {
        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            let executor = executor.clone();
            async move {
                let handle = executor.spawn(async { 1 + 2 });
                let result = handle.await;
                assert_eq!(result, 3);
                *flag.borrow_mut() = true;
            }
        });
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_finish_in_2_step_when_1_next_frame_await() {
        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            let executor = executor.clone();
            async move {
                executor.next_frame().await;
                *flag.borrow_mut() = true;
            }
        });
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_finish_in_11_step_when_10_next_frame_await() {
        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            let executor = executor.clone();
            async move {
                for _ in 0..10 {
                    executor.next_frame().await;
                }
                *flag.borrow_mut() = true;
            }
        });

        for _ in 0..10 {
            let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
            assert_eq!(&*flag.borrow(), &false);
            assert_eq!(result, StepState::RemainTasks);
        }
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn select_macro_should_work_as_expected() {
        async fn async_fn_1(executor: Executor) {
            executor.next_frame().await;
        }
        async fn async_fn_2(executor: Executor) {
            executor.next_frame().await;
            executor.next_frame().await;
        }

        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            let executor = executor.clone();
            async move {
                select! {
                    () = async_fn_1(executor.clone()).fuse() => (),
                    () = async_fn_2(executor.clone()).fuse() => (),
                };
                *flag.borrow_mut() = true;
            }
        });

        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);

        assert_eq!(Rc::strong_count(&executor.inner), 1);
    }

    #[test]
    fn join_macro_should_work_as_expected() {
        async fn async_fn_1(executor: Executor) {
            executor.next_frame().await;
        }
        async fn async_fn_2(executor: Executor) {
            executor.next_frame().await;
            executor.next_frame().await;
        }

        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            let executor = executor.clone();
            async move {
                join!(async_fn_1(executor.clone()), async_fn_2(executor.clone()),);
                *flag.borrow_mut() = true;
            }
        });

        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);

        assert_eq!(Rc::strong_count(&executor.inner), 1);
    }

    #[test]
    fn delay_future_should_wake_after_delay_time() {
        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            let executor = executor.clone();
            async move {
                executor.delay(Duration::from_millis(32)).await;
                *flag.borrow_mut() = true;
            }
        });

        let result = executor.step(Duration::from_secs_f64(0.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn multi_delay_future_should_wake_after_delay_time() {
        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            let executor = executor.clone();
            async move {
                executor.delay(Duration::from_millis(16)).await;
                executor.delay(Duration::from_millis(16)).await;
                select! {
                    () = executor.delay(Duration::from_millis(16)).fuse() => (),
                    () = executor.delay(Duration::from_millis(32)).fuse() => (),
                }
                *flag.borrow_mut() = true;
            }
        });

        let result = executor.step(Duration::from_secs_f64(0.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn delay_and_next_frame_await() {
        let flag = Rc::new(RefCell::new(false));
        let executor = Executor::new();
        executor.spawn({
            let flag = Rc::clone(&flag);
            let executor = executor.clone();
            async move {
                executor.next_frame().await;
                executor.delay(Duration::from_millis(16)).await;
                executor.next_frame().await;
                select! {
                    () = executor.next_frame().fuse() => (),
                    () = executor.delay(Duration::from_millis(64)).fuse() => (),
                }
                *flag.borrow_mut() = true;
            }
        });

        let result = executor.step(Duration::from_secs_f64(0.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 20.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 20.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 20.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = executor.step(Duration::from_secs_f64(1.0 / 20.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }
}
