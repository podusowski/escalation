use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn send(client: &mut TcpStream, payload: Protocol) {
    log::trace!("Sending {:?}", payload);
    let message = Message { value: payload };
    let buf = bson::to_vec(&message).unwrap();
    client.write_u32(buf.len() as u32).await.unwrap();
    client.write_all(&buf).await.unwrap();
}

pub async fn receive(stream: &mut TcpStream) -> Protocol {
    let size = stream.read_u32().await.unwrap() as usize;
    let mut buf = [0; 1024];
    stream.read_exact(&mut buf[0..size]).await.unwrap();
    let message: Message = bson::from_reader(&buf[..]).unwrap();
    log::trace!("Received {:?}", message.value);
    message.value
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Protocol {
    Ping,
    Pong,
}

/// `bson` crate can't serialize `enum` directly as it doesn't appear as
/// a "document" to it. This wrapper fixes it.
#[derive(Deserialize, Serialize, Debug)]
struct Message {
    pub value: Protocol,
}
