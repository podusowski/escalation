use tokio::io::AsyncReadExt;

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
    esc_common::send(&mut stream, esc_common::Protocol::Ping).await;

    // Receive a Pong.
    let size = stream.read_u32().await.unwrap();
    let mut buf = vec![0u8; size as usize];
    stream.read_exact(&mut buf).await.unwrap();

    eprintln!("{}", path);
}
