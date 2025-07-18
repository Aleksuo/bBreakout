use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    math::bounding::Aabb2d,
    prelude::*,
};

use crate::{
    game::{
        ball::Ball,
        common::{components::*, constants::*, system_sets::GameplaySet},
        game_events::{CollisionEvent, TileDestroyedEvent},
    },
    game_state::{GameState, OnGameState},
};

#[derive(Component)]
pub struct Tile;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup_tiles)
        .add_systems(FixedUpdate, on_collision_event.in_set(GameplaySet));
}

pub fn setup_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let x_start = LEFT_WALL_X + (BLOCK_THICKNESS / 2.) + (TILE_WIDTH / 2.);
    let mut x_pos = x_start;
    let mut y_pos = 345.;
    for i in 0..TILES_PER_COLUMN {
        for _j in 0..TILES_PER_ROW {
            let mut brick_color = Color::from(BLUE);
            if i < 3 {
                brick_color = Color::from(RED);
            } else if i < 6 {
                brick_color = Color::from(GREEN);
            }
            commands.spawn((
                OnGameState(GameState::Game),
                Transform::from_xyz(x_pos, y_pos, 0.),
                Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
                    x: TILE_WIDTH - TILE_GAP,
                    y: BLOCK_THICKNESS,
                }))),
                MeshMaterial2d(materials.add(brick_color)),
                Tile,
                Static,
                Aabb(Aabb2d::new(
                    Vec2::new(x_pos, y_pos),
                    Vec2::new((TILE_WIDTH - TILE_GAP) / 2., BLOCK_THICKNESS / 2.),
                )),
            ));
            x_pos += TILE_WIDTH;
        }
        y_pos -= BLOCK_THICKNESS + TILE_GAP;
        x_pos = x_start;
    }
}

fn on_collision_event(
    mut commands: Commands,
    mut collision_reader: EventReader<CollisionEvent>,
    mut tile_destroyed_writer: EventWriter<TileDestroyedEvent>,
    tiles_q: Query<&Tile>,
    balls_q: Query<&Ball>,
) {
    for col_event in collision_reader.read() {
        if let Some(tile_entity) = maybe_collision_has_tile(col_event, tiles_q) {
            let other_entity = other_entity_in_col(&tile_entity, col_event);
            if let Some(_ball_entity) = maybe_entity_is_ball(&other_entity, balls_q) {
                commands.entity(tile_entity).despawn();
                tile_destroyed_writer.write(TileDestroyedEvent);
            }
        }
    }
}

fn other_entity_in_col(first_entity: &Entity, col_event: &CollisionEvent) -> Entity {
    if *first_entity == col_event.0 {
        return col_event.1;
    }
    col_event.0
}

fn maybe_collision_has_tile(col_event: &CollisionEvent, tiles_q: Query<&Tile>) -> Option<Entity> {
    maybe_entity_is_tile(&col_event.0, tiles_q).or(maybe_entity_is_tile(&col_event.1, tiles_q))
}

fn maybe_entity_is_tile(entity: &Entity, tiles_q: Query<&Tile>) -> Option<Entity> {
    match tiles_q.get(*entity) {
        Ok(_tile) => Some(*entity),
        Err(_e) => None,
    }
}

fn maybe_entity_is_ball(entity: &Entity, balls_q: Query<&Ball>) -> Option<Entity> {
    match balls_q.get(*entity) {
        Ok(_ball) => Some(*entity),
        Err(_e) => None,
    }
}
