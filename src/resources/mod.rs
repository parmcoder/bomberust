use crate::components::{
};
use serde::{Deserialize, Serialize};

mod sprite;

pub use self::sprite::{initialize_sprite_resource, SpriteResource};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TetriminosEntityData {
    pub animation_component: Animation,
    pub motion2d_component: Motion2DComponent,
}
