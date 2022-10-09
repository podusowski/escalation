use std::net::{Ipv4Addr, SocketAddrV4};

use esc_common::Message;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 1234);
    let listener = TcpListener::bind(addr).await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Make sure we print the port on stderr because tests are expecting it.
    eprintln!("listening on port: {}", addr.port());
    log::info!("Listening on port: {}.", addr.port());

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
