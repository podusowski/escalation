use tokio::io::{AsyncWriteExt, AsyncReadExt};

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

    stream.write_u32(1).await.unwrap();
    let ping = esc_common::Protocol::Ping;
    let buf = bson::to_vec(&ping).unwrap();
    stream.write_all(&buf).await.unwrap();

    eprintln!("{}", path);
}
