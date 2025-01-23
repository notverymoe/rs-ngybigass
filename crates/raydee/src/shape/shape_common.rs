// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

pub trait ShapeCommon {
    fn bounding_box(&self) -> [Vec2; 2];
}