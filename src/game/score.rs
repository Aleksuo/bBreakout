use bevy::prelude::*;

#[derive(Resource, DerefMut, Deref)]
pub struct Score(pub u32);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
}
