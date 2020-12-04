use crate::entities::{Piece, Position};

use amethyst::assets::Handle;
use amethyst::core::ecs::{Component, DenseVecStorage, Entities, ReadExpect};
use amethyst::core::Transform;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use amethyst::{
    core::math::{Point2, Point3, Vector3},
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
    renderer::debug_drawing::DebugLinesComponent,
};

pub struct PieceImage;

impl Component for PieceImage {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct RenderSystem;

impl RenderSystem {
    fn draw_crossed_square(
        &self,
        debug_line: &mut DebugLinesComponent,
        position: &Position,
        color: Srgba,
    ) {
        debug_line.add_rectangle_2d(
            Point2::new(position.col as f32, position.row as f32),
            Point2::new((position.col + 1) as f32, (position.row + 1) as f32),
            0.0,
            color,
        );
        debug_line.add_line(
            Point3::new(position.col as f32, position.row as f32, 0.0),
            Point3::new((position.col + 1) as f32, (position.row + 1) as f32, 0.0),
            color,
        );
        debug_line.add_line(
            Point3::new(position.col as f32, (position.row + 1) as f32, 0.0),
            Point3::new((position.col + 1) as f32, position.row as f32, 0.0),
            color,
        );
    }
}

impl<'s> System<'s> for RenderSystem {
    type SystemData = (
        ReadStorage<'s, Piece>,
        WriteStorage<'s, PieceImage>,
        ReadStorage<'s, Position>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteStorage<'s, Tint>,
    );

    fn run(
        &mut self,
        (
            pieces,
            mut pieces_placed,
            positions,
            mut transforms,
            entities,
            mut sprite_renders,
            sprite_sheet_handle,
            mut tints,
        ): Self::SystemData,
    ) {
        for (_, entity) in (&mut pieces_placed, &*entities).join() {
            entities.delete(entity).unwrap();
        }

        for (piece, position) in (&pieces, &positions).join() {
            for self_pos in piece.get_filled_positions(position) {
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: 0,
                };

                let mut sprite_transform = Transform::default();
                sprite_transform.set_scale(Vector3::new(0.065, 0.065, 1.0));
                sprite_transform.set_translation_xyz(
                    self_pos.col as f32 + 0.5,
                    self_pos.row as f32 + 0.5,
                    0.0,
                );

                let tint = Tint(piece.piece_type.get_color());

                entities
                    .build_entity()
                    .with(PieceImage {}, &mut pieces_placed)
                    .with(sprite_render, &mut sprite_renders)
                    .with(sprite_transform, &mut transforms)
                    .with(tint, &mut tints)
                    .build();
            }
        }
    }
}
