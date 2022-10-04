use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin};

fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //commands.spawn().insert_bundle(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Icosphere {
    //        radius: 30.0,
    //        subdivisions: 20,
    //    })),
    //    material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
    //    ..default()
    //});

    // Ship.
    commands.spawn().insert_bundle(PbrBundle {
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
    });
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

fn ui(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_startup_system(spawn_entities)
        .add_startup_system(spawn_lights_and_camera)
        .add_system(ui)
        .run();
}
