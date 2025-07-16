use bevy::{color::palettes::css::BLACK, prelude::*};

mod camera;
mod game;
mod game_over;
mod game_state;
mod main_menu;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            camera::plugin,
            game_state::plugin,
            game::plugin,
            main_menu::plugin,
            game_over::plugin,
        ))
        .insert_resource(ClearColor(Color::from(BLACK)));
    }
}
