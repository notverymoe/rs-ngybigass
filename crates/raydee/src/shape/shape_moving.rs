// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::{primitives::{Circle, Rectangle}, Vec2};
use macro_attr_2018::macro_attr;
use enum_derive_2018::EnumFromInner;

use crate::prelude::*;

macro_attr! {
    #[derive(EnumFromInner!, Debug, Clone, Copy)]
    pub enum ShapeMoving {
        Circle(Circle),
        Rectangle(Rectangle),
    }
}

impl ShapeMoving {

    #[must_use] 
    pub fn grown_by(self, by: f32) -> Self {
        match self {
            ShapeMoving::Circle(s)    => ShapeMoving::Circle(Circle::new(s.radius+by)),
            ShapeMoving::Rectangle(s) => ShapeMoving::Rectangle(Rectangle{half_size: s.half_size + by}),
        }
    }

    #[must_use] 
    pub fn shrunk_by(self, by: f32) -> Self {
        match self {
            ShapeMoving::Circle(s)    => ShapeMoving::Circle(Circle::new(s.radius-by)),
            ShapeMoving::Rectangle(s) => ShapeMoving::Rectangle(Rectangle{half_size: s.half_size - by}),
        }
    }

}

impl ShapeCommon for ShapeMoving {
    fn bounding_box(&self) -> [Vec2; 2] {
        match self {
            ShapeMoving::Circle(s)    => s.bounding_box(),
            ShapeMoving::Rectangle(s) => s.bounding_box(),
        }
    }
}

impl ShapeDebug for ShapeMoving {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        match self {
            ShapeMoving::Circle(s) => s.get_debug_shape_data(),
            ShapeMoving::Rectangle(s) => s.get_debug_shape_data(),
        }
    }
}

impl RayTarget for ShapeMoving {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        match self {
            ShapeMoving::Circle(s) => s.raycast(origin, ray),
            ShapeMoving::Rectangle(s) => s.raycast(origin, ray),
        }
    }

    fn raycast_enter(&self, origin: Vec2, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeMoving::Circle(s) => s.raycast_enter(origin, ray),
            ShapeMoving::Rectangle(s) => s.raycast_enter(origin, ray),
        }
    }

    fn raycast_exit(&self, origin: Vec2, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeMoving::Circle(s) => s.raycast_exit(origin, ray),
            ShapeMoving::Rectangle(s) => s.raycast_exit(origin, ray),
        }
    }
}
