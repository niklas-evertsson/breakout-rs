use bevy::{
    math::Vec2,
    prelude::{Component, Deref, DerefMut},
};

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
