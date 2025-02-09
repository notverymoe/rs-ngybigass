// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{asset::{Assets, Handle}, ecs::component::Component, image::Image, math::UVec2, platform_support::collections::{hash_map::Entry, HashMap}, render::render_resource::encase::private::RuntimeSizedArray};

use super::{MultiTextureAtlas, MultiTextureAtlasAllocator, MultiTextureAtlasSlotID};

#[derive(Debug)]
pub struct Tileset {
    pub slots: Box<[MultiTextureAtlasSlotID]>,
    pub size:  UVec2,
}

#[derive(Debug)]
pub struct QueuedTileset {
    id: i64,
    handle: Handle<Image>,
    offset:  UVec2,
    spacing: UVec2,
}

#[derive(Debug, Default, Component)]
pub struct MultiTextureAtlasLoader {
    queue:    Vec<QueuedTileset>,
    tilesets: HashMap<i64, Tileset>,
    registry: MultiTextureAtlasAllocator,
}

impl MultiTextureAtlasLoader {

    #[must_use]
    pub fn process_queue_required(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn process_queue(
        &mut self, 
        atlas: &mut MultiTextureAtlas,
        images: &Assets<Image>
    ) {
        self.queue.retain(|entry| {
            let Some(tileset) = self.tilesets.get(&entry.id) else { return false; };
            let Some(image  ) = images.get(&entry.handle) else { return true; };

            for x in 0..tileset.size.x {
                for y in 0..tileset.size.y {
                    let offset = entry.offset + UVec2::new(
                        (atlas.bounds().x + entry.spacing.x) * x,
                        (atlas.bounds().y + entry.spacing.y) * y,
                    );
                    
                    if let Result::Err(e) = atlas.set(
                        tileset.slots[(x + y*tileset.size.x) as usize], 
                        image, 
                        offset.extend(0)
                    ) {
                        bevy::log::error!("Tileset subtile load encountered error, with id {}@({}, {}): {}", entry.id, x, y, e);
                    }
                }
            }
            bevy::log::info!("Tileset loaded texture: {}", entry.id);
            false
        });
    }

}

impl MultiTextureAtlasLoader {

    pub fn clear(&mut self) {
        self.queue.clear();
        self.tilesets.clear();
        self.registry.clear();
    }

    #[must_use]
    pub fn get(&self, id: i64) -> Option<&Tileset> {
        self.tilesets.get(&id)
    }

}

impl MultiTextureAtlasLoader {

    pub fn insert(
        &mut self, 
        id: i64,
        handle: Handle<Image>,
        size:    UVec2, 
        offset:  UVec2,
        spacing: UVec2
    ) -> Option<&Tileset> {

        let count = size.element_product() as usize;
        let entry = self.tilesets.entry(id);
        let mut slots = vec![MultiTextureAtlasSlotID::default(); count].into_boxed_slice();

        // Reuse slots in current entry
        let reuse_count = if let Entry::Occupied(entry) = &entry {
            let entry = entry.get();
            let count = entry.slots.len().min(slots.len());
            slots[0..count].copy_from_slice(&entry.slots[0..count]);
            count
        } else {
            0
        };

        // Allocate extra slots if required
        if reuse_count < count {
            let remaining = count - reuse_count;
            for i in 0..remaining {
                if let Some(id) = self.registry.allocate() { 
                    slots[reuse_count+i] = id;
                } else {
                    slots[reuse_count..i].iter().for_each(|id| self.registry.deallocate(*id));
                    return None;
                }
            }
        }

        // We have enough slots, dealloc the unused ones from the current entry
        if let Entry::Occupied(entry) = &entry {
            let entry = entry.get();
            for id in &entry.slots[reuse_count..] {
                self.registry.deallocate(*id);
            }
        }


        // Queue entry, check to see if there's already a request for this id and replacing it.
        let queue_entry = QueuedTileset{id, handle, offset, spacing};
        if let Some(existing) = self.queue.iter_mut().find(|v| v.id == id) {
            *existing = queue_entry;
        } else {
            self.queue.push(queue_entry);
        }

        // Update tileset and return
        let entry = entry.insert(Tileset{slots, size});
        Some(entry.into_mut()) // returns ref, not mut ref
    }

}
