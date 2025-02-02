// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{asset::{AssetServer, Assets, Handle}, image::{Image, ImageLoaderSettings, ImageSampler, TextureAtlasLayout}, platform_support::collections::HashMap};
use bevy_asset_ldtk::{schemas::latest::LdtkJson, util::ldtk_make_texture_atlas_layout};

#[derive(Debug, Default)]
pub struct Tilesets {
    pub lookup: HashMap<i64, Tileset>,
}

#[derive(Debug)]
pub struct Tileset {
    pub image:  Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub struct TilesetSprite {
    pub tileset: i64,
    pub tile_id: i64,
}


pub fn ldtk_load_tilesets(
    asset_server: &AssetServer,
    assets_texture_asset_layouts: &mut Assets<TextureAtlasLayout>,
    project: &LdtkJson,
    root_path: &str,
) -> Tilesets {
    let mut tilesets = Tilesets::default();
    let defs: &bevy_asset_ldtk::schemas::latest::Definitions = &project.defs;
    for tileset in &defs.tilesets {
        if let Some(rel_path) = &tileset.rel_path {
            tilesets.lookup.insert(
                tileset.uid,
                Tileset{
                    image: asset_server.load_with_settings::<Image, ImageLoaderSettings>(
                        [root_path, rel_path].join(""),
                        |s: &mut _| {
                            *s = ImageLoaderSettings {
                                sampler: ImageSampler::nearest(),
                                ..ImageLoaderSettings::default()
                            }
                        },
                    ),
                    layout: assets_texture_asset_layouts.add(ldtk_make_texture_atlas_layout(tileset)),
                }
            );
        }
    }
    tilesets
}