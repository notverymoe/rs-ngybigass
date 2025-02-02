// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{ecs::system::{Commands, EntityCommands}, math::{primitives::Rectangle, Vec2}, transform::components::Transform};
use bevy_asset_ldtk::{schemas::latest::{TileInstance, LayerInstance}, util::ldtk_resolve_layer_position};

mod sprite;
pub use sprite::*;

mod tileset;
pub use tileset::*;

mod player;
pub use player::*;

use crate::collision::CollisionMap;

pub fn spawn_ldtk_tile<'a>(
    commands: &'a mut Commands,
    layer: &LayerInstance,
    tile: &TileInstance,
    tileset: &Tileset,
    collision_map: &mut CollisionMap,
) -> EntityCommands<'a> {
    let position = ldtk_resolve_layer_position(layer, &tile.px);

    collision_map.get_mut(0).insert(
        position,
        Rectangle::from_size(Vec2::ONE),
        None
    );

    commands.spawn((
        ldtk_tile_sprite(tile, tileset),
        Transform::from_translation(position.extend(0.0))
    ))
}