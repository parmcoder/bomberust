// how tetriminos are formed with components
use amethyst::{
    ecs::{Component, DenseVecStorage},
    renderer::palette::rgb::Srgba,
};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::entities::position::Position;
use amethyst::core::ecs::rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator};

//Normal Piece

pub struct Piece {
    pub block_type: PieceType,
    pub rotation: u8,
}

impl Piece {
    pub fn new(block_type: PieceType) -> Self {
        Self {
            block_type,
            rotation: 0,
        }
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation - 1) % 4;
    }

    pub fn get_filled_positions(&self, pos: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let shape: PieceShape = self.block_type.get_shape(self.rotation);
        (0..4).par_iter_mut().for_each(|row| {
            (0..4).par_iter_mut().for_each(|col| {
                if (shape & (1 << (row * 4 + col))) != 0 {
                    positions.push(Position {
                        row: pos.row + (3 - row),
                        col: pos.col + col,
                    })
                }
            })
        });
        positions
    }
}

impl Component for Piece {
    type Storage = DenseVecStorage<Self>;
}

// https://tetris.fandom.com/wiki/SRS
#[derive(Copy, Clone)]
pub enum PieceType {
    O,
    J,
    L,
    I,
    S,
    Z,
    T,
}

type PieceShape = u16;

impl PieceType {
    pub fn get_shape(&self, rotation: u8) -> PieceShape {
        let shapes = match *self {
            PieceType::O => [0xCC00, 0xCC00, 0xCC00, 0xCC00],
            PieceType::J => [0x44C0, 0x8E00, 0x6440, 0x0E20],
            PieceType::L => [0x4460, 0x0E80, 0xC440, 0x2E00],
            PieceType::I => [0x0F00, 0x2222, 0x00F0, 0x4444],
            PieceType::S => [0x06C0, 0x8C40, 0x6C00, 0x4620],
            PieceType::Z => [0x0C60, 0x4C80, 0xC600, 0x2640],
            PieceType::T => [0x0E40, 0x4C40, 0x4E00, 0x4640],
        };
        shapes[rotation as usize % 4]
    }

    pub fn get_color(&self) -> Srgba {
        match *self {
            PieceType::O => Srgba::new(0.94, 0.94, 0.0, 1.0),
            PieceType::J => Srgba::new(0.94, 0.63, 0.0, 1.0),
            PieceType::L => Srgba::new(0.0, 0.0, 0.94, 1.0),
            PieceType::I => Srgba::new(0.0, 0.94, 0.94, 1.0),
            PieceType::S => Srgba::new(0.0, 0.94, 0.0, 1.0),
            PieceType::Z => Srgba::new(0.94, 0.0, 0.0, 1.0),
            PieceType::T => Srgba::new(0.64, 0.0, 0.94, 1.0),
        }
    }
}

impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(0, 7) {
            0 => PieceType::O,
            1 => PieceType::J,
            2 => PieceType::L,
            3 => PieceType::I,
            4 => PieceType::S,
            5 => PieceType::Z,
            _ => PieceType::T,
        }
    }
}

// Dropped Piece
pub struct DroppedPiece {
    pub block_type: BlockType,
}

impl Component for DroppedPiece {
    type Storage = DenseVecStorage<Self>;
}

impl DroppedPiece {
    pub fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }
}

// Track Positions, we see them as an object not matrix
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
#[storage(VecStorage)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

