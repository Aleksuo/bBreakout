use bevy::prelude::*;

use crate::game::{common::system_sets::GameplaySet, game_events::TileDestroyedEvent};

#[derive(Resource, DerefMut, Deref)]
pub struct Score(pub u32);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Score(0))
        .add_systems(FixedUpdate, on_tile_destroyed_event.in_set(GameplaySet));
}

fn on_tile_destroyed_event(
    mut score_res: ResMut<Score>,
    mut tile_destroyed_reader: EventReader<TileDestroyedEvent>,
) {
    for _event in tile_destroyed_reader.read() {
        score_res.0 += 1;
    }
}
