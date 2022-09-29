use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, io::AsyncWriteExt};

pub async fn send(client: &mut TcpStream, payload: Protocol) {
    log::trace!("Sending {:?}", payload);
    let message = Message { value: payload };
    let buf = bson::to_vec(&message).unwrap();
    client.write_u32(buf.len() as u32).await.unwrap();
    client.write_all(&buf).await.unwrap();
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Protocol {
    Ping,
    Pong,
}

/// `bson` crate can't serialize `enum` directly as it doesn't appear as
/// a "document" to it. This wrapper fixes it.
#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub value: Protocol,
}
