use crate::constants::{MUSIC_TRACKS, DROP_SOUND, CLEAR_SOUND};
use amethyst::audio::OggFormat;
use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, Source, SourceHandle},
    ecs::{World, WorldExt},
};
use std::{iter::Cycle, vec::IntoIter};

pub struct Sounds {
    pub clear_sfx: SourceHandle,
    pub drop_sfx: SourceHandle,
}
pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}
/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

/// Initialise audio in the world. This will eventually include
/// the background tracks as well as the sound effects, but for now
/// we'll just work on sound effects.
pub fn initialise_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25); // Music is a bit loud, reduce the volume.
        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };
        let sound = Sounds {
            drop_sfx: load_audio_track(&loader, &world, DROP_SOUND),
            clear_sfx: load_audio_track(&loader, &world, CLEAR_SOUND),
        };

        (sound, music)
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
    world.insert(music);
}

pub fn play_drop_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.drop_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_clear_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.clear_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}
