use bevy::{prelude::*, sprite::Material2d};

use crate::{
    game::common::system_sets::GameplaySet,
    game_state::{GameState, OnGameState},
};

#[derive(Component)]
struct ParticleMarker;

#[derive(Component)]
pub struct ShrinkingParticle;

#[derive(Component)]
struct ParticleLifeTimer(Timer);

#[derive(Bundle)]
pub struct ParticleBundle<M: Material2d> {
    transform: Transform,
    mesh: Mesh2d,
    material: MeshMaterial2d<M>,
    marker: ParticleMarker,
    lifetime: ParticleLifeTimer,
    active_state: OnGameState,
}

impl<M: Material2d> ParticleBundle<M> {
    pub fn new(pos_x: f32, pos_y: f32, mesh: Mesh2d, material: MeshMaterial2d<M>) -> Self {
        ParticleBundle {
            transform: Transform::from_xyz(pos_x, pos_y, -1.),
            mesh,
            material,
            marker: ParticleMarker,
            lifetime: ParticleLifeTimer(Timer::from_seconds(0.5, TimerMode::Once)),
            active_state: OnGameState(GameState::Game),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (tick_particle_lifetimes, shrink_shrinking_particles).in_set(GameplaySet),
    );
}

fn tick_particle_lifetimes(
    mut commands: Commands,
    mut query: Query<(&mut ParticleLifeTimer, Entity), With<ParticleMarker>>,
    time: Res<Time>,
) {
    for (mut timer, entity) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn shrink_shrinking_particles(
    mut query: Query<(&mut Transform, &ParticleLifeTimer), With<ShrinkingParticle>>,
) {
    for (mut transform, timer) in query.iter_mut() {
        let scale_factor = timer.0.fraction_remaining();
        *transform = transform.with_scale(Vec3::new(scale_factor, scale_factor, 1.));
    }
}
