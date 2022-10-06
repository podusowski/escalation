use bevy::prelude::*;
use std::time::{Duration, Instant};

pub fn entities_movement(mut query: Query<(&mut Transform, &Movement)>) {
    let speed = 1.;
    for (mut transform, course) in query.iter_mut() {
        let route = course.destination - course.start;
        let elapsed = Instant::now() - course.start_time;
        let estimated = Duration::from_secs_f32(route.length() / speed);
        let progress = elapsed.as_secs_f32() / estimated.as_secs_f32();
        transform.translation += route * progress;
    }
}

/// The place where the ship is flying to.
#[derive(Component)]
pub struct Movement {
    pub start: Vec3,
    pub start_time: Instant,
    pub destination: Vec3,
}
