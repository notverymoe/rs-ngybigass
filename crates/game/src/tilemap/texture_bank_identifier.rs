// Copyright 2023 Natalie Baker // AGPLv3 //

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureBankIdentifier(u16);

impl TextureBankIdentifier {

    pub const EMPTY: Self = Self(0);

    #[must_use]
    pub const fn new(bank: u16, slot: u16) -> Self {
        assert!(bank <=  15, "Invalid bank for TextureBankIdentifier, must be <=15");
        assert!(slot <= 255, "Invalid bank for TextureBankIdentifier, must be <=255");
        Self((bank << 8) | slot)
    }

    #[must_use]
    pub const fn to_raw(self) -> u16 {
        self.0
    }

    #[must_use]
    pub const fn from_raw(raw: u16) -> Option<Self> {
        if raw <= 0x0FFF {
            Some(Self::from_raw_unchecked(raw))
        } else {
            None
        }
    }

    #[must_use]
    const fn from_raw_unchecked(raw: u16) -> Self {
        Self(raw)
    }

}

impl TextureBankIdentifier {

    #[must_use]
    pub const fn bank(self) -> usize {
        (self.0 >> 8) as usize
    }

    #[must_use]
    pub const fn slot(self) -> usize {
        (self.0 & 0x00FF) as usize
    }

}

impl TextureBankIdentifier {

    #[must_use]
    pub fn next(self) -> Option<Self> {
        let next = self.to_raw() + 1;
        (next <= 0x0FFF).then(|| Self::from_raw_unchecked(next))
    }

}

