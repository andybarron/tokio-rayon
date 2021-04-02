use crate::Handle;
use rayon::ThreadPool;
use tokio::sync::oneshot;

/// Extension trait that integrates Rayon's [`ThreadPool`](rayon::ThreadPool)
/// with Tokio.
pub trait AsyncThreadPool {
    /// Asynchronous wrapper around Rayon's
    /// [`ThreadPool::spawn`](rayon::ThreadPool::spawn).
    ///
    /// Runs a function on the global Rayon thread pool with LIFO priority,
    /// produciing a future that resolves with the function's return value.
    ///
    /// # Errors
    /// Forwards Tokio's [`RecvError`](tokio::sync::oneshot::error::RecvError), i.e. if the channel is closed.
    fn spawn_async<F, R>(&self, func: F) -> Handle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;

    /// Asynchronous wrapper around Rayon's
    /// [`ThreadPool::spawn_fifo`](rayon::ThreadPool::spawn_fifo).
    ///
    /// Runs a function on the global Rayon thread pool with FIFO priority,
    /// produciing a future that resolves with the function's return value.
    ///
    /// # Errors
    /// Forwards Tokio's [`RecvError`](tokio::sync::oneshot::error::RecvError), i.e. if the channel is closed.
    fn spawn_fifo_async<F, R>(&self, f: F) -> Handle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;
}

impl AsyncThreadPool for ThreadPool {
    fn spawn_async<F, R>(&self, func: F) -> Handle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        self.spawn(move || {
            let _ = tx.send(func());
        });

        Handle { rx }
    }

    fn spawn_fifo_async<F, R>(&self, func: F) -> Handle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        self.spawn_fifo(move || {
            let _ = tx.send(func());
        });

        Handle { rx }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::{ThreadPool, ThreadPoolBuilder};

    fn build_thread_pool() -> ThreadPool {
        ThreadPoolBuilder::new().num_threads(1).build().unwrap()
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_spawn_async_works() {
        let pool = build_thread_pool();
        let result = pool
            .spawn_async(|| {
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
        let pool = build_thread_pool();
        let result = pool
            .spawn_fifo_async(|| {
                let thread_index = rayon::current_thread_index();
                assert_eq!(thread_index, Some(0));
            })
            .await;
        assert_eq!(result, Ok(()));
        let thread_index = rayon::current_thread_index();
        assert_eq!(thread_index, None);
    }
}
