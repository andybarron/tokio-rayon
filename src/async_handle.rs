use pin_project::pin_project;
use std::future::Future;
use std::panic::resume_unwind;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::oneshot::Receiver;

/// Async handle for a blocking task running in a Rayon thread pool.
///
/// If the spawned task panics, `poll()` will propagate the panic.
#[must_use]
#[pin_project]
#[derive(Debug)]
pub struct AsyncHandle<T> {
    #[pin]
    pub(crate) rx: Receiver<thread::Result<T>>,
}

impl<T> Future for AsyncHandle<T> {
    type Output = Result<T, RecvError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().rx.poll(cx).map_ok(|result| match result {
            Ok(data) => data,
            Err(err) => resume_unwind(err),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::init;
    use std::panic::catch_unwind;
    use std::thread;
    use tokio::sync::oneshot::channel;

    #[tokio::test(flavor = "current_thread")]
    #[should_panic(expected = "Task failed successfully")]
    async fn test_poll_propagates_panic() {
        init();
        let panic_err = catch_unwind(|| {
            panic!("Task failed successfully");
        })
        .unwrap_err();

        let (tx, rx) = channel::<thread::Result<()>>();
        let handle = AsyncHandle { rx };
        tx.send(Err(panic_err)).unwrap();
        handle.await.unwrap();
    }
}
