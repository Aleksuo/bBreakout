use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    MainMenu,
    #[default]
    Game,
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>();
}
