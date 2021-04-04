use std::future::Future;
use std::panic::resume_unwind;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use tokio::sync::oneshot::Receiver;

/// Async handle for a blocking task running in a Rayon thread pool.
///
/// If the spawned task panics, `poll()` will propagate the panic.
#[must_use]
#[derive(Debug)]
pub struct AsyncRayonHandle<T> {
    pub(crate) rx: Receiver<thread::Result<T>>,
}

impl<T> Future for AsyncRayonHandle<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let rx = Pin::new(&mut self.rx);
        rx.poll(cx).map(|result| {
            result
                .expect("Unreachable error: Tokio channel closed")
                .unwrap_or_else(|err| resume_unwind(err))
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

    #[tokio::test]
    #[should_panic(expected = "Task failed successfully")]
    async fn test_poll_propagates_panic() {
        init();
        let panic_err = catch_unwind(|| {
            panic!("Task failed successfully");
        })
        .unwrap_err();

        let (tx, rx) = channel::<thread::Result<()>>();
        let handle = AsyncRayonHandle { rx };
        tx.send(Err(panic_err)).unwrap();
        handle.await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unreachable error: Tokio channel closed")]
    async fn test_unreachable_channel_closed() {
        init();
        let (_, rx) = channel::<thread::Result<()>>();
        let handle = AsyncRayonHandle { rx };
        handle.await;
    }
}
