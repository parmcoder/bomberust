use amethyst::{
    assets::{Loader,AssetStorage},
    ecs::{World, WorldExt},
    audio::{output::Output, Source, SourceHandle, AudioSink},
};
use amethyst::audio::OggFormat;
use std::{iter::Cycle, vec::IntoIter};
use crate::audio::sounds::Sounds;
use crate::constants::{MUSIC_TRACKS, DROP_SOUND};


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
            bounce_sfx: load_audio_track(&loader, &world, DROP_SOUND),
            score_sfx: load_audio_track(&loader, &world, DROP_SOUND),
        };

        (sound, music)

    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
    world.insert(music);

}