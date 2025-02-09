// Copyright 2025 Natalie Baker // AGPLv3 //

use crate::schema as schema;

use super::{LdtkTileset, LdtkWorld};

#[derive(Debug, Clone, Copy)]
pub struct LdtkRoot<'a>(&'a schema::LdtkJson);

impl <'a> LdtkRoot<'a> {
    #[must_use] 
    pub const fn wrap(v: &'a schema::LdtkJson) -> Self {
        Self(v)
    }
}

impl LdtkRoot<'_> {

    pub fn worlds(&self) -> impl Iterator<Item = LdtkWorld<'_>> {
        self.0.worlds.iter().map(|v| LdtkWorld::wrap(v, self))
    }

    #[must_use] 
    pub fn get_world(&self, i: usize) -> Option<LdtkWorld<'_>> {
        self.0.worlds.get(i).map(|v| LdtkWorld::wrap(v, self))
    }

    #[must_use] 
    pub fn worlds_len(&self) -> usize {
        self.0.worlds.len()
    }

}

impl LdtkRoot<'_> {

    pub fn tilesets(&self) -> impl Iterator<Item = LdtkTileset<'_>> {
        self.0.defs.tilesets.iter().map(|v| LdtkTileset::wrap(v, self))
    }

    #[must_use] 
    pub fn get_tileset(&self, i: usize) -> Option<LdtkTileset<'_>> {
        self.0.defs.tilesets.get(i).map(|v| LdtkTileset::wrap(v, self))
    }

    #[must_use] 
    pub fn tilesets_len(&self) -> usize {
        self.0.defs.tilesets.len()
    }

}