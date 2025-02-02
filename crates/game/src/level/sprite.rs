// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{image::TextureAtlas, math::Vec2, sprite::Sprite};
use bevy_asset_ldtk::schemas::latest::TileInstance;

use super::tileset::Tileset;

#[must_use]
pub fn ldtk_tile_sprite(
    tile: &TileInstance,
    tileset: &Tileset,
) -> Sprite {
    Sprite{
        custom_size: Some(Vec2::ONE),
        flip_x: tile.f & 0x01 == 0x01,
        flip_y: tile.f & 0x02 == 0x02,
        ..Sprite::from_atlas_image(
            tileset.image.clone(), 
            TextureAtlas{
                index: tile.t as usize,
                layout: tileset.layout.clone()
            }
        )
    }
}