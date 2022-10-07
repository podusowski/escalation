mod movement;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use movement::entities_movement;
use ui::ConsolePlugin;

fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
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
            ..default()
        })
        .insert(Ship);
}

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
struct Ship;

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
        // Game logic
        .add_startup_system(spawn_entities)
        .add_system(entities_movement)
        // Others
        .add_startup_system(spawn_lights_and_camera)
        .run();
}
