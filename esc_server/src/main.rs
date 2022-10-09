use esc_common::Message;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = TcpListener::bind("localhost:1234").await.unwrap();

    // This line is parser by the tests.
    eprintln!("listening on port: 1234");

    log::info!("Listening on port 1234.");

    loop {
        let (mut client, addr) = listener.accept().await.unwrap();
        log::info!("Connection from '{}' established.", addr);
        let message = esc_common::receive(&mut client).await;

        match message {
            Ok(esc_common::Message::Ping) => {
                esc_common::send(&mut client, Message::Pong).await;
            }
            _ => {
                log::warn!("Unknown message: {:?}", message);
            }
        }
    }
}
