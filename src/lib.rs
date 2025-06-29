use bevy::prelude::*;

use crate::{
    menu::GameState,
    screen::{despawn_screen, spawn_screen},
};

mod camera;
mod game;
mod menu;
mod screen;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            camera::plugin,
            screen::plugin,
            menu::plugin,
            game::plugin,
        ))
        .add_systems(OnEnter(GameState::Game), spawn_screen)
        .add_systems(OnExit(GameState::Game), despawn_screen);
    }
}
