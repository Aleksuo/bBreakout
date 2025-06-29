use crate::{
    game::common::{components::*, constants::*},
    game_state::{GameState, OnGameState},
};
use bevy::{
    color::palettes::css::{ORANGE, WHITE_SMOKE},
    math::bounding::Aabb2d,
    prelude::*,
};

#[derive(Component)]
struct Wall;

pub(super) fn plugin(_app: &mut App) {}

pub fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Left wall
    commands.spawn((
        OnGameState(GameState::Game),
        Wall,
        Transform::from_xyz(LEFT_WALL_X, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
            x: BLOCK_THICKNESS,
            y: WALL_LENGTH,
        }))),
        MeshMaterial2d(materials.add(Color::from(ORANGE))),
        Static,
        AABB(Aabb2d::new(
            Vec2::new(LEFT_WALL_X, 0.0),
            Vec2::new(BLOCK_THICKNESS / 2., WALL_LENGTH / 2.),
        )),
    ));
    // Right wall
    commands.spawn((
        OnGameState(GameState::Game),
        Wall,
        Transform::from_xyz(RIGHT_WALL_X, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
            x: BLOCK_THICKNESS,
            y: WALL_LENGTH,
        }))),
        MeshMaterial2d(materials.add(Color::from(ORANGE))),
        Static,
        AABB(Aabb2d::new(
            Vec2::new(RIGHT_WALL_X, 0.0),
            Vec2::new(BLOCK_THICKNESS / 2., WALL_LENGTH / 2.),
        )),
    ));
    commands.spawn((
        OnGameState(GameState::Game),
        Wall,
        Transform::from_xyz(0., 355., 0.0),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
            x: WALL_LENGTH,
            y: BLOCK_THICKNESS,
        }))),
        MeshMaterial2d(materials.add(Color::from(ORANGE))),
        Static,
        AABB(Aabb2d::new(
            Vec2::new(0., 355.),
            Vec2::new(WALL_LENGTH / 2., BLOCK_THICKNESS / 2.),
        )),
    ));
    commands.spawn((
        OnGameState(GameState::Game),
        Wall,
        Transform::from_xyz(0., -355., 0.0),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
            x: WALL_LENGTH - BLOCK_THICKNESS,
            y: BLOCK_THICKNESS,
        }))),
        MeshMaterial2d(materials.add(Color::from(WHITE_SMOKE))),
        Static,
        InstantDeath,
        AABB(Aabb2d::new(
            Vec2::new(0., -355.),
            Vec2::new(WALL_LENGTH / 2., BLOCK_THICKNESS / 2.),
        )),
    ));
}
