use bevy::color::palettes::css::WHITE_SMOKE;
use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::game::common::{components::*, constants::*, system_sets::*};
use crate::game_state::{GameState, OnGameState};

#[derive(Component)]
struct PlayerPaddle;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (handle_input).in_set(InputSet));
}

pub fn setup_player_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        OnGameState(GameState::Game),
        PlayerPaddle,
        Transform::from_xyz(0., -300., 0.0),
        Velocity(Vec2 { x: 0.0, y: 0.0 }),
        Mesh2d(meshes.add(Rectangle::from_size(Vec2 {
            x: PLAYER_PADDLE_LENGTH,
            y: BLOCK_THICKNESS,
        }))),
        Dynamic,
        MeshMaterial2d(materials.add(Color::from(WHITE_SMOKE))),
        Aabb(Aabb2d::new(
            Vec2::new(0., -300.),
            Vec2::new(PLAYER_PADDLE_LENGTH / 2., BLOCK_THICKNESS / 2.),
        )),
    ));
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<PlayerPaddle>>,
) {
    let mut vel: f32 = 0.0;
    let (mut velocity_comp, transform_comp) = query.single_mut().unwrap();
    if keys.pressed(KeyCode::KeyA) {
        vel -= PLAYER_MOVE_SPEED;
    }
    if keys.pressed(KeyCode::KeyD) {
        vel += PLAYER_MOVE_SPEED
    }
    let clamped = clamp_paddle_loc(transform_comp);
    if clamped {
        velocity_comp.0.x = 0.;
    } else {
        velocity_comp.0.x = vel;
    }
}

fn clamp_paddle_loc(mut transform: Mut<'_, Transform>) -> bool {
    let mut pos = transform.translation;
    let half_paddle_length = PLAYER_PADDLE_LENGTH / 2.;
    let half_block_thickness = BLOCK_THICKNESS / 2.;
    let left_max = LEFT_WALL_X + half_block_thickness + half_paddle_length;
    if pos.x < left_max {
        pos.x = left_max;
        transform.translation = pos;
        return true;
    }
    let right_max = RIGHT_WALL_X - half_block_thickness - half_paddle_length;
    if pos.x > right_max {
        pos.x = right_max;
        transform.translation = pos;
        return true;
    }
    false
}
