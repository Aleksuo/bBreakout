use bevy::{
    color::palettes::css::{BLACK, WHITE_SMOKE},
    prelude::*,
};

use crate::game_state::{GameState, OnGameState};

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
        OnGameState(GameState::MainMenu),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        BackgroundColor(Color::from(BLACK)),
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![
                (
                    Text::new("bBreakout"),
                    TextFont {
                        font_size: 80.,
                        ..default()
                    },
                    TextColor(Color::from(WHITE_SMOKE)),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.)),
                        ..default()
                    }
                ),
                (
                    Button,
                    BackgroundColor(Color::from(BLACK)),
                    BorderColor(Color::from(WHITE_SMOKE)),
                    button_node.clone(),
                    MenuButtonAction::NewGame,
                    children![Text::new("New game"), TextColor(Color::from(WHITE_SMOKE))],
                ),
                (
                    Button,
                    BackgroundColor(Color::from(BLACK)),
                    BorderColor(Color::from(WHITE_SMOKE)),
                    button_node.clone(),
                    MenuButtonAction::Settings,
                    children![Text::new("Settings"), TextColor(Color::from(WHITE_SMOKE))],
                ),
                #[cfg(not(target_arch = "wasm32"))]
                (
                    Button,
                    BackgroundColor(Color::from(BLACK)),
                    BorderColor(Color::from(WHITE_SMOKE)),
                    button_node.clone(),
                    MenuButtonAction::QuitGame,
                    children![Text::new("Quit game"), TextColor(Color::from(WHITE_SMOKE))],
                ),
            ],
        )],
    ));
}

fn menu_action(
    mut app_exit: EventWriter<AppExit>,
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
                    warn!("Settings handler is not implemented yet")
                }
                MenuButtonAction::QuitGame => {
                    app_exit.write(AppExit::Success);
                }
            }
        }
    }
}
