// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::math::IVec2;

use crate::schema as schema;

use super::LdtkLayer;

#[derive(Debug, Clone, Copy)]
pub struct LdtkEntity<'a>{
    pub(crate) entity:  &'a schema::EntityInstance,
    pub(crate) layer: &'a LdtkLayer<'a>,
}

impl<'a> LdtkEntity<'a> {

    #[must_use] 
    pub const fn wrap(entity: &'a schema::EntityInstance, layer: &'a LdtkLayer<'a>) -> Self {
        Self{entity, layer}
    }

}

impl LdtkEntity<'_> {

    #[must_use]
    pub const fn identifier(&self) -> &String {
        &self.entity.identifier
    }

    #[must_use]
    pub fn offset_px(&self) -> IVec2 {
        self.layer.offset_px() + self.offset_local_px()
    }

    #[must_use]
    pub fn offset_local_px(&self) -> IVec2 {
        IVec2::new(
            self.entity.px[0] as i32, 
            self.layer.size_px().y - (self.entity.px[1] + self.entity.height) as i32
        )
    }

    #[must_use]
    pub const fn layer(&self) -> &LdtkLayer {
        self.layer
    }

}
