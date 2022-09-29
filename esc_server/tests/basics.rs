async fn spawn_server() -> (tokio::process::Child, tokio::net::TcpStream) {
    let path = env!("CARGO_BIN_EXE_esc_server");
    let process = tokio::process::Command::new(path)
        .kill_on_drop(true)
        .spawn()
        .unwrap();

    let stream = loop {
        if let Ok(stream) = tokio::net::TcpStream::connect("localhost:1234").await {
            break stream;
        }
    };

    (process, stream)
}

#[tokio::test]
async fn basic_test() {
    let (_, mut stream) = spawn_server().await;

    esc_common::send(&mut stream, esc_common::Protocol::Ping).await;

    let pong = esc_common::receive(&mut stream).await;
    assert!(matches!(pong, esc_common::Protocol::Pong));
}
