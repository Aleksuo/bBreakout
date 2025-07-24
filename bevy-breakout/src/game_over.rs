use bevy::{
    color::palettes::css::{BLACK, WHITE_SMOKE},
    prelude::*,
};

use crate::game_state::{GameState, OnGameState};

type GameOverMenuActionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static GameOverMenuAction),
    (Changed<Interaction>, With<Button>),
>;

#[derive(Component)]
enum GameOverMenuAction {
    NewGame,
    MainMenu,
}
#[derive(Resource, DerefMut, Deref)]
pub struct FinalScore(pub u32);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(FinalScore(0))
        .add_systems(OnEnter(GameState::GameOver), spawn_menu)
        .add_systems(Update, menu_action);
}

fn spawn_menu(mut commands: Commands, final_score_res: Res<FinalScore>) {
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
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![
                (
                    Text::new("Game over"),
                    TextFont {
                        font_size: 70.,
                        ..default()
                    },
                    TextColor(Color::from(WHITE_SMOKE)),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.)),
                        ..default()
                    }
                ),
                (
                    Text::new(format!("Final score: {}", final_score_res.0)),
                    TextColor(Color::from(WHITE_SMOKE)),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.)),
                        ..default()
                    }
                ),
                (
                    GameOverMenuAction::NewGame,
                    Button,
                    BackgroundColor(Color::from(BLACK)),
                    BorderColor(Color::from(WHITE_SMOKE)),
                    button_node.clone(),
                    children![Text::new("Try again"), TextColor(Color::from(WHITE_SMOKE))],
                ),
                (
                    GameOverMenuAction::MainMenu,
                    Button,
                    BackgroundColor(Color::from(BLACK)),
                    BorderColor(Color::from(WHITE_SMOKE)),
                    button_node.clone(),
                    children![
                        Text::new("Back to main menu"),
                        TextColor(Color::from(WHITE_SMOKE))
                    ],
                )
            ]
        )],
    ));
}

fn menu_action(
    interaction_query: GameOverMenuActionQuery,
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
