use bevy::{
    color::palettes::css::{GRAY, WHITE_SMOKE},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, button_interaction);
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut border_color) in interaction_query.iter_mut() {
        border_color.0 = match interaction {
            Interaction::Hovered => Color::from(GRAY),
            Interaction::Pressed => Color::from(GRAY),
            Interaction::None => Color::from(WHITE_SMOKE),
        }
    }
}
