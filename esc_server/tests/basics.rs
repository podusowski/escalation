use std::{
    net::{Ipv4Addr, SocketAddrV4},
    process::Stdio,
};

use tokio::io::{AsyncBufReadExt, BufReader};

async fn spawn_server() -> (tokio::process::Child, u16) {
    let path = env!("CARGO_BIN_EXE_esc_server");
    let mut child = tokio::process::Command::new(path)
        .kill_on_drop(true)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let port_pattern = regex::Regex::new(r"^listening on port: (\d+)$").unwrap();

    let mut stdout = BufReader::new(
        child
            .stderr
            .as_mut()
            .expect("child is missing stderr handle"),
    )
    .lines();

    let port = loop {
        let line = stdout
            .next_line()
            .await
            .expect("error while reading output")
            .expect("server process did not print the port number");
        if let Some(captures) = port_pattern.captures(&line) {
            let port: u16 = captures.get(1).unwrap().as_str().parse().unwrap();
            break port;
        }
    };

    (child, port)
}

async fn connect(port: u16) -> tokio::net::TcpStream {
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    let stream = loop {
        if let Ok(stream) = tokio::net::TcpStream::connect(addr).await {
            break stream;
        }
    };

    stream
}

async fn spawn_server_and_connect() -> (tokio::process::Child, tokio::net::TcpStream) {
    let (process, port) = spawn_server().await;
    let stream = connect(port).await;
    (process, stream)
}

#[tokio::test]
async fn basic_test() {
    let (_process, mut stream) = spawn_server_and_connect().await;

    esc_common::send(&mut stream, esc_common::Message::Ping).await;

    let pong = esc_common::receive(&mut stream).await;
    assert!(matches!(pong, Ok(esc_common::Message::Pong)));
}

#[tokio::test]
async fn can_connect_multiple_times() {
    let (_process, port) = spawn_server().await;

    // One by one.
    {
        let _stream = connect(port).await;
    }

    {
        let _stream = connect(port).await;
    }
}
