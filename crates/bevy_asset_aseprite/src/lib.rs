// Copyright 2025 Natalie Baker // AGPLv3 //

use aseprite_loader::loader::Tag;

use bevy::prelude::*;

mod loader;

#[derive(Asset, TypePath, Debug)]
pub struct AsepriteAsset {
    pub image: Handle<Image>,
    pub tags:  Vec<Tag>,
}

pub struct AsepriteAssetPlugin;

impl Plugin for AsepriteAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<AsepriteAsset>()
            .init_asset_loader::<loader::AsepriteAssetLoader>()
            .init_asset_loader::<loader::AsepriteImageLoader>();
    }
}