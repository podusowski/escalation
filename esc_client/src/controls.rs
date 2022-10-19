use std::time::Instant;

use bevy::prelude::*;

use crate::{movement::Movement, Ship};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        transform: Transform::default().looking_at(
            Vec3 {
                x: 0.,
                y: 1.,
                z: 1.,
            },
            Vec3::Y,
        ),
        ..default()
    });
}

pub fn mouse_clicks(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    ships: Query<(Entity, &Transform), With<Ship>>,
) {
    let window = windows.get_primary().unwrap();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let position = (
                -(window.width() / 2. - position.x),
                -(window.height() / 2. - position.y),
            );
            info!("fly {:?}", position);

            for (ship, transform) in ships.iter() {
                commands.entity(ship).insert(Movement {
                    start_point: transform.translation,
                    when_started: Instant::now(),
                    destination: Vec3::new(position.1, -position.0, 0.),
                });
            }
        }
    }
}
