mod controls;
mod movement;
mod networking;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use controls::SelectedShip;
use movement::entities_movement;
use ui::ConsolePlugin;

fn spawn_lights_and_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 500.0).looking_at(Vec3::ZERO, Vec3::X),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}

/// Marker for the ships, that is entities which can fly somewhere.
#[derive(Component)]
pub struct Ship;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // GUI
        .add_plugin(EguiPlugin)
        .add_plugin(ConsolePlugin)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        // Networking
        .insert_resource(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
        .add_startup_system(networking::networking)
        .add_system(networking::incoming_packets)
        // Game logic
        .add_system(entities_movement)
        .add_system(controls::mouse_clicks)
        .insert_resource::<Option<SelectedShip>>(None)
        .add_system(controls::controls_ui)
        // Others
        .add_startup_system(spawn_lights_and_camera)
        .run();
}
