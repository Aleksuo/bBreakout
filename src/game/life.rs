use bevy::prelude::*;

use crate::{
    game::{
        ball::Ball,
        common::system_sets::GameplaySet,
        game_events::{BallDestroyedEvent, BallSpawnEvent},
    },
    game_state::GameState,
};

#[derive(Resource)]
pub struct Lives(pub u32);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Lives(3))
        .add_systems(OnEnter(GameState::Game), setup_lives)
        .add_systems(FixedUpdate, on_ball_destroyed.in_set(GameplaySet));
}

fn setup_lives(mut lives_res: ResMut<Lives>) {
    lives_res.0 = 3;
}

fn on_ball_destroyed(
    mut ball_destroyed_reader: EventReader<BallDestroyedEvent>,
    mut lives_res: ResMut<Lives>,
    mut ball_spawn_writer: EventWriter<BallSpawnEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    ball_query: Query<&Ball>,
) {
    if ball_destroyed_reader.is_empty() {
        return;
    }
    if ball_query.iter().len() <= ball_destroyed_reader.len() && lives_res.0 > 0 {
        lives_res.0 -= 1;
        if lives_res.0 > 0 {
            ball_spawn_writer.write(BallSpawnEvent);
        }
    }

    if lives_res.0 == 0 {
        game_state.set(GameState::GameOver);
    }

    ball_destroyed_reader.clear();
}
