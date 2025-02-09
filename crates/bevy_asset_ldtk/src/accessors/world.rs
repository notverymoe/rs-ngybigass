// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::math::IVec2;

use crate::schema as schema;

use super::{LdtkLevel, LdtkRoot};

#[derive(Debug, Clone, Copy)]
pub struct LdtkWorld<'a> {
    pub(crate) world: &'a schema::World,
    pub(crate) root:  &'a LdtkRoot<'a>,

    min: IVec2,
    max: IVec2,
}

impl <'a> LdtkWorld<'a> {

    #[must_use] 
    pub fn wrap(world: &'a schema::World, root: &'a LdtkRoot) -> Self {
        let mut min = IVec2::MAX;
        let mut max = IVec2::MIN;
        world.levels.iter().for_each(|level| {
            let world_pos  = IVec2::new(level.world_x as i32, level.world_y as i32);
            let level_size = IVec2::new(level.px_wid  as i32, level.px_hei  as i32);
            min = min.min(world_pos);
            max = max.max(world_pos+ level_size);
        });
        Self{world, root, min, max}
    }

    #[must_use]
    pub const fn get_raw(&self) -> &schema::World {
        self.world
    }
    
}

impl LdtkWorld<'_> {

    #[must_use]
    pub fn size_px(&self) -> IVec2 {
        self.max - self.min
    }

}

impl LdtkWorld<'_> {

    pub fn levels(&self) -> impl Iterator<Item = LdtkLevel<'_>> {
        self.world.levels.iter().map(|v| LdtkLevel::wrap(v, self))
    }

    #[must_use] 
    pub fn get_level(&self, i: usize) -> Option<LdtkLevel<'_>> {
        self.world.levels.get(i).map(|v| LdtkLevel::wrap(v, self))
    }

    #[must_use] 
    pub fn levels_len(&self) -> usize {
        self.world.levels.len()
    }

    #[must_use]
    pub const fn root(&self) -> &LdtkRoot {
        self.root
    }

}