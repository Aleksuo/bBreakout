use bevy::{
    color::palettes::css::{BLACK, GRAY, WHITE_SMOKE},
    prelude::*,
};

use crate::audio::PlaySoundEvent;

type MenuButtonBorderInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static mut BorderColor),
    (Changed<Interaction>, With<MenuButton>),
>;

fn button_node() -> Node {
    Node {
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
    }
}

#[derive(Component)]
#[require(
    Button,
    BackgroundColor = BLACK,
    BorderColor = WHITE_SMOKE,
    Node = button_node()
)]
pub struct MenuButton;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            menu_button_border_interaction,
            menu_button_sound_interaction,
        ),
    );
}

fn menu_button_border_interaction(mut interaction_query: MenuButtonBorderInteractionQuery) {
    for (interaction, mut border_color) in interaction_query.iter_mut() {
        border_color.0 = match interaction {
            Interaction::Hovered => Color::from(GRAY),
            Interaction::Pressed => Color::from(GRAY),
            Interaction::None => Color::from(WHITE_SMOKE),
        }
    }
}

fn menu_button_sound_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut play_sound_writer: EventWriter<PlaySoundEvent>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                play_sound_writer.write(PlaySoundEvent::BallHit);
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}
