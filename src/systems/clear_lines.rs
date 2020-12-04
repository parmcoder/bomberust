use amethyst::{
    derive::SystemDesc,
    ecs::{
        prelude::{Join, ReaderId, System, SystemData, Write, WriteStorage},
        Entities,
    },
    shrev::EventChannel,
};

use crate::audio::{play_clear_sound, Sounds};
use crate::constants::BOARD_WIDTH;
use crate::entities::{DroppedPiece, Position};
use crate::events::PieceLandEvent;
use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::ecs::{Read, ReadExpect};
use amethyst::core::Transform;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(SystemDesc)]
pub struct LineClearSystem {
    reader_id: Option<ReaderId<PieceLandEvent>>,
}

impl LineClearSystem {
    pub fn new() -> Self {
        Self { reader_id: None }
    }
}

impl<'s> System<'s> for LineClearSystem {
    type SystemData = (
        WriteStorage<'s, DroppedPiece>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        Write<'s, EventChannel<PieceLandEvent>>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
            mut dropped_pieces,
            mut positions,
            mut transforms,
            entities,
            mut land_channel,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| land_channel.register_reader());

        for _ in land_channel.read(reader_id) {
            let mut drop_pos_row = HashMap::new();
            for (entity, _, dead_position, dead_transform) in (
                &*entities,
                &mut dropped_pieces,
                &mut positions,
                &mut transforms,
            )
                .join()
            {
                drop_pos_row
                    .entry(dead_position.row)
                    .or_insert_with(Vec::new)
                    .push((entity, dead_position, dead_transform));
            }

            let mut rows_to_descend: HashMap<i8, i8> = HashMap::new();
            for dead_row in drop_pos_row.keys() {
                if let Some(pieces_to_clear) = drop_pos_row.get(dead_row) {
                    if pieces_to_clear.len() >= BOARD_WIDTH as usize {
                        play_clear_sound(&*sounds, &storage, audio_output.as_deref());

                        for block_to_destroy in pieces_to_clear {
                            entities.delete(block_to_destroy.0).unwrap();
                        }

                        for other_row in drop_pos_row.keys().filter(|x| x > &dead_row) {
                            match rows_to_descend.entry(*other_row) {
                                Entry::Vacant(e) => {
                                    e.insert(1);
                                }
                                Entry::Occupied(mut e) => {
                                    *e.get_mut() += 1;
                                }
                            }
                        }
                    }
                }
            }

            for row_to_descend in rows_to_descend {
                if let Some(pieces_to_move) = drop_pos_row.get_mut(&row_to_descend.0) {
                    for piece_to_move in pieces_to_move {
                        piece_to_move.1.row -= row_to_descend.1;
                        piece_to_move
                            .2
                            .prepend_translation_y(-(row_to_descend.1 as f32));
                    }
                }
            }
        }
    }
}
