use std::net::{Ipv4Addr, SocketAddrV4};

use clap::Parser;
use esc_common::Message;
use tokio::net::TcpListener;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 1234)]
    port: u16,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, args.port);
    let listener = TcpListener::bind(addr).await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Make sure we print the port on stderr because tests are expecting it.
    println!("listening on port: {}", addr.port());
    log::info!("Listening on port: {}.", addr.port());

    loop {
        let (mut client, addr) = listener.accept().await.unwrap();
        log::info!("Connection from '{}' established.", addr);

        tokio::spawn(async move {
            let message = esc_common::receive(&mut client).await;

            match message {
                Ok(esc_common::Message::Ping) => {
                    esc_common::send(&mut client, Message::Pong).await;
                }
                Ok(esc_common::Message::Login {
                    login: _,
                    password: _,
                }) => {
                    esc_common::send(&mut client, Message::LoggedIn { id: 1 }).await;
                }
                _ => {
                    log::warn!("Unknown message: {:?}", message);
                }
            }
        });
    }
}
