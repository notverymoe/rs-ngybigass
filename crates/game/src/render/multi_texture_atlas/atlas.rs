use bevy::{asset::{Assets, Handle}, image::{Image, TextureFormatPixelInfo}, math::{UVec2, UVec3}, prelude::Component, render::render_resource::TextureFormat};
use thiserror::Error;

use super::{MultiTextureAtlasBuilder, MultiTextureAtlasSlotID};

#[derive(Debug, Error)]
pub enum MultiTextureAtlasSetError {
    #[error("Out of range. Attempt to copy from source image out of range <at: {0} size: {2}> but image is only <{1}>")]
    OutOfRangeSource(UVec3, UVec3, UVec2),

    #[error("Invalid format. Source image is formatted as <{0:?}>, expected <{1:?}>")]
    InvalidFormat(TextureFormat, TextureFormat),
}

#[derive(Debug, Component)]
pub struct MultiTextureAtlas {
    bounds: UVec2,
    format: TextureFormat,
    banks: Box<[TextureBank]>
}

impl MultiTextureAtlas {

    pub const BANK_COUNT: usize = 16;
    pub const SLOT_COUNT: usize = 256;

    pub fn new(
        options: MultiTextureAtlasBuilder, 
        assets: &mut Assets<Image>
    ) -> Self {
        let bank_image = options.create_bank_image();
        let mut banks = Vec::with_capacity(Self::BANK_COUNT);
        for _ in 0..Self::BANK_COUNT { banks.push(TextureBank::new(bank_image.clone(), assets)); }
        Self {
            bounds: options.bounds, 
            format: options.format,
            banks: banks.into_boxed_slice()
        }
    }

    #[must_use]
    pub const fn bounds(&self) -> UVec2 {
        self.bounds
    }

    pub fn handles(&self) -> impl Iterator<Item = &Handle<Image>> {
        self.banks.iter().map(|v| &v.handle)
    }

    pub fn sync(&mut self, assets: &mut Assets<Image>) {
        for bank in &mut self.banks {
            if core::mem::replace(&mut bank.dirty, false) {
                assets.insert(bank.handle.id(), bank.data.clone());
            }
        }
    }

    #[must_use]
    pub fn sync_required(&self) -> bool {
        self.banks.iter().any(|v| v.dirty)
    }

}

impl MultiTextureAtlas {

    pub fn set(&mut self, id: MultiTextureAtlasSlotID, src: &Image, src_offset: UVec3) -> Result<(), MultiTextureAtlasSetError> {
        self.set_check(src, src_offset)?;

        let pixel_size = self.format.pixel_size();

        let src_x = src_offset.x as usize * pixel_size;
        let src_y = src_offset.y as usize;
        let src_w = src.texture_descriptor.size.width as usize * pixel_size;
        let src_layer_size = get_layer_size_bytes(
            self.format, 
            UVec2::new(src.texture_descriptor.size.width, src.texture_descriptor.size.height)
        );

        let dst_w = self.bounds.x as usize * pixel_size;
        let dst_h = self.bounds.y as usize;
        let dst_layer_size = get_layer_size_bytes(self.format, self.bounds);

        let bank = self.banks.get_mut(id.bank).unwrap();
    
        copy_image_unchecked(
            &mut bank.data.data.as_mut().unwrap()[dst_layer_size*id.slot..], 
            dst_w, dst_h, 
            &src.data.as_ref().unwrap()[src_layer_size*(src_offset.z as usize)..],
            src_x, src_y, src_w
        );

        bank.dirty = true;

        Ok(())
    }

    pub fn set_check(&self, src: &Image, src_offset: UVec3) -> Result<(), MultiTextureAtlasSetError> {
        if src.texture_descriptor.format != self.format {
            return Err(MultiTextureAtlasSetError::InvalidFormat(src.texture_descriptor.format, self.format));
        }

        if src_offset.x+self.bounds.x > src.texture_descriptor.size.width ||
           src_offset.y+self.bounds.y > src.texture_descriptor.size.height ||
           src_offset.z >= src.texture_descriptor.size.depth_or_array_layers {
            return Err(MultiTextureAtlasSetError::OutOfRangeSource(
                src_offset, 
                UVec3::new(
                    src.texture_descriptor.size.width,
                    src.texture_descriptor.size.height,
                    src.texture_descriptor.size.depth_or_array_layers,
                ),
                self.bounds
            ));
        }

        Ok(())
    }

}

#[derive(Debug)]
struct TextureBank {
    dirty:  bool,
    handle: Handle<Image>,
    data:   Image,
}

impl TextureBank {

    pub fn new(data: Image, assets: &mut Assets<Image>) -> Self {
        Self{
            dirty: false,
            handle: assets.add(data.clone()),
            data,
        }
    }

}

#[must_use] 
pub fn get_layer_size_bytes(format: TextureFormat, bounds: UVec2) -> usize {
    format.pixel_size()*(bounds.x as usize)*(bounds.y as usize)
}

#[allow(clippy::too_many_arguments)]
pub fn copy_image_unchecked(
    dst: &mut [u8],
    dst_w: usize,
    dst_h: usize,

    src: &[u8],
    src_x: usize,
    src_y: usize,
    src_w: usize,
) {
    if src_x == 0 && src_w == dst_w {
        // Single copy
        let layer_size = dst_h*dst_w;
        let src_off = src_y*src_w;
        dst[0..layer_size].copy_from_slice(&src[src_off..(src_off+layer_size)]);
    } else {
        // Scanline copy
        for off_y in 0..dst_h {
            let src_off = (src_y + off_y)*src_w + src_x;
            let dst_off = off_y*dst_w;
            dst[dst_off..(dst_off + dst_w)].copy_from_slice(&src[src_off..(src_off + dst_w)]);
        }
    }
}
