use amethyst::{
    derive::SystemDesc,
    ecs::{
        prelude::{ReaderId, System, SystemData, Write, WriteStorage},
        Entities,
    },
    shrev::EventChannel,
};

use crate::constants::BOARD_HEIGHT;
use crate::entities::{Piece, Position};
use crate::events::PieceLandEvent;

#[derive(SystemDesc)]
pub struct PieceSpawnSystem {
    reader_id: Option<ReaderId<PieceLandEvent>>,
}

impl PieceSpawnSystem {
    pub fn new() -> Self {
        Self { reader_id: None }
    }
}

impl<'s> System<'s> for PieceSpawnSystem {
    type SystemData = (
        WriteStorage<'s, Piece>,
        Write<'s, EventChannel<PieceLandEvent>>,
        WriteStorage<'s, Position>,
        Entities<'s>,
    );

    fn run(&mut self, (mut blocks, mut land_channel, mut positions, entities): Self::SystemData) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| land_channel.register_reader());

        for _ in land_channel.read(reader_id) {
            let mut b = Piece::new(rand::random());
            b.rotation = 0;
            entities
                .build_entity()
                .with(b, &mut blocks)
                .with(
                    Position {
                        row: BOARD_HEIGHT as i8 - 4,
                        col: 3,
                    },
                    &mut positions,
                )
                .build();
        }
    }
}
