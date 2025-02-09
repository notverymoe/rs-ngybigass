// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::math::IVec2;

use crate::schema as schema;

use super::{LdtkLayer, LdtkLevel, LdtkRoot, LdtkWorld};

#[derive(Debug, Clone, Copy)]
pub struct LdtkTile<'a>{
    pub(crate) tile:  &'a schema::TileInstance,
    pub(crate) layer: &'a LdtkLayer<'a>,
}

impl <'a> LdtkTile<'a> {

    #[must_use] 
    pub const fn wrap(tile: &'a schema::TileInstance, layer: &'a LdtkLayer<'a>) -> Self {
        Self{tile, layer}
    }

}

impl LdtkTile<'_> {

    #[must_use]
    pub const fn tileset_index(&self) -> usize {
        self.tile.t as usize
    }

    #[must_use]
    pub const fn flip_x(&self) -> bool{
        self.tile.f & 0x01 != 0
    }

    #[must_use]
    pub const fn flip_y(&self) -> bool{
        #[allow(clippy::nonminimal_bool)]
        !(self.tile.f & 0x02 != 0)
    }

    #[must_use]
    pub fn offset_px(&self) -> IVec2 {
        self.layer.offset_px() + self.offset_local_px()
    }

    #[must_use]
    pub fn offset_local_px(&self) -> IVec2 {
        IVec2::new(
            self.tile.px[0] as i32, 
            self.layer.size_px().y - self.tile.px[1] as i32 - self.layer.layer.grid_size as i32
        )
    }

}

impl LdtkTile<'_> {

    #[must_use]
    pub const fn root(&self) -> &LdtkRoot {
        self.layer.level.world.root
    }

    #[must_use]
    pub const fn world(&self) -> &LdtkWorld {
        self.layer.level.world
    }

    #[must_use]
    pub const fn level(&self) -> &LdtkLevel {
        self.layer.level
    }

    #[must_use]
    pub const fn layer(&self) -> &LdtkLayer {
        self.layer
    }

}