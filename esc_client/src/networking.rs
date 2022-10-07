use bevy::prelude::*;
use tokio::{net::TcpSocket, runtime::Runtime};

pub fn networking(runtime: Res<Runtime>) {
    runtime.spawn(async {
        let socket = TcpSocket::new_v4().unwrap();
        socket
            .connect("127.0.0.1:1234".parse().unwrap())
            .await
            .unwrap();
    });
}
