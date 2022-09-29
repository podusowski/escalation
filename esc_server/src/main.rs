use esc_common::Protocol;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = TcpListener::bind("localhost:1234").await.unwrap();
    log::info!("Listening on port 1234");

    loop {
        let (mut client, _) = listener.accept().await.unwrap();
        let message = esc_common::read(&mut client).await;

        match message {
            esc_common::Protocol::Ping => {
                esc_common::send(&mut client, Protocol::Pong).await;
            }
            _ => {
                log::warn!("Unknown message: {:?}", message)
            }
        }
    }
}
