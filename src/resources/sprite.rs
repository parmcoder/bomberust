// use amethyst::{assets::Handle, ecs::prelude::World, renderer::SpriteSheet};
//
// #[derive(Clone)]
// pub struct SpriteResource {
//     pub tetriminos: Handle<SpriteSheet>,
// }
//
// pub fn initialize_sprite_resource(
//     world: &mut World,
//     tetriminos_handle: Handle<SpriteSheet>,
// ) -> SpriteResource {
//     let sprite_resource = SpriteResource {
//         tetriminos: tetriminos_handle,
//     };
//
//     world.insert(sprite_resource.clone());
//     sprite_resource
// }
