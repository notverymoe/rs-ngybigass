// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::math::UVec2;

use crate::schema as schema;

use super::LdtkRoot;

#[derive(Debug, Clone, Copy)]
pub struct LdtkTileset<'a> {
    pub(crate) tileset: &'a schema::TilesetDefinition,
    pub(crate) root:  &'a LdtkRoot<'a>,
}

impl <'a> LdtkTileset<'a> {

    #[must_use] 
    pub const fn wrap(tileset: &'a schema::TilesetDefinition, root: &'a LdtkRoot) -> Self {
        Self{tileset, root}
    }

}

impl LdtkTileset<'_> {

    #[must_use]
    pub const fn uid(&self) -> i64 {
        self.tileset.uid
    }

    #[must_use]
    pub const fn count(&self) -> UVec2 {
        UVec2::new(
            self.tileset.c_wid as u32,
            self.tileset.c_hei as u32,
        )
    }

    #[must_use]
    pub const fn padding(&self) -> UVec2 {
        UVec2::new(
            self.tileset.padding as u32,
            self.tileset.padding as u32,
        )
    }

    #[must_use]
    pub const fn spacing(&self) -> UVec2 {
        UVec2::new(
            self.tileset.spacing as u32,
            self.tileset.spacing as u32,
        )
    }

    #[must_use]
    pub fn rel_path(&self) -> Option<&str> {
        self.tileset.rel_path.as_deref()
    }

}

impl LdtkTileset<'_> {

    #[must_use] 
    pub const fn root(&self) -> &LdtkRoot {
        self.root
    }

}