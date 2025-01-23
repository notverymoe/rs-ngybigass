// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::{primitives::{Circle, Rectangle}, Vec2};
use macro_attr_2018::macro_attr;
use enum_derive_2018::EnumFromInner;

use crate::prelude::*;

macro_attr! {
    #[derive(EnumFromInner!, Debug, Copy, Clone)]
    pub enum ShapeStatic {
        Circle(Circle),
        Rectangle(Rectangle),
        BoxAlignedRound(RectangleRounded),
        BoxOriented(BoxOriented),
        BoxOrientedRound(BoxOrientedRound),
        Ramp(Ramp),
        RampRound(RampRound),
    }
}

impl ShapeCommon for ShapeStatic {
    fn bounding_box(&self) -> [Vec2; 2] {
        match self {
            ShapeStatic::Circle(s)       => s.bounding_box(),
            ShapeStatic::Rectangle(s) => s.bounding_box(),
            ShapeStatic::BoxAlignedRound(s) => s.bounding_box(),
            ShapeStatic::BoxOriented(s) => s.bounding_box(),
            ShapeStatic::BoxOrientedRound(s) => s.bounding_box(),
            ShapeStatic::Ramp(s) => s.bounding_box(),
            ShapeStatic::RampRound(s) => s.bounding_box(),
        }
    }
}

impl ShapeDebug for ShapeStatic {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        match self {
            ShapeStatic::Circle(s) => s.get_debug_shape_data(),
            ShapeStatic::Rectangle(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxAlignedRound(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxOriented(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxOrientedRound(s) => s.get_debug_shape_data(),
            ShapeStatic::Ramp(s) => s.get_debug_shape_data(),
            ShapeStatic::RampRound(s) => s.get_debug_shape_data(),
        }
    }
}

impl RayTarget for ShapeStatic {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        match self {
            ShapeStatic::Circle(s) => s.raycast(origin, ray),
            ShapeStatic::Rectangle(s) => s.raycast(origin, ray),
            ShapeStatic::BoxAlignedRound(s) => s.raycast(origin, ray),
            ShapeStatic::BoxOriented(s) => s.raycast(origin, ray),
            ShapeStatic::BoxOrientedRound(s) => s.raycast(origin, ray),
            ShapeStatic::Ramp(s) => s.raycast(origin, ray),
            ShapeStatic::RampRound(s) => s.raycast(origin, ray),
        }
    }

    fn raycast_enter(&self, origin: Vec2, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeStatic::Circle(s) => s.raycast_enter(origin, ray),
            ShapeStatic::Rectangle(s) => s.raycast_enter(origin, ray),
            ShapeStatic::BoxAlignedRound(s) => s.raycast_enter(origin, ray),
            ShapeStatic::BoxOriented(s) => s.raycast_enter(origin, ray),
            ShapeStatic::BoxOrientedRound(s) => s.raycast_enter(origin, ray),
            ShapeStatic::Ramp(s) => s.raycast_enter(origin, ray),
            ShapeStatic::RampRound(s) => s.raycast_enter(origin, ray),
        }
    }

    fn raycast_exit(&self, origin: Vec2, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeStatic::Circle(s) => s.raycast_exit(origin, ray),
            ShapeStatic::Rectangle(s) => s.raycast_exit(origin, ray),
            ShapeStatic::BoxAlignedRound(s) => s.raycast_exit(origin, ray),
            ShapeStatic::BoxOriented(s) => s.raycast_exit(origin, ray),
            ShapeStatic::BoxOrientedRound(s) => s.raycast_exit(origin, ray),
            ShapeStatic::Ramp(s) => s.raycast_exit(origin, ray),
            ShapeStatic::RampRound(s) => s.raycast_exit(origin, ray),
        }
    }
}
