use bevy::{
    color::palettes::css::WHITE_SMOKE, ecs::system::SystemId, math::bounding::BoundingCircle,
    platform::collections::HashMap, prelude::*,
};

use crate::{
    game::{
        common::{components::*, constants::*, system_sets::GameplaySet},
        game_events::CollisionEvent,
    },
    game_state::{GameState, OnGameState},
};

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

#[derive(Resource)]
pub struct Lives(pub u32);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, on_collision_event.in_set(GameplaySet))
        .insert_resource(Lives(3))
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
        Transform::from_xyz(0., 0., 0.),
        Velocity(BALL_START_VELOCITY),
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::from(WHITE_SMOKE))),
        Ball,
        BC(BoundingCircle::new(Vec2::new(0., 0.), BALL_RADIUS)),
        Dynamic,
    ));
}

fn on_collision_event(
    mut commands: Commands,
    mut collision_reader: EventReader<CollisionEvent>,
    mut lives_res: ResMut<Lives>,
    systems: Res<BallOneShotSystems>,
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
                lives_res.0 -= 1;
                let id = systems.0[&BallSystemId::SpawnBall];
                commands.run_system(id);
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
