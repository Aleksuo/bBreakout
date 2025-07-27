use bevy::{
    color::palettes::css::{BLACK, WHITE_SMOKE},
    prelude::*,
};

use crate::{
    game_state::{GameState, OnGameState},
    widget::MenuButton,
};

type MenuActionInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static MenuButtonAction),
    (Changed<Interaction>, With<Button>),
>;

#[derive(Component)]
enum MenuButtonAction {
    NewGame,
    Settings,
    #[cfg(not(target_arch = "wasm32"))]
    QuitGame,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), spawn_menu)
        .add_systems(Update, menu_action.run_if(in_state(GameState::MainMenu)));
}

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            OnGameState(GameState::MainMenu),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Text::new("bBreakout"),
                        TextFont {
                            font_size: 80.,
                            ..default()
                        },
                        TextColor(Color::from(WHITE_SMOKE)),
                        Node {
                            margin: UiRect::bottom(Val::Px(20.)),
                            ..default()
                        },
                    ));
                    [
                        (MenuButtonAction::NewGame, "New game"),
                        (MenuButtonAction::Settings, "Settings"),
                        #[cfg(not(target_arch = "wasm32"))]
                        (MenuButtonAction::QuitGame, "Quit game"),
                    ]
                    .into_iter()
                    .for_each(|(action, text)| {
                        col.spawn((
                            MenuButton,
                            action,
                            children![(Text::new(text), TextColor(Color::from(BLACK)))],
                        ));
                    });
                });
        });
}

fn menu_action(
    #[cfg(not(target_arch = "wasm32"))] mut app_exit: EventWriter<AppExit>,
    interaction_query: MenuActionInteractionQuery,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::NewGame => {
                    game_state.set(GameState::Game);
                }
                MenuButtonAction::Settings => {
                    game_state.set(GameState::Settings);
                }
                #[cfg(not(target_arch = "wasm32"))]
                MenuButtonAction::QuitGame => {
                    app_exit.write(AppExit::Success);
                }
            }
        }
    }
}
