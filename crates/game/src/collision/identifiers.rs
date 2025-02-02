// Copyright 2025 Natalie Baker // AGPLv3 //

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollisionMapLayerID(u64);

impl CollisionMapLayerID {

    pub const DEFAULT: Self = Self(0);

    #[must_use]
    pub fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }

}