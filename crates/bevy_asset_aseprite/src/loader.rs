// Copyright 2025 Natalie Baker // AGPLv3 //

use aseprite_loader::loader::{AsepriteFile, LoadImageError, LoadSpriteError};
use thiserror::Error;

use bevy::{asset::{io::Reader, AssetLoader, LoadContext}, image::ImageLoaderSettings, prelude::*, render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}};

use crate::AsepriteAsset;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AsepriteLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// An [`LoadSpriteError`](aseprite_loader::loader::LoadSpriteError) Error
    #[error("Could not load asset: {0}")]
    LoadSpriteError(#[from] LoadSpriteError),

    /// An [`LoadImageError`](aseprite_loader::loader::LoadImageError) Error
    #[error("Could not load asset: {0}")]
    LoadImageError(#[from] LoadImageError),
}

#[derive(Default)]
pub struct AsepriteAssetLoader;

impl AssetLoader for AsepriteAssetLoader {
    type Asset = AsepriteAsset;
    type Settings = ImageLoaderSettings;
    type Error = AsepriteLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &ImageLoaderSettings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut asset_bytes = Vec::new();
        reader.read_to_end(&mut asset_bytes).await?;

        let loader = AsepriteFile::load(asset_bytes.as_slice())?;

        let target_size = usize::from(loader.file.header.width) * usize::from(loader.file.header.height) * 4;
        let frame_count = loader.frames.len();
        let mut data = vec![0_u8; target_size*frame_count];
        loader.combined_frame_image(0, data.as_mut())?;

        for frame in 0..frame_count {
            loader.combined_frame_image(frame, &mut data[frame*target_size..])?;
        }

        let image = load_aseprite_frames(&loader, settings)?;
        let image = load_context.add_labeled_asset::<Image>("frames".to_owned(), image);

        Ok(AsepriteAsset{
            tags: loader.tags,
            image,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["aseprite"]
    }
}

#[derive(Default)]
pub struct AsepriteImageLoader;

impl AssetLoader for AsepriteImageLoader {
    type Asset = Image;
    type Settings = ImageLoaderSettings;
    type Error = AsepriteLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &ImageLoaderSettings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut asset_bytes = Vec::new();
        reader.read_to_end(&mut asset_bytes).await?;

        let loader = AsepriteFile::load(asset_bytes.as_slice())?;
        load_aseprite_frames(&loader, settings)
    }

    fn extensions(&self) -> &[&str] {
        &["aseprite"]
    }
}



fn load_aseprite_frames(loader: &AsepriteFile, settings: &ImageLoaderSettings) -> Result<Image, AsepriteLoaderError> {
    let target_size = usize::from(loader.file.header.width) * usize::from(loader.file.header.height) * 4;
    let frame_count = loader.frames.len();
    let mut data = vec![0_u8; target_size*frame_count];
    loader.combined_frame_image(0, data.as_mut())?;

    for frame in 0..frame_count {
        loader.combined_frame_image(frame, &mut data[frame*target_size..])?;
    }

    Ok(Image{
        data: Some(data),
        sampler: settings.sampler.clone(),
        texture_view_descriptor: None,
        asset_usage: settings.asset_usage,
        texture_descriptor: TextureDescriptor {
            size: Extent3d{
                width:  loader.file.header.width  as u32,
                height: loader.file.header.height as u32,
                depth_or_array_layers: frame_count as u32,
            },
            format: TextureFormat::Rgba8UnormSrgb,
            dimension: TextureDimension::D2,
            label: None,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        },
    })
}