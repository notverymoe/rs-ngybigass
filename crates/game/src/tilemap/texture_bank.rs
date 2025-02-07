// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::VecDeque;

use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::platform_support::collections::HashMap;
use thiserror::Error;

use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages},
};

use super::{TextureBankEntry, TextureBankIdentifier, TextureBankImportConfig};

pub const TEXTURE_BANK_MAX_SLOTS: usize = 65536;

#[derive(Debug, Error)]
pub enum TextureBankError {
    #[error("All texture slots in bank occupied.")]
    SlotsExhausted,

    #[error("Out of range. Attempt to copy from source image out of range <at: {0} size: {2}> but image is only <{1}>")]
    OutOfRangeSource(UVec2, UVec2, UVec2),

    #[error("Out of range. Attempt to index bank <{0}> out of <{1}>.")]
    OutOfRangeBank(usize, usize),

    #[error("Invalid format. Source image is formatted as <{0:?}>, expected <{1:?}>")]
    InvalidFormat(TextureFormat, TextureFormat),

    #[error("Name <{0:?}> already regitered")]
    EntryAlreadyExists(String),
}

#[derive(Component)]
pub struct TextureBank {
    bounds:     UVec2,
    free_slots: VecDeque<TextureBankIdentifier>,
    free_seq:   TextureBankIdentifier,
    lookup:     HashMap<String, TextureBankEntry>,
    images:     Box<[(Image, bool)]>,
    handles:    Box<[Handle<Image>]>,
    dirty:      bool,
    queue:      Vec<TextureBankQueue>,
}

pub struct TextureBankQueue {
    pub name:   String,
    pub src:    Handle<Image>,
    pub config: TextureBankImportConfig,
}

impl TextureBank {

    pub fn new(bounds: UVec2, max_banks: usize, assets: &mut Assets<Image>) -> Self {
        let image = create_image_bank(bounds.x, bounds.y, layer_size_bytes(bounds));
        let images = (0..max_banks)
            .map(|_| (image.clone(), false))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            bounds,
            free_seq: TextureBankIdentifier::default().next().unwrap(),
            free_slots: VecDeque::default(),
            lookup: HashMap::default(),
            handles: (0..max_banks)
                .map(|_| assets.add(image.clone()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            images,
            dirty: false,
            queue: Vec::default()
        }
    }

    #[must_use]
    pub const fn handles(&self) -> &[Handle<Image>] {
        &self.handles
    }

    #[must_use]
    pub const fn needs_sync(&self) -> bool {
        self.dirty
    }

    #[must_use]
    pub fn has_pending_queue(&self) -> bool {
        !self.queue.is_empty()
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&TextureBankEntry> {
        self.lookup.get(name)
    }

    pub fn remove(&mut self, name: &str)  {
        if let Some(entry) = self.lookup.remove(name) {
            self.free_slots.extend(entry.ids());
        }
    }

}

impl TextureBank {

    pub fn sync_images(&mut self, assets: &mut Assets<Image>) {
        for (i, (image, dirty)) in self.images.iter_mut().enumerate() {
            if core::mem::replace(dirty, false) {
                assets.insert(self.handles[i].id(), image.clone());
            }
        }
        self.dirty = false;
    }

    fn queue_defer_set(&mut self, name: String, src: Handle<Image>, config: TextureBankImportConfig) {
        let new = TextureBankQueue{name, src, config};
        if let Some(old) = self.queue.iter_mut().find(|old| new.name == old.name) {
            *old = new;
        } else {
            self.queue.push(new);
        }
    }

    pub fn process_queue(&mut self, assets: &Assets<Image>) {
        self.queue.retain(|entry| {
            if let Some(src) = assets.get(&entry.src) {
                if let Some(ids) = self.lookup.get(&entry.name) {
                    let size = ids.count();
                    for x in 0..size.x {
                        for y in 0..size.y {
                            let id = ids.get(UVec2::new(x,y)).unwrap();
                            let bounds = self.bounds;
                            let (bank, dirty) = &mut self.images[id.bank()];
                            if let Err(e) = do_set(bounds, bank, id, src, entry.config, x, y) {
                                bevy::log::warn!("Could not load texture into slot: {}", e);
                            }
                            *dirty = true;
                        }
                    }
                }
                self.dirty = true;
                false // Image existed, but load was invalid
            } else {
                true // No image loaded, try again later
            }

            
        });
    }

}

impl TextureBank {

    pub fn insert(
        &mut self, 
        name: &str, 
        src: &Handle<Image>, 
        config: TextureBankImportConfig
    ) -> Result<(), TextureBankError> {
        // Free all associated slots, prepare to replace
        if let Some(entry) = self.lookup.remove(name) {
            self.free_slots.extend(entry.ids());
        }

        let entry = if config.count == UVec2::ONE {
            TextureBankEntry::Single(self.try_reserve()?)
        } else {
            let mut ids = Vec::with_capacity((config.count.x * config.count.y) as usize);
            for _ in 0..config.count.x {
                for _ in 0..config.count.y {
                    match self.try_reserve() {
                        Ok(id) => ids.push(id),
                        Err(e) => {
                            self.free_slots.extend(ids);
                            return Err(e);
                        }
                    }
                }
            }
            TextureBankEntry::Sheet{ids: ids.into_boxed_slice(), count: config.count}
        };

        self.lookup.insert(name.to_owned(), entry);
        self.queue_defer_set(name.to_owned(), src.clone(), config); // TODO get owning handle if given weak handle
        Ok(())
    }

}

impl TextureBank {

    pub fn set(&mut self, id: TextureBankIdentifier, src: &Image, config: TextureBankImportConfig, x: u32, y: u32) -> Result<(), TextureBankError> {
        let bounds = self.bounds;
        let (bank, dirty) = self.get_bank(id.bank())?;
        let result = do_set(bounds, bank, id, src, config, x, y);
        if result.is_ok() { *dirty = true;}
        result
    }

}

impl TextureBank {

    pub fn try_reserve_name(&mut self, name: &str) -> Result<&TextureBankEntry, TextureBankError> {
        if self.lookup.contains_key(name) {
            return Err(TextureBankError::EntryAlreadyExists(name.to_owned()));
        }
        let id = self.try_reserve()?;
        self.lookup.insert(name.to_owned(), TextureBankEntry::Single(id));
        Ok(self.lookup.get(name).unwrap())
    }

    pub fn try_reserve(&mut self) -> Result<TextureBankIdentifier, TextureBankError> {
        self.free_slots.pop_front().or_else(|| {
            let free = self.free_seq;
            if let Some(next) = free.next() {
                if next.bank() < self.handles.len() {
                    self.free_seq = next;
                    return Some(free);
                }
            } 
            None
        }).ok_or(TextureBankError::SlotsExhausted)
    }
    
}

impl TextureBank {

    fn get_bank(&mut self, bank: usize) -> Result<&mut (Image, bool), TextureBankError> {
        if bank >= self.handles.len() {
            return Err(TextureBankError::OutOfRangeBank(bank, self.handles.len()));
        }

        Ok(&mut self.images[bank])
    }
} 

fn create_image_bank(width: u32, height: u32, layer_size: usize) -> Image {
    Image{
        texture_descriptor: TextureDescriptor { 
            label: Some("BANK_IMAGE"), 
            size: Extent3d{
                width,
                height,
                depth_or_array_layers: 256
            }, 
            mip_level_count: 1, 
            sample_count: 1, 
            dimension: TextureDimension::D2, 
            format: TextureFormat::Rgba8UnormSrgb, 
            usage: TextureUsages::TEXTURE_BINDING, 
            view_formats: &[],
        },
        data: vec![0; layer_size*256],
        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
            mag_filter:     ImageFilterMode::Nearest,
            min_filter:     ImageFilterMode::Nearest,
            mipmap_filter:  ImageFilterMode::Nearest,
            address_mode_u: ImageAddressMode::ClampToEdge,
            address_mode_v: ImageAddressMode::ClampToEdge,
            address_mode_w: ImageAddressMode::ClampToEdge,
            ..Default::default()
        }),
        texture_view_descriptor: None,
        asset_usage: RenderAssetUsages::RENDER_WORLD,
    }
}

fn do_set(bounds: UVec2, bank: &mut Image, id: TextureBankIdentifier, src: &Image, config: TextureBankImportConfig, x: u32, y: u32) -> Result<(), TextureBankError> {
    if src.texture_descriptor.format != TextureFormat::Rgba8UnormSrgb {
        return Err(TextureBankError::InvalidFormat(src.texture_descriptor.format, TextureFormat::Rgba8UnormSrgb));
    }

    let src_size = UVec2::new(src.width(), src.height());
    let src_from = config.offset + (bounds + config.spacing)*UVec2::new(x, y);
    let src_to   = src_from + bounds;
    if src_to.x > src_size.x || src_to.y > src_size.y {
        return Err(TextureBankError::OutOfRangeSource(src_from, src_size, bounds));
    }

    set_raw_unchecked(bounds, bank, id, &src.data, src_from, src_size);
    Ok(())
}
    
fn set_raw_unchecked(bounds: UVec2, bank: &mut Image, id: TextureBankIdentifier, src: &[u8], src_from: UVec2, src_size: UVec2)  {
    let src_x = src_from.x as usize * 4;
    let src_y = src_from.y as usize;
    let src_w = src_size.x as usize * 4;

    let dst_w = bounds.x as usize * 4;
    let dst_h = bounds.y as usize;

    copy_image_unchecked(&mut bank.data, id.slot()*dst_h, dst_w, dst_h, src, src_x, src_y, src_w);
} 

#[allow(clippy::too_many_arguments)]
fn copy_image_unchecked(
    dst: &mut [u8],
    dst_y: usize,
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
        let dst_off = dst_y*dst_w;
        let src_off = src_y*src_w;
        dst[dst_off..(dst_off+layer_size)].copy_from_slice(&src[src_off..(src_off+layer_size)]);
    } else {
        // Scanline copy
        for off_y in 0..dst_h {
            let src_off = (src_y + off_y)*src_w + src_x;
            let dst_off = (dst_y + off_y)*dst_w;
            dst[dst_off..(dst_off + dst_w)].copy_from_slice(&src[src_off..(src_off + dst_w)]);
        }
    }
}

#[must_use]
const fn layer_size_bytes(bounds: UVec2) -> usize {
    (bounds.x * bounds.y * 4) as usize
}