// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{math::primitives::Rectangle, prelude::Vec2};

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, ShapeCommon};

impl ShapeCommon for Rectangle {
    fn bounding_box(&self) -> [Vec2; 2] {
        [-self.half_size, self.half_size]
    }
}

impl RayTarget for Rectangle {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_rect(origin, self.half_size)
    }
}

impl ShapeDebug for Rectangle {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        ShapeDebugData::polygon( 
            Box::new([
                Vec2::new( self.half_size.x,  self.half_size.y),
                Vec2::new(-self.half_size.x,  self.half_size.y),
                Vec2::new(-self.half_size.x, -self.half_size.y),
                Vec2::new( self.half_size.x, -self.half_size.y),
            ]), 
            Box::new([
                 Vec2::Y,
                -Vec2::X,
                -Vec2::Y,
                 Vec2::X
            ]),
        )
    }
}
