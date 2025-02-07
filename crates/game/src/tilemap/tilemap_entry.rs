// Copyright 2025 Natalie Baker // AGPLv3 //

#![allow(clippy::all, clippy::pedantic, dead_code)]

use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug, Default, Clone, Copy)]
pub struct TilemapEntry {
    texture_id: B14,
    flip_x:     bool,
    flip_y:     bool,
}
