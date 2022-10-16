use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn send(stream: &mut TcpStream, message: Message) {
    log::trace!("Sending {:?}", message);
    let message = Envelope { message };
    let buf = bson::to_vec(&message).unwrap();
    stream.write_u32(buf.len() as u32).await.unwrap();
    stream.write_all(&buf).await.unwrap();
}

pub async fn receive(stream: &mut TcpStream) -> std::io::Result<Message> {
    // TODO: Handle ridiculous (too big) sizes.
    let size = stream.read_u32().await? as usize;
    let mut buf = [0; 1024];
    stream.read_exact(&mut buf[0..size]).await.unwrap();
    let envelop: Envelope = bson::from_reader(&buf[..]).unwrap();
    log::trace!("Received {:?}", envelop.message);
    Ok(envelop.message)
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Message {
    Ping,
    Pong,
    Login { login: String, password: String },
    LoggedIn { id: usize },
}

/// `bson` crate can't serialize `enum` directly as it doesn't appear as
/// a "document" to it. This wrapper fixes it.
#[derive(Deserialize, Serialize, Debug)]
struct Envelope {
    pub message: Message,
}
