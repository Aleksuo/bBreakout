use bevy::{
    color::palettes::css::{BLACK, GRAY, WHITE_SMOKE},
    prelude::*,
};

type ButtonInteractionQuery<'w, 's> = Query<
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
    app.add_systems(Update, menu_button_interaction);
}

fn menu_button_interaction(mut interaction_query: ButtonInteractionQuery) {
    for (interaction, mut border_color) in interaction_query.iter_mut() {
        border_color.0 = match interaction {
            Interaction::Hovered => Color::from(GRAY),
            Interaction::Pressed => Color::from(GRAY),
            Interaction::None => Color::from(WHITE_SMOKE),
        }
    }
}
