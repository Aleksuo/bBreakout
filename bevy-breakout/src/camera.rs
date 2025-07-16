use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, add_camera);
}

fn add_camera(mut commands: Commands) {
    commands.spawn((Camera2d,));
}
