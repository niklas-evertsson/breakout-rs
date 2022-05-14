use bevy::{
    input::Input,
    math::{const_vec3, Vec3},
    prelude::{Color, KeyCode, Query, Res, Transform, With},
};

use crate::{components::Paddle, walls::*, TIME_STEP};

pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const PADDLE_HEIGHT: f32 = 60.0;
pub const PADDLE_SIZE: Vec3 = const_vec3!([120.0, 20.0, 0.0]);

const PADDLE_PADDING: f32 = 10.0;
const PADDLE_SPEED: f32 = 500.0;

pub fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;

    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}
