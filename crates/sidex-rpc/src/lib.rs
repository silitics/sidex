#[doc(hidden)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}

pub use generated::*;

pub mod transport;

// pub trait Endpoint {
//     type CallFut<X>: std::future::Future<Output = protocol::ReturnMessage<X>>;

//     fn call<X, C: CallCtx>(&self, ctx: C, message: protocol::CallMessage<X>) ->
// Self::CallFut<X>     where
//         X: Send;
// }

// pub trait CallCtx {
//     fn get_stream(&self, id: protocol::StreamId);

//     fn setup_stream(&self) -> protocol::StreamId;
// }

// pub trait Serve {
//     fn serve<X, Tx, Rx>(&self, tx: Tx, rx: Rx) -> BoxFuture<'_, ()>
//     where
//         X: Send,
//         Tx: 'static + Unpin + Send + Sink<protocol::Message<X>>,
//         Rx: 'static + Unpin + Send + Stream<Item = protocol::Message<X>>;
// }

// type CallFuture<X> = BoxFuture<'static, protocol::ReturnMessage<X>>;

// struct ServeFuture<'e, X, E, Tx, Rx> {
//     endpoint: &'e E,
//     tx: Tx,
//     rx: Rx,
//     pending: FuturesUnordered<CallFuture<X>>,
//     tx_buffer: VecDeque<protocol::Message<X>>,
// }

// impl<'e, X, E, Tx, Rx> Future for ServeFuture<'e, X, E, Tx, Rx>
// where
//     X: Send,
//     E: Endpoint,
//     Tx: 'static + Unpin + Send + Sink<protocol::Message<X>>,
//     Rx: 'static + Unpin + Send + Stream<Item = protocol::Message<X>>,
// {
//     type Output = ();

//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         while !self.tx_buffer.is_empty() {
//             match self.tx.poll_ready_unpin(cx) {
//                 Poll::Ready(Ok(_)) => {
//                     let message = self.tx_buffer.pop_front().unwrap();
//                     self.tx.start_send_unpin(message);
//                 }
//                 Poll::Ready(Err(err)) => {
//                     panic!("Propagate error!")
//                 }
//                 _ => {
//                     break;
//                 }
//             }
//         }

//         Poll::Pending
//     }
// }

// impl<E: Endpoint> Serve for E {
//     fn serve<X, Tx, Rx>(&self, mut tx: Tx, mut rx: Rx) -> BoxFuture<'_, ()>
//     where
//         X: Send,
//         Tx: 'static + Unpin + Send + Sink<protocol::Message<X>>,
//         Rx: 'static + Unpin + Send + Stream<Item = protocol::Message<X>>,
//     {
//         Box::pin(async move {

//             // let (spawn_queue_tx, mut spawn_queue_rx) =
//             // mpsc::channel::<CallFuture<X>>(4); let tx_loop = async {
//             //     let mut pending_calls = FuturesUnordered::<CallFuture<X>>::new();
//             //     loop {
//             //         futures::select! {
//             //             done = pending_calls.next() => {
//             //                 if let Some(done) = done {
//             //                     if
//             // tx.send(protocol::Message::Return(done)).await.is_err() {
//             //                         panic!("AHHHHHH...")
//             //                     }
//             //                 }
//             //             }
//             //             call = spawn_queue_rx.next() => {
//             //                 if let Some(call) = call {
//             //                     pending_calls.push(call);
//             //                 } else {
//             //                     // RX loop terminated;
//             //                     break;
//             //                 }
//             //             }
//             //         }
//             //     }
//             // };
//             // let rx_loop = async {
//             //     while let Some(message) = rx.next().await {
//             //         match message {
//             //             protocol::Message::Call(_) => todo!(),
//             //             protocol::Message::Return(_) => todo!(),
//             //             protocol::Message::Abort(_) => todo!(),
//             //             protocol::Message::StreamNext(_) => todo!(),
//             //             protocol::Message::StreamClose(_) => todo!(),
//             //         }
//             //     }
//             // };
//             // join(tx_loop, rx_loop).await;
//         })
//     }
// }
