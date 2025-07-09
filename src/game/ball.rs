use bevy::color::palettes::css::GRAY;
use bevy::{
    color::palettes::css::WHITE_SMOKE, ecs::system::SystemId, math::bounding::BoundingCircle,
    platform::collections::HashMap, prelude::*,
};

use crate::game::particle::{ParticleBundle, ShrinkingParticle};
use crate::{
    game::{
        common::{components::*, constants::*, system_sets::GameplaySet},
        game_events::{BallDestroyedEvent, BallSpawnEvent, CollisionEvent},
    },
    game_state::{GameState, OnGameState},
};

#[derive(Component)]
pub struct BallSpeedUpTimer(Timer);

#[derive(Component)]
pub struct TrailSpawnTimer(Timer);

#[derive(Component)]
pub struct Ball;

#[derive(Eq, Hash, PartialEq)]
enum BallSystemId {
    SpawnBall,
}

#[derive(Resource)]
pub struct BallOneShotSystems(HashMap<BallSystemId, SystemId>);

impl FromWorld for BallOneShotSystems {
    fn from_world(world: &mut World) -> Self {
        let mut ball_one_shot_systems = BallOneShotSystems(HashMap::new());
        ball_one_shot_systems
            .0
            .insert(BallSystemId::SpawnBall, world.register_system(spawn_ball));
        ball_one_shot_systems
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup_ball)
        .add_systems(
            FixedUpdate,
            (
                on_spawn_ball_event,
                on_collision_event,
                tick_ball_speed_up_timers,
                tick_ball_trail_spawners,
            )
                .in_set(GameplaySet),
        )
        .init_resource::<BallOneShotSystems>();
}

pub fn setup_ball(mut commands: Commands, systems: Res<BallOneShotSystems>) {
    let id = systems.0[&BallSystemId::SpawnBall];
    commands.run_system(id);
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        OnGameState(GameState::Game),
        BallSpeedUpTimer(Timer::from_seconds(
            BALL_SPEED_UP_INTERVAL,
            TimerMode::Repeating,
        )),
        TrailSpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Transform::from_xyz(0., 0., 0.),
        Velocity(Vec2::new(0., -BALL_START_VELOCITY)),
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::from(WHITE_SMOKE))),
        Ball,
        BC(BoundingCircle::new(Vec2::new(0., 0.), BALL_RADIUS)),
        Dynamic,
    ));
}

fn tick_ball_speed_up_timers(
    mut commands: Commands,
    mut query: Query<(&mut BallSpeedUpTimer, &mut Velocity, Entity), With<Ball>>,
    time: Res<Time>,
) {
    for (mut timer, mut velocity, entity) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let dir = velocity.0.normalize();
            let mut len = velocity.0.length();
            len += BALL_SPEED_INCREMENT_STEP;
            if len >= BALL_MAX_VELOCITY {
                velocity.0 = dir * BALL_MAX_VELOCITY;
                commands.entity(entity).remove::<BallSpeedUpTimer>();
            } else {
                velocity.0 = dir * len;
            }
        }
    }
}

fn tick_ball_trail_spawners(
    mut commands: Commands,
    mut query: Query<(&mut TrailSpawnTimer, &Transform), With<Ball>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (mut timer, transform) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let translation = transform.translation;
            commands.spawn((
                ParticleBundle::new(
                    translation.x,
                    translation.y,
                    Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
                    MeshMaterial2d(materials.add(Color::from(GRAY))),
                ),
                ShrinkingParticle,
            ));
        }
    }
}

fn on_spawn_ball_event(
    mut commands: Commands,
    mut spawn_ball_reader: EventReader<BallSpawnEvent>,
    systems: Res<BallOneShotSystems>,
) {
    let id = systems.0[&BallSystemId::SpawnBall];
    for _ in spawn_ball_reader.read() {
        commands.run_system(id);
    }
}

fn on_collision_event(
    mut commands: Commands,
    mut collision_reader: EventReader<CollisionEvent>,
    mut ball_destroyed_writer: EventWriter<BallDestroyedEvent>,
    instant_death_q: Query<&InstantDeath>,
    balls_q: Query<&Ball>,
) {
    for col_event in collision_reader.read() {
        if let Some(ball_entity) = maybe_collision_has_ball(col_event, balls_q) {
            let other_entity = other_entity_in_col(&ball_entity, col_event);
            if let Some(_instant_death) =
                maybe_entity_is_instant_death(&other_entity, instant_death_q)
            {
                commands.entity(ball_entity).despawn();
                ball_destroyed_writer.write(BallDestroyedEvent);
            }
        }
    }
}

fn maybe_collision_has_ball(col_event: &CollisionEvent, balls_q: Query<&Ball>) -> Option<Entity> {
    maybe_entity_is_ball(&col_event.0, balls_q).or(maybe_entity_is_ball(&col_event.1, balls_q))
}

fn other_entity_in_col(first_entity: &Entity, col_event: &CollisionEvent) -> Entity {
    if *first_entity == col_event.0 {
        return col_event.1;
    }
    col_event.0
}

fn maybe_entity_is_ball(entity: &Entity, balls_q: Query<&Ball>) -> Option<Entity> {
    match balls_q.get(*entity) {
        Ok(_ball) => Some(*entity),
        Err(_e) => None,
    }
}

fn maybe_entity_is_instant_death(
    entity: &Entity,
    instant_death_q: Query<&InstantDeath>,
) -> Option<Entity> {
    match instant_death_q.get(*entity) {
        Ok(_instant_death) => Some(*entity),
        Err(_e) => None,
    }
}
