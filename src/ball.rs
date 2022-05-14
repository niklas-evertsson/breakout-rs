use bevy::{
    math::{const_vec2, const_vec3, Vec2, Vec3},
    prelude::Color,
};

pub const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
pub const BALL_SPEED: f32 = 400.0;
pub const BALL_START_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);
pub const BALL_START_POSITION: Vec3 = const_vec3!([0.0, -50.0, 1.0]);
pub const BALL_COLLISION_SOUND: &str = "sounds/collision.ogg";
