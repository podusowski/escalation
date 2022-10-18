use bevy::prelude::*;
use esc_common::send;
use tokio::{net::TcpSocket, runtime::Runtime};

pub fn networking(runtime: Res<Runtime>) {
    runtime.spawn(async {
        let socket = TcpSocket::new_v4().unwrap();
        let mut stream = socket
            .connect("127.0.0.1:1234".parse().unwrap())
            .await
            .unwrap();

        info!("Connected to server.");

        esc_common::send(&mut stream, esc_common::Message::Ping).await;
        let _ = esc_common::receive(&mut stream).await.unwrap();
        info!("Got pong.");

        send(
            &mut stream,
            esc_common::Message::Login {
                login: "login1".to_owned(),
                password: "password1".to_owned(),
            },
        )
        .await;
    });
}
