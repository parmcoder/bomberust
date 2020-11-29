use amethyst::{
    assets::{Loader,AssetStorage},
    ecs::{World, WorldExt},
    audio::{output::Output, Source, SourceHandle, AudioSink},
};
use amethyst::audio::OggFormat;
use std::{iter::Cycle, vec::IntoIter};


const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

pub fn play_bounce_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_score_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}