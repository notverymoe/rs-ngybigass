// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

pub mod util;

pub mod schemas;
mod loader;

#[derive(Asset, TypePath, Debug, Deref)]
pub struct LDTKProject(schemas::latest::LdtkJson);

pub struct LDTKAssetPlugin;

impl Plugin for LDTKAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<LDTKProject>()
            .init_asset_loader::<loader::LDTKProjectLoader>();
    }
}


