// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::math::IVec2;

use crate::schema::{self as schema, WorldLayout};

use super::{LdtkLayer, LdtkRoot, LdtkWorld};

#[derive(Debug, Clone, Copy)]
pub struct LdtkLevel<'a> {
    pub(crate) level: &'a schema::Level,
    pub(crate) world: &'a LdtkWorld<'a>,
}

impl <'a> LdtkLevel<'a> {

    #[must_use] 
    pub const fn wrap(level: &'a schema::Level, world: &'a LdtkWorld) -> Self {
        Self{level, world}
    }

    #[must_use]
    pub const fn get_raw(&self) -> &schema::Level {
        self.level
    }

}

impl LdtkLevel<'_> {

    #[must_use]
    pub const fn identifier(&self) -> &String {
        &self.level.identifier
    }

    #[must_use]
    pub const fn world_pos(&self) -> Option<IVec2> {
        match self.world.world.world_layout {
            Some(WorldLayout::GridVania | WorldLayout::Free) => Some(IVec2::new(
                self.level.world_x as i32,
                self.level.world_y as i32,
            )),
            _ => None,
        }
    }

}

impl LdtkLevel<'_> {

    pub fn layers(&self) -> impl Iterator<Item = LdtkLayer<'_>> {
        self.level.layer_instances.as_deref().unwrap_or(&[]).iter().map(|v| LdtkLayer::wrap(v, self))
    }

    #[must_use] 
    pub fn get_layer(&self, i: usize) -> Option<LdtkLayer<'_>> {
        self.level.layer_instances.as_ref().and_then(|v| v.get(i).map(|v| LdtkLayer::wrap(v, self)))
    }

    #[must_use] 
    pub fn levels_len(&self) -> usize {
        self.level.layer_instances.as_ref().map_or(0, std::vec::Vec::len)
    }

}

impl LdtkLevel<'_> {

    #[must_use]
    pub const fn root(&self) -> &LdtkRoot {
        self.world.root
    }

    #[must_use]
    pub const fn world(&self) -> &LdtkWorld {
        self.world
    }

}
