#[tokio::test]
async fn basic_test() {
    let path = env!("CARGO_BIN_EXE_esc_server");
    let mut server = tokio::process::Command::new(path)
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    eprintln!("{}", path);
}
