// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Resource, Vec2};

mod camera;
pub use camera::*;

#[derive(Debug, Clone, Copy, Resource)]
pub struct PixelsPerUnit(pub f32);

impl From<PixelsPerUnit> for f32 {
    fn from(value: PixelsPerUnit) -> Self {
        value.0
    }
}

#[must_use]
pub fn ppu_scale_to_fit(
    pixels_per_unit:    impl Into<f32>,
    size_target_units:  Vec2,
    size_source_pixels: Vec2,
) -> Vec2 {
    let pixels_per_unit = pixels_per_unit.into();
    let target_units = size_source_pixels/pixels_per_unit;
    let scale = (target_units / size_target_units).min_element().floor();
    if scale >= 1.0 {
        size_source_pixels / (pixels_per_unit * scale)
    } else {
        size_target_units // If we can't do pixel-perfect sizing due to being too small, force it
    }
}

#[must_use]
pub fn ppu_snap_to(
    pixels_per_unit: impl Into<f32>,
    value_units: f32
) -> f32 {
    let pixels_per_unit = pixels_per_unit.into();
    (value_units*pixels_per_unit).round()/pixels_per_unit
}