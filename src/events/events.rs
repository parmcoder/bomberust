use amethyst::{audio::SourceHandle, core::math::Vector2, ecs::prelude::Entity};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl CollisionEvent {
    pub fn new(entity_a: Entity, entity_b: Entity) -> CollisionEvent {
        CollisionEvent { entity_a, entity_b }
    }
}

#[derive(Debug)]
pub struct PlayAudioEvent {
    pub source: SourceHandle,
}
