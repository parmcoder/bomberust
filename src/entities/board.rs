// the board that store the tetriminos with components

use amethyst::ecs::{Component, DenseVecStorage};

pub const BOARD_WIDTH: u32 = 10;
pub const BOARD_HEIGHT: u32 = 20;

pub struct Board {
    pub width: u32,
    pub height: u32,
    pub info: [[u32; self::BOARD_WIDTH as usize]; self::BOARD_HEIGHT as usize],
}

impl Component for Board {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Board {
    fn default() -> Self {
        Self {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            info: [[0u32; (..BOARD_WIDTH)]; (..BOARD_HEIGHT)]
        }
    }
}