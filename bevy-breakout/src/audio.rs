use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource)]
pub struct SoundPlayer {
    sound_map: HashMap<PlaySoundEvent, Handle<AudioSource>>,
}

impl SoundPlayer {
    fn new(sound_map: HashMap<PlaySoundEvent, Handle<AudioSource>>) -> Self {
        Self { sound_map }
    }
}

#[derive(Event, PartialEq, Eq, Hash)]
pub enum PlaySoundEvent {
    BallHit,
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<PlaySoundEvent>()
        .add_systems(Startup, load_sound_assets)
        .add_systems(FixedUpdate, on_play_sound_event);
}

fn load_sound_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_hit_sound = asset_server.load::<AudioSource>("sounds/BallHit.wav");
    let mut sound_map = HashMap::new();
    sound_map.insert(PlaySoundEvent::BallHit, ball_hit_sound);

    commands.insert_resource(SoundPlayer::new(sound_map));
}

fn on_play_sound_event(
    mut commands: Commands,
    mut event_reader: EventReader<PlaySoundEvent>,
    sound_player: Res<SoundPlayer>,
) {
    if event_reader.is_empty() {
        return;
    }
    for event in event_reader.read() {
        let maybe_sound_handle = sound_player.sound_map.get(event);
        if let Some(sound_handle) = maybe_sound_handle {
            commands.spawn((
                AudioPlayer(sound_handle.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    ..default()
                },
            ));
        } else {
            error!("Loading sound handle failed");
        }
    }
}
