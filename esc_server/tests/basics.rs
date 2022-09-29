use tokio::io::AsyncWriteExt;

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

    eprintln!("{}", path);
}
