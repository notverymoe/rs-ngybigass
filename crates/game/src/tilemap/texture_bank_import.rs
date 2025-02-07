// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::UVec2;

#[derive(Debug, Clone, Copy)]
pub struct TextureBankImportConfig {
    pub count:   UVec2,
    pub offset:  UVec2,
    pub spacing: UVec2,
}

impl Default for TextureBankImportConfig {
    fn default() -> Self {
        Self { 
            count:   UVec2::ONE,
            offset:  UVec2::ZERO,
            spacing: UVec2::ZERO,
        }
    }
}

impl TextureBankImportConfig {

    #[must_use]
    pub const fn new(count: UVec2) -> Self {
        Self { 
            count,
            offset:  UVec2::ZERO,
            spacing: UVec2::ZERO,
        }
    }

    #[must_use]
    pub const fn with_offset(self, offset: UVec2) -> Self {
        Self{offset, ..self}
    }

    #[must_use]
    pub const fn with_spacing(self, spacing: UVec2) -> Self {
        Self{spacing, ..self}
    }

}