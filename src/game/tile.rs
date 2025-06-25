use bevy::{color::palettes::css::BLUE, math::bounding::Aabb2d, prelude::*};

use crate::game::common::{components::*, constants::*};

#[derive(Component)]
pub struct Tile;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_tiles);
}

fn spawn_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let x_start = LEFT_WALL_X + (BLOCK_THICKNESS / 2.) + (TILE_WIDTH / 2.);
    let mut x_pos = x_start.clone();
    let mut y_pos = 345.;
    for _i in 0..TILES_PER_COLUMN {
        for _j in 0..TILES_PER_ROW {
            commands.spawn((
                Transform::from_xyz(x_pos, y_pos, 0.),
                Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
                    x: TILE_WIDTH - TILE_GAP,
                    y: BLOCK_THICKNESS,
                }))),
                MeshMaterial2d(materials.add(Color::from(BLUE))),
                Tile,
                Static,
                AABB(Aabb2d::new(
                    Vec2::new(x_pos, y_pos),
                    Vec2::new((TILE_WIDTH - TILE_GAP) / 2., BLOCK_THICKNESS / 2.),
                )),
            ));
            x_pos += TILE_WIDTH;
        }
        y_pos -= BLOCK_THICKNESS + TILE_GAP;
        x_pos = x_start.clone();
    }
}
