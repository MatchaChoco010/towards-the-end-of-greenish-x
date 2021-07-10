use crate::executor::*;
use std::cell::RefCell;
use std::future::Future;
use std::thread_local;
use std::time::Duration;

thread_local! {
    static THREAD_LOCAL_EXECUTOR: RefCell<Option<Executor>> = RefCell::new(None);
}

pub fn activate_thread_local_executor() {
    THREAD_LOCAL_EXECUTOR.with(|executor| {
        *executor.borrow_mut() = Some(Executor::new());
    });
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    THREAD_LOCAL_EXECUTOR.with(|executor| {
        executor
            .borrow()
            .as_ref()
            .expect("No Executor has been registered in this thread yet.")
            .spawn(future)
    })
}

pub fn step(delta_time: Duration) -> StepState {
    THREAD_LOCAL_EXECUTOR.with(|executor| {
        executor
            .borrow()
            .as_ref()
            .expect("No Executor has been registered in this thread yet.")
            .step(delta_time)
    })
}

pub fn next_frame() -> WaitNextFrameFuture {
    THREAD_LOCAL_EXECUTOR.with(|executor| {
        executor
            .borrow()
            .as_ref()
            .expect("No Executor has been registered in this thread yet.")
            .next_frame()
    })
}

pub fn delay(delay: Duration) -> DelayFuture {
    THREAD_LOCAL_EXECUTOR.with(|executor| {
        executor
            .borrow()
            .as_ref()
            .expect("No Executor has been registered in this thread yet.")
            .delay(delay)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::FutureExt;
    use futures::{join, select};
    use std::rc::Rc;

    #[test]
    #[should_panic]
    fn it_should_panic_if_call_step_before_activating_executor() {
        step(Duration::from_secs_f64(1.0 / 60.0));
    }

    #[test]
    #[should_panic]
    fn it_should_panic_if_call_spawn_before_activating_executor() {
        spawn(async { 1 + 2 });
    }

    #[test]
    fn it_should_run_async_block_in_single_step() {
        activate_thread_local_executor();
        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                *flag.borrow_mut() = true;
            }
        });
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_run_nested_async_block_and_await_in_single_step() {
        activate_thread_local_executor();
        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                let result = async { 1 + async { 2 + 3 }.await }.await;
                assert_eq!(result, 6);
                *flag.borrow_mut() = true;
            }
        });
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_run_multiple_await_in_single_step() {
        activate_thread_local_executor();
        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                let result = async { 1 + 2 }.await;
                assert_eq!(result, 3);
                let result = async { 3 + 4 }.await;
                assert_eq!(result, 7);
                *flag.borrow_mut() = true;
            }
        });
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_run_nested_spawn_in_single_step() {
        activate_thread_local_executor();
        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                let handle = spawn(async { 1 + 2 });
                let result = handle.await;
                assert_eq!(result, 3);
                *flag.borrow_mut() = true;
            }
        });
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_finish_in_2_step_when_1_next_frame_await() {
        activate_thread_local_executor();
        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                next_frame().await;
                *flag.borrow_mut() = true;
            }
        });
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn it_should_finish_in_11_step_when_10_next_frame_await() {
        activate_thread_local_executor();
        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                for _ in 0..10 {
                    next_frame().await;
                }
                *flag.borrow_mut() = true;
            }
        });

        for _ in 0..10 {
            let result = step(Duration::from_secs_f64(1.0 / 60.0));
            assert_eq!(&*flag.borrow(), &false);
            assert_eq!(result, StepState::RemainTasks);
        }
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn select_macro_should_work_as_expected() {
        activate_thread_local_executor();
        async fn async_fn_1() {
            next_frame().await;
        }
        async fn async_fn_2() {
            next_frame().await;
            next_frame().await;
        }

        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                select! {
                    () = async_fn_1().fuse() => (),
                    () = async_fn_2().fuse() => (),
                };
                *flag.borrow_mut() = true;
            }
        });

        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn join_macro_should_work_as_expected() {
        activate_thread_local_executor();
        async fn async_fn_1() {
            next_frame().await;
        }
        async fn async_fn_2() {
            next_frame().await;
            next_frame().await;
        }

        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                join!(async_fn_1(), async_fn_2(),);
                *flag.borrow_mut() = true;
            }
        });

        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn delay_future_should_wake_after_delay_time() {
        activate_thread_local_executor();
        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                delay(Duration::from_millis(32)).await;
                *flag.borrow_mut() = true;
            }
        });

        let result = step(Duration::from_secs_f64(0.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn multi_delay_future_should_wake_after_delay_time() {
        activate_thread_local_executor();
        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                delay(Duration::from_millis(16)).await;
                delay(Duration::from_millis(16)).await;
                select! {
                    () = delay(Duration::from_millis(16)).fuse() => (),
                    () = delay(Duration::from_millis(32)).fuse() => (),
                }
                *flag.borrow_mut() = true;
            }
        });

        let result = step(Duration::from_secs_f64(0.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 60.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }

    #[test]
    fn delay_and_next_frame_await() {
        activate_thread_local_executor();
        let flag = Rc::new(RefCell::new(false));
        spawn({
            let flag = Rc::clone(&flag);
            async move {
                next_frame().await;
                delay(Duration::from_millis(16)).await;
                next_frame().await;
                select! {
                    () = next_frame().fuse() => (),
                    () = delay(Duration::from_millis(64)).fuse() => (),
                }
                *flag.borrow_mut() = true;
            }
        });

        let result = step(Duration::from_secs_f64(0.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 20.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 20.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 20.0));
        assert_eq!(&*flag.borrow(), &false);
        assert_eq!(result, StepState::RemainTasks);
        let result = step(Duration::from_secs_f64(1.0 / 20.0));
        assert_eq!(&*flag.borrow(), &true);
        assert_eq!(result, StepState::Completed);
    }
}
