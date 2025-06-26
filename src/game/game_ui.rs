use bevy::prelude::*;

use crate::game::score::Score;

#[derive(Component)]
struct ScoreTextUI;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, add_ui)
        .add_systems(Update, update_score_ui);
}

fn update_score_ui(score_res: Res<Score>, mut text_query: Single<&mut Text, With<ScoreTextUI>>) {
    if !score_res.is_changed() {
        return;
    }
    text_query.0 = format!("Score: {}", score_res.0);
}

fn add_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        children![(
            ScoreTextUI,
            Text::new("Score: 0"),
            TextShadow::default(),
            Node {
                position_type: PositionType::Relative,
                ..default()
            }
        )],
    ));
}
