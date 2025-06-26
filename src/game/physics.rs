use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;
use bevy::render::mesh::MeshAabb;

use crate::game::common::components::*;
use crate::game::common::constants::*;
use crate::game::common::system_sets::PhysicsSet;
use crate::game::score::Score;
use crate::game::tile::Tile;

#[derive(PartialEq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (move_moving, update_colliders, handle_collisions)
            .chain()
            .in_set(PhysicsSet),
    );
}

fn move_moving(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        let change = velocity.0 * time.delta_secs();
        transform.translation += Vec3::new(change.x, change.y, 0.0);
    }
}

fn handle_collisions(
    mut commands: Commands,
    mut score_res: ResMut<Score>,
    mut bc_query: Query<(&mut BC, &mut Velocity, &mut Transform)>,
    aabb_query: Query<(Entity, &AABB, Option<&Tile>)>,
) {
    for (mut bc, mut vel, mut transform) in &mut bc_query {
        for (entity, aabb, option_tile) in aabb_query {
            if bc.0.intersects(&aabb.0) {
                let epsilon = 0.01;
                let contact = aabb.0.closest_point(bc.0.center);
                let mut delta = bc.0.center - contact;
                let mut dist_sq = delta.length_squared();
                // Center is inside the rectangle, calculate side and return the corresponging unit vector
                if dist_sq == 0. {
                    delta = match get_ball_collision_side(&bc, &aabb) {
                        Side::Left => Vec2::NEG_X,
                        Side::Right => Vec2::X,
                        Side::Top => Vec2::Y,
                        Side::Bottom => Vec2::NEG_Y,
                    };
                    dist_sq = 1.0;
                }
                let dist = dist_sq.sqrt();
                let penetration = bc.0.radius() - dist;
                let normal = delta / dist;

                // Move the ball back outside the rectangle to prevent any wierd behavior
                transform.translation += (penetration + epsilon) * normal.normalize().extend(0.0);
                bc.0.center = transform.translation.truncate();

                vel.0 = vel.0.reflect(normal);

                if option_tile.is_some() {
                    commands.entity(entity).despawn();
                    score_res.0 += 1;
                }
            }
        }
    }
}

fn get_ball_collision_side(bc: &BC, aabb: &AABB) -> Side {
    let closest = aabb.0.closest_point(bc.0.center);
    let delta = bc.0.center - closest;
    if delta.x.abs() > delta.y.abs() {
        if delta.x.is_sign_negative() {
            Side::Left
        } else {
            Side::Right
        }
    } else {
        if delta.y.is_sign_negative() {
            Side::Bottom
        } else {
            Side::Top
        }
    }
}

fn update_colliders(
    meshes: ResMut<Assets<Mesh>>,
    bc_query: Query<(&mut BC, &Transform, &Mesh2d), With<Dynamic>>,
    aabb_query: Query<(&mut AABB, &Transform, &Mesh2d), With<Dynamic>>,
) {
    for (mut bc, transform, _) in bc_query {
        let center = transform.translation.xy();
        bc.0 = BoundingCircle::new(center, BALL_RADIUS);
    }
    for (mut aabb, transform, mesh) in aabb_query {
        let computed_aabb = meshes.get(mesh.id()).unwrap().compute_aabb().unwrap();
        aabb.0 = Aabb2d::new(
            transform.translation.xy(),
            computed_aabb.half_extents.truncate(),
        );
    }
}
