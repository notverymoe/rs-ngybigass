// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::math::IVec2;

use crate::schema as schema;

use super::{LdtkEntity, LdtkLevel, LdtkRoot, LdtkTile, LdtkWorld};

#[derive(Debug, Clone, Copy)]
pub struct LdtkLayer<'a>{
    pub(crate) layer:  &'a schema::LayerInstance,
    pub(crate) level: &'a LdtkLevel<'a>,
}

impl<'a> LdtkLayer<'a> {

    #[must_use] 
    pub const fn wrap(layer: &'a schema::LayerInstance, level: &'a LdtkLevel<'a>) -> Self {
        Self{layer, level}
    }

    #[must_use]
    pub const fn get_raw(&self) -> &schema::LayerInstance {
        self.layer
    }

}

impl LdtkLayer<'_> {

    #[must_use]
    pub const fn identifier(&self) -> &String {
        &self.layer.identifier
    }

    #[must_use]
    pub const fn tileset_uid(&self) -> Option<i64> {
        // TODO override_tileset_uid
        self.layer.tileset_def_uid
    }

    #[must_use]
    pub const fn size_grid(&self) -> i32 {
        self.layer.grid_size as i32
    }

    #[must_use]
    pub fn offset_px(&self) -> IVec2 {
        let pos = self.level.world_pos().unwrap_or_default();

        let origin = IVec2::new(
            self.layer.px_total_offset_x as i32,
            self.layer.px_total_offset_y as i32 + self.size_px().y,
        );

        IVec2::new(
            pos.x + origin.x,
            -(pos.y + origin.y),
        )
    }

    #[must_use]
    pub const fn size_px(&self) -> IVec2 {
        IVec2::new(
            (self.layer.c_wid * self.layer.grid_size) as i32,
            (self.layer.c_hei * self.layer.grid_size) as i32,
        )
    }

}

impl LdtkLayer<'_> {

    pub fn entities(&self) -> impl Iterator<Item = LdtkEntity<'_>> {
        self.layer.entity_instances.iter().map(|v| LdtkEntity::wrap(v, self))
    }

    #[must_use] 
    pub fn get_entity(&self, i: usize) -> Option<LdtkEntity<'_>> {
        self.layer.entity_instances.get(i).map(|v| LdtkEntity::wrap(v, self))
    }

    #[must_use] 
    pub fn entities_len(&self) -> usize {
        self.layer.entity_instances.len()
    }

}

impl LdtkLayer<'_> {

    pub fn grid_tiles(&self) -> impl Iterator<Item = LdtkTile<'_>> {
        self.layer.grid_tiles.iter().map(|v| LdtkTile::wrap(v, self))
    }

    #[must_use] 
    pub fn get_grid_tile(&self, i: usize) -> Option<LdtkTile<'_>> {
        self.layer.grid_tiles.get(i).map(|v| LdtkTile::wrap(v, self))
    }

    #[must_use] 
    pub fn grid_tiles_len(&self) -> usize {
        self.layer.grid_tiles.len()
    }

}

impl LdtkLayer<'_> {

    pub fn auto_layer_tiles(&self) -> impl Iterator<Item = LdtkTile<'_>> {
        self.layer.auto_layer_tiles.iter().map(|v| LdtkTile::wrap(v, self))
    }

    #[must_use] 
    pub fn get_auto_layer_tile(&self, i: usize) -> Option<LdtkTile<'_>> {
        self.layer.auto_layer_tiles.get(i).map(|v| LdtkTile::wrap(v, self))
    }

    #[must_use] 
    pub fn auto_layer_tiles_len(&self) -> usize {
        self.layer.auto_layer_tiles.len()
    }

}

impl LdtkLayer<'_> {

    #[must_use]
    pub const fn root(&self) -> &LdtkRoot {
        self.level.world.root
    }

    #[must_use]
    pub const fn world(&self) -> &LdtkWorld {
        self.level.world
    }

    #[must_use]
    pub const fn level(&self) -> &LdtkLevel {
        self.level
    }

}

