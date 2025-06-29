use bevy::prelude::*;

mod ball;
mod common;
mod game_events;
mod game_ui;
mod physics;
mod player;
mod score;
mod tile;
mod wall;

use crate::{game::common::system_sets, game_state::GameState};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        ball::plugin,
        wall::plugin,
        tile::plugin,
        physics::plugin,
        game_ui::plugin,
        score::plugin,
        game_events::plugin,
        system_sets::plugin,
    ))
    .add_systems(
        OnEnter(GameState::Game),
        (
            player::setup_player_paddle,
            ball::setup_ball,
            wall::setup_walls,
            tile::setup_tiles,
            game_ui::setup_ui,
            score::setup_score,
        ),
    );
}
