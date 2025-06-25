use bevy::prelude::*;

use bevy::{
    color::palettes::css::{BLUE, ORANGE, WHITE_SMOKE},
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    render::mesh::MeshAabb,
};

mod common;
mod player;

use common::{components::*, constants::*, system_sets::*};

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Ball;

#[derive(PartialEq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Resource, DerefMut, Deref)]
struct Score(u32);

#[derive(Component)]
struct ScoreTextUI;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((DefaultPlugins, player::plugin))
        .configure_sets(FixedUpdate, (InputSet.before(TempSet)))
        .insert_resource(Score(0))
        .add_systems(Startup, (add_walls, add_tiles, add_ball, add_ui))
        .add_systems(
            FixedUpdate,
            (move_moving, update_colliders, handle_collisions)
                .chain()
                .in_set(TempSet),
        )
        .add_systems(Update, update_score_ui);
}

fn add_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Left wall

    commands.spawn((
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
}

fn add_tiles(
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

fn add_ball(
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

fn add_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        children![(
            ScoreTextUI,
            Text::new("Score: 0"),
            TextShadow::default(),
            Node {
                position_type: PositionType::Relative,
                ..default()
            }
        )],
    ));
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

fn move_moving(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        let change = velocity.0 * time.delta_secs();
        transform.translation += Vec3::new(change.x, change.y, 0.0);
    }
}

fn update_score_ui(score_res: Res<Score>, mut text_query: Single<&mut Text, With<ScoreTextUI>>) {
    if !score_res.is_changed() {
        return;
    }
    text_query.0 = format!("Score: {}", score_res.0);
}
