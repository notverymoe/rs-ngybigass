// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{asset::load_internal_asset, prelude::*, sprite::Material2dPlugin};

mod material;
pub use material::*;

use crate::render::{mta_sync, MultiTextureAtlas};

pub struct PluginTilemapMaterial;

impl Plugin for PluginTilemapMaterial {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, HANDLE_SHADER_TEXTURE_BANK, "material.wgsl", Shader::from_wgsl);
        app
            .init_asset::<TilemapMaterial>()
            .add_plugins(Material2dPlugin::<TilemapMaterial>::default())
            .add_systems(PostUpdate, tilemap_toucher.after(mta_sync));
    }
}

#[derive(Debug, Clone, Component)]
pub struct TilemapMaterialSync(Entity, Handle<TilemapMaterial>);

impl TilemapMaterialSync {
    #[must_use]
    pub fn new(tilemap_entity: Entity, handle: &Handle<TilemapMaterial>) -> Self {
        Self(tilemap_entity, handle.clone_weak())
    }
}

pub fn tilemap_toucher(
    q_mta: Query<(Entity, &MultiTextureAtlas), Changed<MultiTextureAtlas>>,
    q_sync: Query<&TilemapMaterialSync>,
    mut materials: ResMut<Assets<TilemapMaterial>>,
) {
    q_mta.iter().for_each(|(src, atlas)| {
        q_sync.iter()
            .filter(|TilemapMaterialSync(dst, _)| src == *dst)
            .for_each(|TilemapMaterialSync(_, hdl)| { 
                if let Some(v) = materials.get_mut(hdl) { // This forces the textures to update, as the material is marked dirty when get_mut is called
                    // This syncs the texture atlas. Makes it easier to spawn if this is done lazily.
                    if v.get_multi_texture_atlas().is_none() {
                        v.set_multi_texture_atlas(Some(atlas.handles().cloned().collect::<Vec<_>>().into_boxed_slice()));
                    }
                }
            });
    });
}
