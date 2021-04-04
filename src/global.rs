use crate::AsyncRayonHandle;
use std::panic::{catch_unwind, AssertUnwindSafe};
use tokio::sync::oneshot;

/// Asynchronous wrapper around Rayon's [`spawn`](rayon::spawn).
///
/// Runs a function on the global Rayon thread pool with LIFO priority,
/// produciing a future that resolves with the function's return value.
///
/// # Panics
/// If the task function panics, the panic will be propagated through the
/// returned future. This will NOT trigger the Rayon thread pool's panic
/// handler.
///
/// If the returned handle is dropped, and the return value of `func` panics
/// when dropped, that panic WILL trigger the thread pool's panic
/// handler.
pub fn spawn<F, R>(func: F) -> AsyncRayonHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    rayon::spawn(move || {
        let _result = tx.send(catch_unwind(AssertUnwindSafe(func)));
    });

    AsyncRayonHandle { rx }
}

/// Asynchronous wrapper around Rayon's [`spawn_fifo`](rayon::spawn_fifo).
///
/// Runs a function on the global Rayon thread pool with FIFO priority,
/// produciing a future that resolves with the function's return value.
///
/// # Panics
/// If the task function panics, the panic will be propagated through the
/// returned future. This will NOT trigger the Rayon thread pool's panic
/// handler.
///
/// If the returned handle is dropped, and the return value of `func` panics
/// when dropped, then that panic WILL trigger the thread pool's panic
/// handler.
pub fn spawn_fifo<F, R>(func: F) -> AsyncRayonHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    rayon::spawn_fifo(move || {
        let _result = tx.send(catch_unwind(AssertUnwindSafe(func)));
    });

    AsyncRayonHandle { rx }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::init;

    #[tokio::test]
    async fn test_spawn_async_works() {
        init();
        let result = spawn(|| {
            let thread_index = rayon::current_thread_index();
            assert_eq!(thread_index, Some(0));
            1337_usize
        })
        .await;
        assert_eq!(result, 1337);
        let thread_index = rayon::current_thread_index();
        assert_eq!(thread_index, None);
    }

    #[tokio::test]
    async fn test_spawn_fifo_async_works() {
        init();
        let result = spawn_fifo(|| {
            let thread_index = rayon::current_thread_index();
            assert_eq!(thread_index, Some(0));
            1337_usize
        })
        .await;
        assert_eq!(result, 1337);
        let thread_index = rayon::current_thread_index();
        assert_eq!(thread_index, None);
    }

    #[tokio::test]
    #[should_panic(expected = "Task failed successfully")]
    async fn test_spawn_propagates_panic() {
        init();
        let handle = spawn(|| {
            panic!("Task failed successfully");
        });

        handle.await;
    }

    #[tokio::test]
    #[should_panic(expected = "Task failed successfully")]
    async fn test_spawn_fifo_propagates_panic() {
        init();
        let handle = spawn_fifo(|| {
            panic!("Task failed successfully");
        });

        handle.await;
    }
}
