use bevy::{color::palettes::css::WHITE_SMOKE, prelude::*};

use crate::{
    game_state::{GameState, OnGameState},
    widget::MenuButton,
};

type SettingsMenuActionInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static SettingsMenuAction),
    (Changed<Interaction>, With<Button>),
>;

#[derive(Component)]
enum SettingsMenuAction {
    MainMenu,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Settings), spawn_menu)
        .add_systems(Update, menu_action.run_if(in_state(GameState::Settings)));
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        OnGameState(GameState::GameOver),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(
                MenuButton,
                children![
                    SettingsMenuAction::MainMenu,
                    Text::new("Back to main menu"),
                    TextColor(Color::from(WHITE_SMOKE))
                ]
            )]
        ],
    ));
}

fn menu_action(
    interaction_query: SettingsMenuActionInteractionQuery,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, game_over_menu_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match game_over_menu_action {
                SettingsMenuAction::MainMenu => {
                    game_state.set(GameState::MainMenu);
                }
            }
        }
    }
}
