use bevy::{asset::AssetMetaCheck, color::palettes::css::BLACK, prelude::*};

mod audio;
mod camera;
mod game;
mod game_over;
mod game_state;
mod main_menu;
mod widget;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#game-canvas".into()),
                        ..default()
                    }),
                    ..default()
                }),
            audio::plugin,
            camera::plugin,
            game_state::plugin,
            game::plugin,
            main_menu::plugin,
            game_over::plugin,
            widget::plugin,
        ))
        .insert_resource(ClearColor(Color::from(BLACK)));
    }
}
