// Copyright 2025 Natalie Baker // AGPLv3 //

use super::MultiTextureAtlasSlotID;

#[derive(Debug, Default)]
pub struct MultiTextureAtlasAllocator {
    freelist: Vec<MultiTextureAtlasSlotID>,
    head:     MultiTextureAtlasSlotID
}

impl MultiTextureAtlasAllocator {

    pub fn allocate(&mut self) -> Option<MultiTextureAtlasSlotID> {
        if self.freelist.is_empty() {
            if let Some(next) = self.head.next() {
                let result = self.head;
                self.head = next;
                Some(result)
            } else {
                None
            }
        } else {
            Some(self.freelist.remove(self.freelist.len()-1))
        }
    }
     
    pub fn deallocate(&mut self, handle: MultiTextureAtlasSlotID) {
        debug_assert!(self.freelist.iter().any(|v| *v == handle), "Double slot deallocation in MultiTextureAtlasAllocator");
        debug_assert!(self.head <= handle, "Attempt to free unallocated slot in MultiTextureAtlasAllocator");
        self.freelist.push(handle);
    }

    pub fn clear(&mut self) {
        self.freelist.clear();
        self.head = MultiTextureAtlasSlotID::default();
    }

}

