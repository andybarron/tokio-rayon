use crate::AsyncHandle;
use std::panic::{catch_unwind, AssertUnwindSafe};
use tokio::sync::oneshot;

/// Asynchronous wrapper around Rayon's [`spawn`](rayon::spawn).
///
/// Runs a function on the global Rayon thread pool with LIFO priority,
/// produciing a future that resolves with the function's return value.
///
/// # Panics
/// If the task function panics, the panic will be propagated through the
/// returned future. Thie will NOT trigger the Rayon thread pool's panic
/// handler.
pub fn spawn_async<F, R>(func: F) -> AsyncHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    rayon::spawn(move || {
        let _result = tx.send(catch_unwind(AssertUnwindSafe(func)));
    });

    AsyncHandle { rx }
}

/// Asynchronous wrapper around Rayon's [`spawn_fifo`](rayon::spawn_fifo).
///
/// Runs a function on the global Rayon thread pool with FIFO priority,
/// produciing a future that resolves with the function's return value.
///
/// # Panics
/// If the task function panics, the panic will be propagated through the
/// returned future. Thie will NOT trigger the Rayon thread pool's panic
/// handler.
pub fn spawn_fifo_async<F, R>(func: F) -> AsyncHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    rayon::spawn_fifo(move || {
        let _result = tx.send(catch_unwind(AssertUnwindSafe(func)));
    });

    AsyncHandle { rx }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::init;

    #[tokio::test(flavor = "current_thread")]
    async fn test_spawn_async_works() {
        init();
        let result = spawn_async(|| {
            let thread_index = rayon::current_thread_index();
            assert_eq!(thread_index, Some(0));
            1337_usize
        })
        .await;
        assert_eq!(result, 1337);
        let thread_index = rayon::current_thread_index();
        assert_eq!(thread_index, None);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_spawn_fifo_async_works() {
        init();
        let result = spawn_fifo_async(|| {
            let thread_index = rayon::current_thread_index();
            assert_eq!(thread_index, Some(0));
            1337_usize
        })
        .await;
        assert_eq!(result, 1337);
        let thread_index = rayon::current_thread_index();
        assert_eq!(thread_index, None);
    }
}
