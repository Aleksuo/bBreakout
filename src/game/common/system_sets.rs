use ::bevy::prelude::*;

use crate::game_state::GameState;

pub(crate) fn plugin(app: &mut App) {
    app.configure_sets(
        FixedUpdate,
        (InputSet.before(PhysicsSet), GameplaySet.after(PhysicsSet)),
    )
    .configure_sets(
        FixedUpdate,
        (
            InputSet.run_if(in_state(GameState::Game)),
            PhysicsSet.run_if(in_state(GameState::Game)),
            GameplaySet.run_if(in_state(GameState::Game)),
        ),
    )
    .configure_sets(
        Update,
        (
            InputSet.run_if(in_state(GameState::Game)),
            PhysicsSet.run_if(in_state(GameState::Game)),
            GameplaySet.run_if(in_state(GameState::Game)),
        ),
    );
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct InputSet;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct PhysicsSet;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct GameplaySet;
