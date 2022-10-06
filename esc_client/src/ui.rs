use crate::{movement::Movement, Ship};
use bevy::prelude::*;
use bevy_egui::EguiContext;
use std::time::Instant;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Console::default()).add_system(console);
    }
}

/// Data storage for the console.
#[derive(Default)]
struct Console {
    content: Vec<String>,
    command: String,
}

#[derive(Debug)]
struct Fly {
    x: i32,
    y: i32,
    z: i32,
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
                        commands.entity(ship).insert(Movement {
                            start_point: transform.translation,
                            when_started: Instant::now(),
                            destination: Vec3::new(x as f32, y as f32, z as f32),
                        });
                    }
                }
                None => todo!(),
            }
        }
    });
}
