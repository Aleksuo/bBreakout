use bevy::{
    math::bounding::{Aabb2d, BoundingCircle},
    prelude::*,
};

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct BC(pub BoundingCircle);

#[derive(Component)]
pub struct Aabb(pub Aabb2d);

#[derive(Component)]
pub struct Dynamic;

#[derive(Component)]
pub struct Static;

#[derive(Component)]
pub struct InstantDeath;
