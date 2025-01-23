// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Entity, Resource};

mod identifiers;
pub use identifiers::*;

mod layer;
pub use layer::*;

use raydee::prelude::ShapeStatic;

#[derive(Debug, Clone, Copy)]
pub struct CollisionMapEntry {
    pub identifier: CollisionMapLayerID,
    pub entity:     Option<Entity>,
    pub collider:   ShapeStatic,
    pub origin:     Vec2,
}

#[derive(Debug, Default, Resource)]
pub struct CollisionMap {
    layers: [CollisionMapLayer; 16],
}

impl CollisionMap {

    pub fn clear(&mut self) {
        self.layers.iter_mut().for_each(CollisionMapLayer::clear);
    }

    pub fn iter(&self) -> impl Iterator<Item = &CollisionMapLayer> {
        self.layers.iter()
    }

    #[must_use]
    pub fn get(&self, layer: usize) -> &CollisionMapLayer {
        self.try_get(layer).unwrap()
    }

    #[must_use]
    pub fn get_mut(&mut self, layer: usize) -> &mut CollisionMapLayer {
        self.try_get_mut(layer).unwrap()
    }

    #[must_use]
    pub fn try_get(&self, layer: usize) -> Option<&CollisionMapLayer> {
        (layer < self.layers.len()).then(|| &self.layers[layer])
    }

    #[must_use]
    pub fn try_get_mut(&mut self, layer: usize) -> Option<&mut CollisionMapLayer> {
        (layer < self.layers.len()).then(|| &mut self.layers[layer])
    }

}