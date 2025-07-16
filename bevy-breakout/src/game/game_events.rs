use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<CollisionEvent>()
        .add_event::<TileDestroyedEvent>()
        .add_event::<BallDestroyedEvent>()
        .add_event::<BallSpawnEvent>();
}

#[derive(Event)]
pub struct CollisionEvent(pub Entity, pub Entity);

#[derive(Event)]
pub struct BallSpawnEvent;

#[derive(Event)]
pub struct TileDestroyedEvent;

#[derive(Event)]
pub struct BallDestroyedEvent;
