use crate::AsyncHandle;
use rayon::ThreadPool;
use std::panic::{catch_unwind, AssertUnwindSafe};
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
    /// # Panics
    /// If the task function panics, the panic will be propagated through the
    /// returned future. Thie will NOT trigger the Rayon thread pool's panic
    /// handler.
    fn spawn_async<F, R>(&self, func: F) -> AsyncHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;

    /// Asynchronous wrapper around Rayon's
    /// [`ThreadPool::spawn_fifo`](rayon::ThreadPool::spawn_fifo).
    ///
    /// Runs a function on the global Rayon thread pool with FIFO priority,
    /// produciing a future that resolves with the function's return value.
    ///
    /// # Panics
    /// If the task function panics, the panic will be propagated through the
    /// returned future. Thie will NOT trigger the Rayon thread pool's panic
    /// handler.
    fn spawn_fifo_async<F, R>(&self, f: F) -> AsyncHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;
}

impl AsyncThreadPool for ThreadPool {
    fn spawn_async<F, R>(&self, func: F) -> AsyncHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        self.spawn(move || {
            let _result = tx.send(catch_unwind(AssertUnwindSafe(func)));
        });

        AsyncHandle { rx }
    }

    fn spawn_fifo_async<F, R>(&self, func: F) -> AsyncHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        self.spawn_fifo(move || {
            let _result = tx.send(catch_unwind(AssertUnwindSafe(func)));
        });

        AsyncHandle { rx }
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
                1337_usize
            })
            .await;
        assert_eq!(result, 1337);
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
                1337_usize
            })
            .await;
        assert_eq!(result, 1337);
        let thread_index = rayon::current_thread_index();
        assert_eq!(thread_index, None);
    }
}
