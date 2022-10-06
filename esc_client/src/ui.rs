use crate::{movement::Movement, Ship};
use bevy::prelude::*;
use bevy_egui::EguiContext;
use std::{str::FromStr, time::Instant};
use thiserror::Error;

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

#[derive(Debug, Error, PartialEq)]
enum ParseCommandError {
    #[error("unknown command: '{0}'")]
    Unknown(String),
    #[error("invalid argument for '{0}': '{1}'")]
    InvalidArgument(String, String),
}

#[derive(Debug, PartialEq)]
struct Fly {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Fly {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_whitespace().collect::<Vec<&str>>();
        match tokens[0] {
            "fly" => Ok(Fly {
                x: tokens[1].parse().map_err(|_| {
                    ParseCommandError::InvalidArgument(tokens[0].to_owned(), tokens[1].to_owned())
                })?,
                y: tokens[2].parse().map_err(|_| {
                    ParseCommandError::InvalidArgument(tokens[0].to_owned(), tokens[2].to_owned())
                })?,
                z: tokens[3].parse().map_err(|_| {
                    ParseCommandError::InvalidArgument(tokens[0].to_owned(), tokens[3].to_owned())
                })?,
            }),
            _ => Err(ParseCommandError::Unknown(tokens[0].to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fly_command() {
        let event = Fly::from_str("fly 1 2 3").unwrap();
        assert_eq!(1, event.x);
        assert_eq!(2, event.y);
        assert_eq!(3, event.z);
    }

    #[test]
    fn parse_fly_command_with_errors() {
        assert_eq!(
            Err(ParseCommandError::Unknown("not_a_command".to_owned())),
            Fly::from_str("not_a_command")
        );
        assert_eq!(
            Err(ParseCommandError::InvalidArgument(
                "fly".to_owned(),
                "a".to_owned()
            )),
            Fly::from_str("fly a b c")
        );
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
            match Fly::from_str(&command) {
                Ok(Fly { x, y, z }) => {
                    for (ship, transform) in ships.iter() {
                        console.content.push(format!("{:?} is moving", ship));
                        commands.entity(ship).insert(Movement {
                            start_point: transform.translation,
                            when_started: Instant::now(),
                            destination: Vec3::new(x as f32, y as f32, z as f32),
                        });
                    }
                }
                Err(err) => {
                    console.content.push(err.to_string());
                }
            }
        }
    });
}
