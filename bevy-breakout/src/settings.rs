use bevy::{
    audio::Volume,
    color::palettes::css::{BLACK, WHITE_SMOKE},
    prelude::*,
};

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
    DecreaseVolume,
    IncreaseVolume,
    MainMenu,
}

#[derive(Component)]
struct VolumeTextField;

const VOLUME_INCREMENT: f32 = 0.05;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Settings), spawn_menu)
        .add_systems(
            Update,
            (menu_action, update_volume_text).run_if(in_state(GameState::Settings)),
        );
}

fn spawn_menu(mut commands: Commands, global_volume: Res<GlobalVolume>) {
    commands.spawn((
        OnGameState(GameState::Settings),
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
                (Text::new("Volume:"), TextColor(Color::from(WHITE_SMOKE))),
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    children![
                        (
                            SettingsMenuAction::DecreaseVolume,
                            MenuButton,
                            children![(Text::new("<"), TextColor(Color::from(BLACK)))]
                        ),
                        (
                            VolumeTextField,
                            Text::new(format!("{:.2}", global_volume.volume.to_linear())),
                            TextColor(Color::from(WHITE_SMOKE))
                        ),
                        (
                            SettingsMenuAction::IncreaseVolume,
                            MenuButton,
                            children![(Text::new(">"), TextColor(Color::from(BLACK)))]
                        ),
                    ],
                ),
                (
                    MenuButton,
                    SettingsMenuAction::MainMenu,
                    children![(
                        Text::new("Back to main menu"),
                        TextColor(Color::from(BLACK))
                    )]
                )
            ]
        )],
    ));
}

fn menu_action(
    interaction_query: SettingsMenuActionInteractionQuery,
    mut game_state: ResMut<NextState<GameState>>,
    mut global_volume: ResMut<GlobalVolume>,
) {
    for (interaction, game_over_menu_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match game_over_menu_action {
                SettingsMenuAction::MainMenu => {
                    game_state.set(GameState::MainMenu);
                }
                SettingsMenuAction::DecreaseVolume => {
                    let mut current_lin_vol = global_volume.volume.to_linear();
                    current_lin_vol -= VOLUME_INCREMENT;
                    global_volume.volume = Volume::Linear(current_lin_vol.clamp(0., 1.));
                }
                SettingsMenuAction::IncreaseVolume => {
                    let mut current_lin_vol = global_volume.volume.to_linear();
                    current_lin_vol += VOLUME_INCREMENT;
                    global_volume.volume = Volume::Linear(current_lin_vol.clamp(0., 1.));
                }
            }
        }
    }
}

fn update_volume_text(
    global_volume: Res<GlobalVolume>,
    mut text_query: Single<&mut Text, With<VolumeTextField>>,
) {
    if !global_volume.is_changed() {
        return;
    }
    text_query.0 = format!("{:.2}", global_volume.volume.to_linear());
}
