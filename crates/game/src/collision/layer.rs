// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{math::{IVec2, Vec2}, prelude::Entity, platform_support::collections::hash_map::HashMap};
use raydee::prelude::*;

use super::{CollisionMapEntry, CollisionMapLayerID};

#[derive(Debug, Default)]
pub struct CollisionMapLayer {
    identifier_next: CollisionMapLayerID,
    ranges: HashMap<CollisionMapLayerID, ([IVec2; 2], CollisionMapEntry)>,
    entries: HashMap<IVec2, Vec<CollisionMapEntry>>,
}

impl CollisionMapLayer {

    const CHUNK_SHR: u32 = 3;

    pub fn clear(&mut self) {
        self.ranges.clear();
        self.entries.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &CollisionMapEntry> {
        self.ranges.values().map(|v| &v.1)
    }

    pub fn visit(&self, start: Vec2, end: Vec2, mut process: impl FnMut(&[CollisionMapEntry])) {
        let bounds = [
            Self::calculate_chunk(start),
            Self::calculate_chunk(end  ),
        ];

        for x in bounds[0].x..=bounds[1].x {
            for y in bounds[0].y..=bounds[1].y {
                if let Some(entries) = self.entries.get(&IVec2::new(x, y))  {
                    process(entries);
                }
            }
        }
    }

    pub fn insert(&mut self, origin: Vec2, collider: impl Into<ShapeStatic>, entity: Option<Entity>) -> CollisionMapLayerID {
        let collider = collider.into();
        let identifier = self.identifier_next;
        self.identifier_next = identifier.next().expect("Exhausted Collision Map IDs");

        let entry = CollisionMapEntry{
            identifier,
            entity,
            collider,
            origin,
        };

        let bounds = Self::calculate_bounds(origin, collider.bounding_box());
        self.ranges.insert(identifier, (bounds, entry));


        for x in bounds[0].x..=bounds[1].x {
            for y in bounds[0].y..=bounds[1].y {
                let idx = IVec2::new(x, y);
                self.entries.entry(idx).or_default().push(entry);
            }
        }

        identifier
    }

    pub fn remove(&mut self, identifier: CollisionMapLayerID) -> bool {
        if let Some((bounds, _)) = self.ranges.remove(&identifier) {
            for x in bounds[0].x..=bounds[1].x {
                for y in bounds[0].y..=bounds[1].y {
                    let idx = IVec2::new(x, y);
                    let entry = self.entries.get_mut(&idx).unwrap();
                    let idx = entry.iter().enumerate().find_map(|(i, v)| (v.identifier == identifier).then_some(i)).unwrap();
                    entry.swap_remove(idx);
                }
            }
            true
        } else {
            false
        }
    }
 
    #[must_use]
    pub fn calculate_chunk(point: Vec2) ->IVec2 {
        point.as_ivec2().map(|v| v >> Self::CHUNK_SHR)
    }
 
    #[must_use]
    pub fn calculate_bounds(origin: Vec2, bounds: [Vec2; 2]) -> [IVec2; 2] {
        [
            Self::calculate_chunk(origin + bounds[0]),
            Self::calculate_chunk(origin + bounds[1]),
        ]
    }

}