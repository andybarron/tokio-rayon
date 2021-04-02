use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::oneshot::Receiver;

#[pin_project]
pub struct Handle<T> {
    #[pin]
    pub(crate) rx: Receiver<T>,
}

impl<T> Future for Handle<T> {
    type Output = Result<T, RecvError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().rx.poll(cx)
    }
}
