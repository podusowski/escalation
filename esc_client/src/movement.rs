use bevy::prelude::*;
use std::time::{Duration, Instant};

/// Updates positions of entities having `Movement` component.
pub fn entities_movement(mut query: Query<(&mut Transform, &Movement)>) {
    let speed = 1.;
    for (mut transform, course) in query.iter_mut() {
        let route = course.destination - course.start_point;
        let elapsed = Instant::now() - course.when_started;
        let estimated = Duration::from_secs_f32(route.length() / speed);
        let progress = elapsed.as_secs_f32() / estimated.as_secs_f32();
        transform.translation += route * progress;
    }
}

/// The place where the ship is flying to.
#[derive(Component)]
pub struct Movement {
    pub start_point: Vec3,
    pub when_started: Instant,
    pub destination: Vec3,
}
