// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{asset::{io::Reader, AssetLoader, LoadContext}, prelude::*};
use thiserror::Error;

use crate::{schema::LdtkJson, LDTKProject};

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LDTKProjectLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// An [Serde](serde) Error
    #[error("Could not load asset: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Default)]
pub struct LDTKProjectLoader;

impl AssetLoader for LDTKProjectLoader {
    type Asset = LDTKProject;
    type Settings = ();
    type Error = LDTKProjectLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut asset_bytes = Vec::new();
        reader.read_to_end(&mut asset_bytes).await?;
        let project: LdtkJson = serde_json::from_slice(asset_bytes.as_slice())?;
        Ok(LDTKProject(project))
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}