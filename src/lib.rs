use bevy::prelude::*;

mod game;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(game::plugin);
    }
}
