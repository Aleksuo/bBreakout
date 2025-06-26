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

use common::system_sets::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        DefaultPlugins,
        player::plugin,
        ball::plugin,
        wall::plugin,
        tile::plugin,
        physics::plugin,
        game_ui::plugin,
        score::plugin,
        game_events::plugin,
    ))
    .configure_sets(
        FixedUpdate,
        (InputSet.before(PhysicsSet), GameplaySet.after(PhysicsSet)),
    );
}
