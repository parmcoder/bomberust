use crate::entities::{DroppedPiece, Piece, Position};
use crate::events::{PieceLandEvent, ResetFallTimerEvent};
use amethyst::assets::{Handle, AssetStorage};
use amethyst::core::ecs::{
    Entities, Join, Read, ReadExpect, ReadStorage, ReaderId, System, Write, WriteStorage,
};

use amethyst::core::{Time, Transform};
use amethyst::renderer::resources::Tint;
use amethyst::renderer::{SpriteRender, SpriteSheet};

use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::math::Vector3;
use crate::audio::{play_drop_sound, Sounds};
use amethyst::audio::Source;
use amethyst::audio::output::Output;

use crate::constants::FALL_TIMER;

// This is how a piece should drop
pub struct DroppingSystem {
    fall_timer: f32, // Seconds until next step down
    reader_id: Option<ReaderId<ResetFallTimerEvent>>,
}

impl DroppingSystem {
    pub fn new() -> Self {
        Self {
            fall_timer: FALL_TIMER,
            reader_id: None,
        }
    }
}
impl<'s> System<'s> for DroppingSystem {

    // There are plenty of data we need to use
    type SystemData = (
        ReadStorage<'s, Piece>,
        WriteStorage<'s, DroppedPiece>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Entities<'s>,
        Write<'s, EventChannel<PieceLandEvent>>,
        Write<'s, EventChannel<ResetFallTimerEvent>>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteStorage<'s, Tint>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
            pieces,
            mut dropped_pieces,
            mut positions,
            mut transforms,
            time,
            entities,
            mut land_channel,
            mut reset_channel,
            mut sprite_renders,
            sprite_sheet_handle,
            mut tints,
            storage, sounds, audio_output
        ): Self::SystemData,
    ) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| reset_channel.register_reader());

        for _ in reset_channel.read(reader_id) {
            self.fall_timer = FALL_TIMER;
        }

        self.fall_timer -= time.delta_seconds();

        // Wait until the next fall, if the time has come, then do these...
        if self.fall_timer <= 0.0 {

            // reset the fall timer, so that we can move it again
            self.fall_timer = FALL_TIMER;

            // get the data of dropped pieces
            let dropped_positions = (&mut dropped_pieces, &mut positions)
                .join()
                .map(|(_, pos)| *pos)
                .collect::<Vec<_>>();

            // prepare this vector for our next drop pieces
            let mut last_dropped_pieces = Vec::<(DroppedPiece, Position)>::new();

            // we are gonna check if dropping pieces will collide or not
            for (entity, piece, position) in (&*entities, &pieces, &mut positions).join() {
                let mut collide = false;

                // check collision, whether there is any piece below or reached the floor
                for self_pos in piece.get_filled_positions(position) {
                    if self_pos.row == 0 {
                        collide = true;
                        break;
                    }

                    for other_pos in &dropped_positions {
                        let pos_below_self = Position {
                            row: self_pos.row - 1,
                            col: self_pos.col,
                        };
                        if pos_below_self == *other_pos {
                            collide = true;
                        }
                    }
                }

                // if collision occur, then just say that we have landed the piece here.
                if collide {
                    for new_dropped_pos in piece.get_filled_positions(position) {
                        last_dropped_pieces
                            .push((DroppedPiece::new(piece.piece_type), new_dropped_pos));
                    }
                    entities.delete(entity).unwrap();

                    land_channel.single_write(PieceLandEvent {});
                    play_drop_sound(&*sounds, &storage, audio_output.as_deref());
                } else {
                    position.row -= 1;
                }
            }

            // for every pieces we have landed, we need to draw it. The rendering system cannot interfere.
            for (new_dropped_piece, new_pos) in last_dropped_pieces {
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: 0,
                };

                let mut sprite_transform = Transform::default();
                sprite_transform.set_scale(Vector3::new(0.065, 0.065, 1.0));
                sprite_transform.set_translation_xyz(
                    new_pos.col as f32 + 0.5,
                    new_pos.row as f32 + 0.5,
                    0.0,
                );

                let tint = Tint(new_dropped_piece.piece_type.get_color());

                entities
                    .build_entity()
                    .with(new_dropped_piece, &mut dropped_pieces)
                    .with(new_pos, &mut positions)
                    .with(sprite_render, &mut sprite_renders)
                    .with(sprite_transform, &mut transforms)
                    .with(tint, &mut tints)
                    .build();
            }
        }
    }
}
