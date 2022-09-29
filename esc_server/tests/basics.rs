use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn basic_test() {
    let path = env!("CARGO_BIN_EXE_esc_server");
    let _server = tokio::process::Command::new(path)
        .kill_on_drop(true)
        .spawn()
        .unwrap();

    let mut stream = loop {
        if let Ok(stream) = tokio::net::TcpStream::connect("localhost:1234").await {
            break stream;
        }
    };

    // Send a Ping.
    let ping = esc_common::Message {
        value: esc_common::Protocol::Ping,
    };
    let buf = bson::to_vec(&ping).unwrap();
    stream.write_u32(buf.len() as u32).await.unwrap();
    stream.write_all(&buf).await.unwrap();

    // Receive a Pong.
    //let size = stream.read_u32().await.unwrap();
    //let mut buf = vec![0u8; size as usize];
    //stream.read_exact(&mut buf).await.unwrap();

    eprintln!("{}", path);
}
