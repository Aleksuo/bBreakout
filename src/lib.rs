use bevy::prelude::*;

mod camera;
mod game;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::plugin, game::plugin));
    }
}
