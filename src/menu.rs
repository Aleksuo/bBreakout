use bevy::{color::palettes::css::BLUE, ecs::spawn::SpawnIter, prelude::*};

use crate::game_state::{self, GameState, OnGameState};

#[derive(Component)]
enum MenuButtonAction {
    NewGame,
    QuitGame,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), spawn_menu)
        .add_systems(Update, menu_action.run_if(in_state(GameState::MainMenu)));
}

fn spawn_menu(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    commands.spawn((
        OnGameState(GameState::MainMenu),
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
                    (MenuButtonAction::NewGame, "New game"),
                    (MenuButtonAction::QuitGame, "Quit game"),
                ]
                .into_iter()
                .map(move |(action, text)| {
                    (
                        Button,
                        BackgroundColor(Color::from(BLUE)),
                        button_node.clone(),
                        action,
                        children![Text::new(text)],
                    )
                })
            ))
        )],
    ));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::NewGame => {
                    game_state.set(GameState::Game);
                }
                MenuButtonAction::QuitGame => {
                    warn!("Quit game handler not implemented yet");
                }
            }
        }
    }
}
