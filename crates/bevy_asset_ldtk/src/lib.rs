// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

pub mod accessors;
pub mod loader;
pub mod schema;
pub mod util;

#[derive(Asset, TypePath, Debug, Deref)]
pub struct LDTKProject(schema::LdtkJson);

pub struct LDTKAssetPlugin;

impl Plugin for LDTKAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<LDTKProject>()
            .init_asset_loader::<loader::LDTKProjectLoader>();
    }
}


