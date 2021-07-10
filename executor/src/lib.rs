mod executor;
pub use crate::executor::{Executor, JoinHandle, StepState};

#[cfg(feature = "thread-local-executor")]
mod thread_local;
#[cfg(feature = "thread-local-executor")]
pub use crate::thread_local::{activate_thread_local_executor, delay, next_frame, spawn, step};
