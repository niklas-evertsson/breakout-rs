use bevy::{
    audio::{Audio, AudioSource},
    prelude::{EventReader, Handle, Res},
};

use crate::events::CollisionEvent;

const MUTE: bool = true;

pub struct CollisionSound(pub Handle<AudioSource>);

pub fn play_collision_sound(
    collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<CollisionSound>,
) {
    if MUTE {
        return;
    }

    if !collision_events.is_empty() {
        audio.play(sound.0.clone());
    }
}
