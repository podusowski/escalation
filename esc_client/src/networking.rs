use bevy::prelude::*;
use esc_common::{receive, send, Message};
use tokio::{net::TcpSocket, runtime::Runtime};

use crate::{controls::SelectedShip, Ship};

/// Process incoming network messages and apply them into the game's logic.
pub fn incoming_packets(
    mut commands: Commands,
    mut receiver: ResMut<tokio::sync::mpsc::Receiver<Message>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut selected_ship: ResMut<Option<SelectedShip>>,
) {
    while let Ok(message) = receiver.try_recv() {
        info!("Received '{:?}' in Bevy system.", message);
        match message {
            Message::Ships(ships) => {
                info!("Received list of the ships: {:?}.", ships);

                for ship in ships {
                    let entity = commands
                        .spawn()
                        .insert_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box {
                                min_x: 0.,
                                max_x: 50.,
                                min_y: 0.,
                                max_y: 10.,
                                min_z: 0.,
                                max_z: 20.,
                            })),
                            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                            transform: Transform::from_translation(ship.position),
                            ..default()
                        })
                        .insert(Ship)
                        .id();

                    *selected_ship = Some(SelectedShip { entity });
                }
            }
            _ => {
                warn!("Unknown message received: {:?}.", message);
            }
        }
    }
}

/// Set up the networking and provide `mpsc` channels for other systems.
pub fn networking(mut commands: Commands, runtime: Res<Runtime>) {
    // Channel for incoming messages. Sender is passed to Bevy as a resource
    // for systems to use.
    let (sender, receiver) = tokio::sync::mpsc::channel::<Message>(10);
    commands.insert_resource(receiver);

    runtime.spawn(async move {
        let socket = TcpSocket::new_v4().unwrap();
        let mut stream = socket
            .connect("127.0.0.1:1234".parse().unwrap())
            .await
            .unwrap();

        info!("Connected to server.");

        esc_common::send(&mut stream, esc_common::Message::Ping).await;
        let _ = esc_common::receive(&mut stream).await.unwrap();
        info!("Server is responding to ping.");

        send(
            &mut stream,
            esc_common::Message::Login {
                login: "login1".to_owned(),
                password: "password1".to_owned(),
            },
        )
        .await;

        // TODO: Move that logic to Bevy's system. Otherwise we won't be able
        // to interact with the engine.

        // TODO: Figure out how to handle errors in this context.
        let logged_in = receive(&mut stream).await;
        assert!(matches!(logged_in, Ok(Message::LoggedIn { .. })));

        loop {
            let incoming = receive(&mut stream).await;
            sender.send(incoming.unwrap()).await.unwrap();
        }
    });
}
