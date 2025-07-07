use bevy::prelude::*;

pub const PLAYER_MOVE_SPEED: f32 = 200.0;
pub const PLAYER_PADDLE_LENGTH: f32 = 50.;
pub const BLOCK_THICKNESS: f32 = 10.;
pub const LEFT_WALL_X: f32 = -400.;
pub const RIGHT_WALL_X: f32 = 400.;
pub const HORIZONTAL_WALL_LENGTH: f32 = 800.;
pub const VERTICAL_WALL_LENGTH: f32 = 700.;
pub const TILES_PER_ROW: u32 = 20;
pub const TILES_PER_COLUMN: u32 = 9;
pub const TILE_WIDTH: f32 = 39.5;
pub const TILE_GAP: f32 = 5.;
pub const BALL_RADIUS: f32 = 5.;
pub const BALL_START_VELOCITY: Vec2 = Vec2::new(0., -150.);
