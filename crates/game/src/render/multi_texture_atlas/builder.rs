// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{asset::{Assets, RenderAssetUsages}, image::{Image, ImageSampler}, math::UVec2, render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}};

use super::{get_layer_size_bytes, MultiTextureAtlas, MultiTextureAtlasLoader};

pub struct MultiTextureAtlasBuilder {
    // The size of a tile in the atlas
    pub bounds: UVec2,
    
    /// Format of the texture.
    pub format: TextureFormat,

    /// The [`ImageSampler`] to use during rendering.
    pub sampler: ImageSampler,

    /// Mip count of texture. For a texture with no extra mips, this must be 1.
    pub mip_level_count: u32,
}

impl MultiTextureAtlasBuilder {
    #[must_use]
    pub fn new(bounds: UVec2) -> Self {
        Self { 
            bounds, 
            format:  TextureFormat::Rgba8UnormSrgb, 
            sampler: ImageSampler::nearest(), 
            mip_level_count: 1
        }
    }

    #[must_use]
    pub fn with_format(self, format: TextureFormat) -> Self {
        Self{ format, ..self }
    }

    #[must_use]
    pub fn with_sampler(self, sampler: ImageSampler) -> Self {
        Self{ sampler, ..self }
    }

    #[must_use]
    pub fn with_mip_level_count(self, mip_level_count: u32) -> Self {
        Self{ mip_level_count, ..self }
    }

    pub fn build(self, images: &mut Assets<Image>) -> MultiTextureAtlas {
        MultiTextureAtlas::new(self, images)
    }

    pub fn build_with_loader(self, images: &mut Assets<Image>) -> (MultiTextureAtlas, MultiTextureAtlasLoader) {
        (
            self.build(images),
            MultiTextureAtlasLoader::default()
        )
    }

    #[must_use]
    pub fn create_bank_image(&self) -> Image {
        let layer_size = get_layer_size_bytes(self.format, self.bounds);
        Image{
            texture_descriptor: TextureDescriptor{ 
                label: None, 
                size: Extent3d{
                    width: self.bounds.x,
                    height: self.bounds.y,
                    depth_or_array_layers: MultiTextureAtlas::SLOT_COUNT as u32
                }, 
                mip_level_count: self.mip_level_count, 
                sample_count: 1, 
                dimension: TextureDimension::D2, 
                format: self.format, 
                usage: TextureUsages::TEXTURE_BINDING, 
                view_formats: &[],
            },
            data: vec![0; layer_size*MultiTextureAtlas::SLOT_COUNT],
            sampler: self.sampler.clone(),
            texture_view_descriptor: None,
            asset_usage: RenderAssetUsages::RENDER_WORLD,
        }
    }

}
