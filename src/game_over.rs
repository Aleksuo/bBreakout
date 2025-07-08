use bevy::{
    color::palettes::css::{BLACK, WHITE_SMOKE},
    ecs::spawn::SpawnIter,
    prelude::*,
};

use crate::game_state::{GameState, OnGameState};

#[derive(Component)]
enum GameOverMenuAction {
    NewGame,
    MainMenu,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), spawn_menu)
        .add_systems(Update, menu_action);
}

fn spawn_menu(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        border: UiRect {
            top: Val::Px(10.),
            bottom: Val::Px(10.),
            right: Val::Px(10.),
            left: Val::Px(10.),
        },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    commands.spawn((
        OnGameState(GameState::GameOver),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Children::spawn(SpawnIter(
                [
                    (GameOverMenuAction::NewGame, "Try again"),
                    (GameOverMenuAction::MainMenu, "Back to main menu"),
                ]
                .into_iter()
                .map(move |(action, text)| {
                    (
                        Button,
                        BackgroundColor(Color::from(BLACK)),
                        BorderColor(Color::from(WHITE_SMOKE)),
                        button_node.clone(),
                        action,
                        children![Text::new(text), TextColor(Color::from(WHITE_SMOKE))],
                    )
                })
            ))
        )],
    ));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &GameOverMenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, game_over_menu_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match game_over_menu_action {
                GameOverMenuAction::MainMenu => {
                    game_state.set(GameState::MainMenu);
                }
                GameOverMenuAction::NewGame => {
                    game_state.set(GameState::Game);
                }
            }
        }
    }
}
