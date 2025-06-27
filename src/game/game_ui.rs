use bevy::prelude::*;

use crate::game::{ball::Lives, score::Score};

#[derive(Component)]
struct ScoreTextUI;

#[derive(Component)]
struct LivesTextUI;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, add_ui)
        .add_systems(Update, (update_score_ui, update_lives_ui));
}

fn update_score_ui(score_res: Res<Score>, mut text_query: Single<&mut Text, With<ScoreTextUI>>) {
    if !score_res.is_changed() {
        return;
    }
    text_query.0 = format!("Score: {}", score_res.0);
}

fn update_lives_ui(lives_res: Res<Lives>, mut text_query: Single<&mut Text, With<LivesTextUI>>) {
    if !lives_res.is_changed() {
        return;
    }
    text_query.0 = format!("Lives: {}", lives_res.0);
}

fn add_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        children![
            (
                ScoreTextUI,
                Text::new("Score: 0"),
                TextShadow::default(),
                Node {
                    position_type: PositionType::Relative,
                    ..default()
                }
            ),
            (
                LivesTextUI,
                Text::new("Lives: 3"),
                TextShadow::default(),
                Node {
                    position_type: PositionType::Relative,
                    ..default()
                }
            )
        ],
    ));
}
