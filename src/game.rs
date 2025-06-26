use bevy::prelude::*;

mod ball;
mod common;
mod game_ui;
mod physics;
mod player;
mod tile;
mod wall;

use common::system_sets::*;

#[derive(Resource, DerefMut, Deref)]
struct Score(u32);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        DefaultPlugins,
        player::plugin,
        ball::plugin,
        wall::plugin,
        tile::plugin,
        physics::plugin,
        game_ui::plugin,
    ))
    .configure_sets(FixedUpdate, InputSet.before(PhysicsSet))
    .insert_resource(Score(0));
}
