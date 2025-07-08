use bevy::prelude::*;

mod ball;
mod common;
mod game_events;
mod game_ui;
mod life;
mod physics;
mod player;
mod score;
mod tile;
mod wall;

use crate::game::common::system_sets;

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
        life::plugin,
    ));
}
