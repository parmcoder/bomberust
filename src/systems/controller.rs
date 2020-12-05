use amethyst::{
    core::Time,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

use crate::constants::BOARD_WIDTH;
use crate::entities::{DroppedPiece, Piece, Position};
use crate::events::ResetFallTimerEvent;
use std::collections::{HashMap, HashSet};

/*
For this one, we need to know what actions have been performed by the player.
*/
#[derive(SystemDesc)]
pub struct PieceInputSystem {
    last_actions: HashSet<String>,
    action_timers: HashMap<String, f32>,
}

impl PieceInputSystem {
    pub fn new() -> Self {
        Self {
            last_actions: HashSet::new(),
            action_timers: HashMap::new(),
        }
    }

    /* Sometimes when user want to spam the button for faster action,
    and we do not want them to break our game.

    Parallel system will work on their own but the rendering might not be able to handle them.
    Also, it is a cheating move so this function prevent them from doing that.
    */
    fn action_no_spam(&mut self, input: &InputHandler<StringBindings>, name: &str) -> bool {
        let contains = self.last_actions.contains(name);
        let action = input.action_is_down(name).unwrap_or(false);
        if contains && !action {
            self.last_actions.remove(name);
        } else if !contains && action {
            self.last_actions.insert(String::from(name));
        } else if contains && action {
            return false;
        }

        action
    }

    /**
    Remember when the for loop runs, it runs super fast! Here we use the delta_seconds to measure
    how far we want the user to move.
    */
    fn action_with_timer<T: PartialEq>(
        &mut self,
        time: &Time,
        default_seconds: f32,
        name: &str,
        value: T,
        default_value: T,
    ) -> T {
        let timer = self
            .action_timers
            .entry(String::from(name))
            .or_insert(default_seconds);

        if *timer <= 0.0 {
            if value != default_value {
                *timer = default_seconds;
            }
            value
        } else if value == default_value {
            *timer = 0.0;
            default_value
        } else {
            *timer -= time.delta_seconds();
            default_value
        }
    }

    /*
    Of course, once we found that there are any dropped piece that collides. We can avoid so illegal moves.
    This can be further improved into kick detection where player can do tricky moves.
    */
    fn position_collides(piece: &Piece, position: &Position, dropped_pieces: &[Position]) -> bool {
        for self_pos in piece.get_filled_positions(&position) {
            let outside_bounds =
                || self_pos.col < 0 || self_pos.col >= BOARD_WIDTH as i8 || self_pos.row < 0;
            let in_dropped = || dropped_pieces.iter().any(|dropped_pos| self_pos == *dropped_pos);
            if outside_bounds() || in_dropped() {
                return true;
            }
        }

        false
    }

    /*
    Hard drop is the function that send the user down to the floor immediately and we will let the drop system
    handle what to do next.
    */
    fn hard_drop(piece: &Piece, position: &mut Position, dropped_positions: &[Position]) {
        let down_collides = |pos: &Position| {
            let down_pos = Position {
                row: pos.row - 1,
                col: pos.col,
            };

            Self::position_collides(piece, &down_pos, dropped_positions)
        };

        while !down_collides(position) {
            position.row -= 1;
        }
    }
}

impl<'s> System<'s> for PieceInputSystem {
    type SystemData = (
        WriteStorage<'s, Piece>,
        WriteStorage<'s, DroppedPiece>,
        WriteStorage<'s, Position>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, EventChannel<ResetFallTimerEvent>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut pieces, mut dropped_pieces, mut positions, input, mut reset_channel, time): Self::SystemData,
    ) {

        let dropped_positions = (&mut dropped_pieces, &mut positions)
            .join()
            .map(|(_, pos)| *pos)
            .collect::<Vec<_>>();

        // let's move those pieces
        for (piece, position) in (&mut pieces, &mut positions).join() {

            // Make sure it is no spam, the hard drop
            if self.action_no_spam(&*input, &"drop_hard".to_string()) {
                Self::hard_drop(piece, position, &dropped_positions);
            }

            let movement_input = input.axis_value("move_x").unwrap_or(0.0);
            // Move it according to the move_x value
            let movement = self.action_with_timer(&*time, 0.08, "move_x", movement_input, 0.0);

            let soft_drop_input = input.action_is_down("drop_soft").unwrap_or(false);
            let soft_drop =
                self.action_with_timer(&*time, 0.14, "drop_soft", soft_drop_input, false);

            let new_position = Position {
                row: position.row - soft_drop as i8,
                col: position.col - movement as i8,
            };

            let mut new_piece = Piece {
                piece_type: piece.piece_type,
                rotation: piece.rotation,
            };

            // Make sure that the rotation is in order
            let rotated = self.action_no_spam(&*input, &"rotate_cw".to_string());
            let rotated_ccw = self.action_no_spam(&*input, &"rotate_ccw".to_string());

            if rotated {
                new_piece.rotate_cw();
            } else if rotated_ccw {
                new_piece.rotate_ccw();
            } else if movement == 0.0 && !soft_drop {
                continue;
            }

            // if collision occurs, the piece cannot move. We can turn this into SRS later.
            if Self::position_collides(&new_piece, &new_position, &dropped_positions) {
                continue;
            }

            // Let's reset the time and keep the up with the change
            if position.row != new_position.row {
                reset_channel.single_write(ResetFallTimerEvent {});
            }

            // Change position, rotation
            position.row = new_position.row;
            position.col = new_position.col;
            piece.rotation = new_piece.rotation;
        }
    }
}
