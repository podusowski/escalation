use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:1234").await.unwrap();

    loop {
        let (mut client, _) = listener.accept().await.unwrap();

        // Read a "packet" from stream. Packets are implemented by sending a
        // size first and following it with rest of the data.
        let size = client.read_u32().await.unwrap() as usize;
        let mut buf = [0; 1024];
        client.read_exact(&mut buf[0..size]).await.unwrap();
        let message: esc_common::Message = bson::from_reader(&buf[..]).unwrap();
    }
}
