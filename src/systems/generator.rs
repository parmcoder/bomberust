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

// Just simple rng generator
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

    fn run(&mut self, (mut pieces, mut land_channel, mut positions, entities): Self::SystemData) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| land_channel.register_reader());

        // when a piece is landed, we generate a new one.
        for _ in land_channel.read(reader_id) {
            let mut b = Piece::new(rand::random());
            b.rotation = 0;
            entities
                .build_entity()
                .with(b, &mut pieces)
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
    // PS. In modern tetris, the system is called 7-bag, here is the explaination.
    /*
    https://tetris.fandom.com/wiki/Random_Generator
    
    Random Generator generates a sequence of all seven one-sided tetrominoes 
    (I, J, L, O, S, T, Z) permuted randomly, as if they were drawn from a bag. 
    Then it deals all seven tetrominoes to the piece sequence before generating another bag. 
    There are 7!, or 5,040, permutations of seven elements, and it is believed that Tetris assigns a nearly equal 
    probability to each of these, making it much less likely that the player will get an obscenely long run without 
    a desired tetromino. It can produce a maximum of 12 tetrominoes between one I and the next I, and a run of S and Z 
    tetrominoes is limited to a maximum of 4. Exception: In Random Generator as implemented in Tetris The Grand Master Ace, 
    the first piece of the first bag is always I, J, L, or T, just as in the traditional TGM randomizer.
    */
}
