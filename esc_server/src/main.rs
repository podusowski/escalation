use esc_common::Protocol;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = TcpListener::bind("localhost:1234").await.unwrap();
    log::info!("Listening on port 1234.");

    loop {
        let (mut client, addr) = listener.accept().await.unwrap();
        log::info!("Connection from '{}' established.", addr);
        let message = esc_common::receive(&mut client).await;

        match message {
            Ok(esc_common::Protocol::Ping) => {
                esc_common::send(&mut client, Protocol::Pong).await;
            }
            _ => {
                log::warn!("Unknown message: {:?}", message);
            }
        }
    }
}
