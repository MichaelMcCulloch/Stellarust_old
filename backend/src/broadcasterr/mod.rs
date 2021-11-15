use actix_web::{web::Bytes, Error};
use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

mod test;

pub trait TBroadcaster: Send {
    fn new_client(&mut self) -> Client;
    fn send(&self, msg: &str);
}

pub struct Broadcasterr {
    clients: Vec<Sender<Bytes>>,
}

impl TBroadcaster for Broadcasterr {
    fn new_client(&mut self) -> Client {
        let (bytes_sender, bytes_receiver) = channel(100);

        bytes_sender
            .try_send(Bytes::from("event: connected\ndata: connected\n\n"))
            .unwrap();

        self.clients.push(bytes_sender);
        Client(bytes_receiver)
    }

    fn send(&self, msg: &str) {
        let msg = Bytes::from(["event: message\ndata: ", msg, "\n\n"].concat());

        for client in self.clients.iter() {
            client.try_send(msg.clone()).unwrap_or(());
        }
    }
}

pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
