use bevy::{color::palettes::css::WHITE_SMOKE, math::bounding::BoundingCircle, prelude::*};

use crate::game::common::{components::*, constants::*};

#[derive(Component)]
struct Ball;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ball);
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Transform::from_xyz(0., 0., 0.),
        Velocity(BALL_START_VELOCITY),
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::from(WHITE_SMOKE))),
        Ball,
        BC(BoundingCircle::new(Vec2::new(0., 0.), BALL_RADIUS)),
        Dynamic,
    ));
}
