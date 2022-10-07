async fn spawn_server() -> tokio::process::Child {
    let path = env!("CARGO_BIN_EXE_esc_server");
    tokio::process::Command::new(path)
        .kill_on_drop(true)
        .spawn()
        .unwrap()
}

async fn connect() -> tokio::net::TcpStream {
    let stream = loop {
        if let Ok(stream) = tokio::net::TcpStream::connect("localhost:1234").await {
            break stream;
        }
    };

    stream
}

async fn spawn_server_and_connect() -> (tokio::process::Child, tokio::net::TcpStream) {
    let process = spawn_server().await;
    let stream = connect().await;
    (process, stream)
}

#[tokio::test]
async fn basic_test() {
    let (_process, mut stream) = spawn_server_and_connect().await;

    esc_common::send(&mut stream, esc_common::Protocol::Ping).await;

    let pong = esc_common::receive(&mut stream).await;
    assert!(matches!(pong, Ok(esc_common::Protocol::Pong)));
}

#[tokio::test]
async fn can_connect_multiple_times() {
    let _process = spawn_server().await;

    // One by one.
    {
        let _stream = connect().await;
    }

    {
        let _stream = connect().await;
    }
}
