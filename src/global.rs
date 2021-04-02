use crate::Handle;
use tokio::sync::oneshot;

/// Asynchronous wrapper around Rayon's [`spawn`](rayon::spawn).
///
/// Runs a function on the global Rayon thread pool with LIFO priority,
/// produciing a future that resolves with the function's return value.
///
/// # Errors
/// Forwards Tokio's [`RecvError`](tokio::sync::oneshot::error::RecvError), i.e. if the channel is closed.
pub fn spawn_async<F, R>(func: F) -> Handle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    rayon::spawn(move || {
        let _ = tx.send(func());
    });

    Handle { rx }
}

/// Asynchronous wrapper around Rayon's [`spawn_fifo`](rayon::spawn_fifo).
///
/// Runs a function on the global Rayon thread pool with FIFO priority,
/// produciing a future that resolves with the function's return value.
///
/// # Errors
/// Forwards Tokio's [`RecvError`](tokio::sync::oneshot::error::RecvError), i.e. if the channel is closed.
pub fn spawn_fifo_async<F, R>(func: F) -> Handle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    rayon::spawn_fifo(move || {
        let _ = tx.send(func());
    });

    Handle { rx }
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
        })
        .await;
        assert_eq!(result, Ok(()));
        let thread_index = rayon::current_thread_index();
        assert_eq!(thread_index, None);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_spawn_fifo_async_works() {
        init();
        let result = spawn_fifo_async(|| {
            let thread_index = rayon::current_thread_index();
            assert_eq!(thread_index, Some(0));
        })
        .await;
        assert_eq!(result, Ok(()));
        let thread_index = rayon::current_thread_index();
        assert_eq!(thread_index, None);
    }
}
