// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::schemas::latest::{LayerInstance, TilesetDefinition};

#[must_use]
pub fn ldtk_make_texture_atlas_layout(tileset: &TilesetDefinition) -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        UVec2::ONE * tileset.tile_grid_size as u32,
        tileset.c_wid as u32,
        tileset.c_hei as u32, 
        (tileset.spacing != 0).then(|| UVec2::new(tileset.spacing as u32, tileset.spacing as u32) ),
        (tileset.padding != 0).then(|| UVec2::new(tileset.padding as u32, tileset.padding as u32) ),
    )
}

#[must_use]
pub fn ldtk_resolve_layer_position(layer: &LayerInstance, object_px: &[i64]) -> Vec2 {
    ldtk_resolve_position(layer.px_total_offset_x, layer.px_total_offset_y, object_px, layer.grid_size as f32)
}

#[must_use]
pub fn ldtk_resolve_position(offset_x: i64, offset_y: i64, object_px: &[i64], ppu: f32) -> Vec2 {
    Vec2::new(
        ((offset_x + object_px[0]) as f32)/ppu, 
       -((offset_y + object_px[1]) as f32)/ppu
   )
}