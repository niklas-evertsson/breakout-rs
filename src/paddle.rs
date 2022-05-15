use bevy::{
    math::{const_vec3, Vec3},
    prelude::{info, EventReader, Query, Res, Transform, With},
    window::{CursorMoved, Windows},
};

use crate::{components::Paddle, walls::*};

pub const PADDLE_HEIGHT: f32 = 60.0;
pub const PADDLE_SIZE: Vec3 = const_vec3!([120.0, 20.0, 0.0]);

const PADDLE_PADDING: f32 = 10.0;

pub fn move_paddle(
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    if cursor_moved_events.is_empty() {
        return;
    }

    let window = windows.primary();
    let window_half_with = window.width() / 2.0;

    let mut new_position: f32 = 0.0;
    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
        new_position = event.position.x - window_half_with;
    }

    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    let mut paddle_transform = query.single_mut();
    paddle_transform.translation.x = new_position.clamp(left_bound, right_bound);
}
