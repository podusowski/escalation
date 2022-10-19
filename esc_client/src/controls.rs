use std::time::Instant;

use bevy::prelude::*;

use crate::{movement::Movement, Ship};

/// Controls current ship's movement with mouse keys.
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
