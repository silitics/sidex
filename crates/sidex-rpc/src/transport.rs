//! A bi-directional [`Transport`] trait for RPC messages.
//!
//! The [`Transport`] trait is automatically implemented for all eligible types.

use futures::{channel::mpsc, Sink, Stream};
use pin_project::pin_project;

use crate::protocol::Message;

pub trait Transport<X>
where
    Self: Stream<Item = Message<X>>,
    Self: Sink<Message<X>>,
{
}

impl<T, X> Transport<X> for T
where
    T: Stream<Item = Message<X>>,
    T: Sink<Message<X>>,
{
}

#[pin_project]
pub struct Channel<X> {
    #[pin]
    sender: mpsc::Sender<Message<X>>,
    #[pin]
    receiver: mpsc::Receiver<Message<X>>,
}

impl<X> Stream for Channel<X> {
    type Item = Message<X>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        this.receiver.poll_next(cx)
    }
}
