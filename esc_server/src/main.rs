use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = TcpListener::bind("localhost:1234").await.unwrap();
    log::info!("Listening on port 1234");

    loop {
        let (mut client, _) = listener.accept().await.unwrap();

        // Read a "packet" from stream. Packets are implemented by sending a
        // size first and following it with rest of the data.
        let size = client.read_u32().await.unwrap() as usize;
        let mut buf = [0; 1024];
        client.read_exact(&mut buf[0..size]).await.unwrap();
        let message: esc_common::Message = bson::from_reader(&buf[..]).unwrap();
        log::trace!("Received: {:?}", message);

        match message {
            esc_common::Message {
                value: esc_common::Protocol::Ping,
            } => {
                let pong = esc_common::Message {
                    value: esc_common::Protocol::Pong,
                };
                let buf = bson::to_vec(&pong).unwrap();
                client.write_u32(buf.len() as u32).await.unwrap();
                client.write_all(&buf).await.unwrap();
            }
            _ => {
                log::warn!("Unknown message: {:?}", message)
            }
        }
    }
}
