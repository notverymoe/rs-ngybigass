// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::UVec2;

use super::TextureBankIdentifier;

pub enum TextureBankEntry {
    Single(TextureBankIdentifier), 
    Sheet{
        ids: Box<[TextureBankIdentifier]>,
        count: UVec2,
    }
}

impl TextureBankEntry {

    #[must_use]
    pub const fn ids(&self) -> &[TextureBankIdentifier] {
        match self {
            TextureBankEntry::Single(id) => core::slice::from_ref(id),
            TextureBankEntry::Sheet { ids, count: _ } => ids,
        }
    } 

    #[must_use]
    pub const fn single(&self) -> TextureBankIdentifier {
        match self {
            TextureBankEntry::Single(id) => *id,
            TextureBankEntry::Sheet { ids, count: _ } => ids[0],
        }
    }

    #[must_use]
    pub const fn get(&self, at: UVec2) -> Option<TextureBankIdentifier> {
        match self {
            TextureBankEntry::Single(id) => if at.x < 1 && at.y < 1 { Some(*id) } else { None },
            TextureBankEntry::Sheet { ids, count } => if at.x < count.x && at.y < count.y { 
                let offset = at.y*count.x + at.x;
                Some((*ids)[offset as usize]) 
            } else { 
                None 
            },
        }
    }

    #[must_use]
    pub const fn count(&self) -> UVec2 {
        match self {
            TextureBankEntry::Single(_) => UVec2::ONE,
            TextureBankEntry::Sheet { ids: _, count } => *count,
        }
    }

}
