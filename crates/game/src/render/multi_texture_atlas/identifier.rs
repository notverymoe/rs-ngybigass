// Copyright 2025 Natalie Baker // AGPLv3 //

use core::cmp::Ordering;

use super::MultiTextureAtlas;

#[derive(Debug, Default, Clone, Copy)]
pub struct MultiTextureAtlasSlotID {
    pub bank: usize,
    pub slot: usize,
}

impl Eq for MultiTextureAtlasSlotID {}

impl PartialEq for MultiTextureAtlasSlotID {
    fn eq(&self, other: &Self) -> bool {
        self.bank == other.bank && self.slot == other.slot
    }
}

impl Ord for MultiTextureAtlasSlotID {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO PAINFUL
        match (self.bank.cmp(&other.bank), self.slot.cmp(&other.slot)) {
            (Ordering::Less,    Ordering::Less   ) => Ordering::Less,
            (Ordering::Less,    Ordering::Equal  ) => Ordering::Less,
            (Ordering::Less,    Ordering::Greater) => Ordering::Less,
            (Ordering::Equal,   Ordering::Less   ) => Ordering::Less,
            (Ordering::Equal,   Ordering::Equal  ) => Ordering::Equal,
            (Ordering::Equal,   Ordering::Greater) => Ordering::Greater,
            (Ordering::Greater, Ordering::Less   ) => Ordering::Greater,
            (Ordering::Greater, Ordering::Equal  ) => Ordering::Greater,
            (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
        }
    }
}

impl PartialOrd for MultiTextureAtlasSlotID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl MultiTextureAtlasSlotID {

    #[must_use]
    pub fn new(bank: usize, slot: usize) -> Option<Self> {
        (bank < MultiTextureAtlas::BANK_COUNT && slot < MultiTextureAtlas::SLOT_COUNT)
            .then_some(Self{bank, slot})
    }

    #[must_use]
    pub fn next(self) -> Option<Self> {
        let slot = self.slot + 1;
        if slot >= MultiTextureAtlas::SLOT_COUNT { 
            (self.bank < MultiTextureAtlas::BANK_COUNT).then_some(Self{bank: self.bank + 1, slot: 0})
        } else { 
            Some(Self{bank: self.bank, slot}) 
        }
    }

}
