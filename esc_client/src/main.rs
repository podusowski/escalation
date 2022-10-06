use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin};

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
        .insert(Ship)
        // FIXME: A temporary, hardcoded destination.
        .insert(Destination {
            start: Vec3::default(),
            start_time: Instant::now(),
            destination: Vec3::new(10., 0., 0.),
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

fn entities_movement(mut query: Query<(&mut Transform, &Destination)>) {
    let speed = 1.;
    for (mut transform, course) in query.iter_mut() {
        let route = course.destination - course.start;
        let elapsed = Instant::now() - course.start_time;
        let estimated = Duration::from_secs_f32(route.length() / speed);
        let progress = elapsed.as_secs_f32() / estimated.as_secs_f32();
        transform.translation += route * progress;
    }
}

/// Marker for the ships, that is entities which can fly somewhere.
#[derive(Component)]
struct Ship;

/// The place where the ship is flying to.
#[derive(Component)]
struct Destination {
    start: Vec3,
    start_time: Instant,
    destination: Vec3,
}

#[derive(Debug)]
struct Fly {
    x: i32,
    y: i32,
    z: i32,
}

/// Data storage for the console.
#[derive(Default)]
struct Console {
    content: Vec<String>,
    command: String,
}

fn process_command(console: &mut Console, command: &str) -> Option<Fly> {
    let command = command.split_whitespace().collect::<Vec<&str>>();
    match command[0] {
        "fly" => Some(Fly {
            x: command[1].parse().unwrap(),
            y: command[2].parse().unwrap(),
            z: command[3].parse().unwrap(),
        }),
        _ => {
            console
                .content
                .push(format!("unknown command: {}", command[0]));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fly_command() {
        let mut console = Console::default();
        let event = process_command(&mut console, "fly 1 2 3").unwrap();
        assert_eq!(1, event.x);
        assert_eq!(2, event.y);
        assert_eq!(3, event.z);
    }
}

/// System for drawing and managing the console.
fn console(
    mut egui_context: ResMut<EguiContext>,
    mut console: ResMut<Console>,
    mut commands: Commands,
    ships: Query<(Entity, &Transform), With<Ship>>,
) {
    egui::Window::new("Console").show(egui_context.ctx_mut(), |ui| {
        for line in &console.content {
            ui.label(line.as_str());
        }

        let response = ui.text_edit_singleline(&mut console.command);
        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
            let command = std::mem::take(&mut console.command);

            // TODO: Handling `process_command`'s result should probably be a
            // separate system.
            match dbg!(process_command(&mut console, &command)) {
                Some(Fly { x, y, z }) => {
                    for (ship, transform) in ships.iter() {
                        console.content.push(format!("{:?} is moving", ship));
                        commands.entity(ship).insert(Destination {
                            start: transform.translation,
                            start_time: Instant::now(),
                            destination: Vec3::new(x as f32, y as f32, z as f32),
                        });
                    }
                }
                None => todo!(),
            }
        }
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_startup_system(spawn_entities)
        .add_system(entities_movement)
        .add_startup_system(spawn_lights_and_camera)
        .insert_resource(Console::default())
        .add_system(console)
        .run();
}
