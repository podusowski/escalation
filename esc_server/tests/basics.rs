use assert_matches::assert_matches;
use esc_common::{receive, Message};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    process::Stdio,
};
use tokio::io::{AsyncBufReadExt, BufReader};

async fn spawn_server() -> (tokio::process::Child, u16) {
    let path = env!("CARGO_BIN_EXE_esc_server");
    let mut child = tokio::process::Command::new(path)
        .arg("--port")
        .arg("0")
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let port_pattern = regex::Regex::new(r"^listening on port: (\d+)$").unwrap();

    let mut stdout = BufReader::new(
        child
            .stdout
            .as_mut()
            .expect("child is missing stdout handle"),
    )
    .lines();

    // TODO: Currently there is no way of reading server's log output. One way
    //       of solving this without loosing the port is to spawn a task which
    //       continuously reads the output and flushes it on the test's log.
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

#[tokio::test]
async fn holds_multiple_active_connections() {
    let (_process, port) = spawn_server().await;

    let mut stream1 = connect(port).await;
    let mut stream2 = connect(port).await;

    esc_common::send(&mut stream2, esc_common::Message::Ping).await;
    let pong = esc_common::receive(&mut stream2).await;
    assert!(matches!(pong, Ok(esc_common::Message::Pong)));

    esc_common::send(&mut stream1, esc_common::Message::Ping).await;
    let pong = esc_common::receive(&mut stream1).await;
    assert!(matches!(pong, Ok(esc_common::Message::Pong)));
}

#[tokio::test]
async fn login() {
    let (_process, mut stream) = spawn_server_and_connect().await;

    esc_common::send(
        &mut stream,
        esc_common::Message::Login {
            login: "login1".to_owned(),
            password: "password1".to_owned(),
        },
    )
    .await;

    let logged_in = esc_common::receive(&mut stream).await;
    assert_matches!(logged_in, Ok(esc_common::Message::LoggedIn { id: _ }));

    let ships = receive(&mut stream).await;
    assert_matches!(ships, Ok(Message::Ships(ships)) if ships == [1]);
}
