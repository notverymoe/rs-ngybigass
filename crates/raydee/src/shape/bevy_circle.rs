// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{math::primitives::Circle, prelude::Vec2};

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, ShapeCommon};

impl ShapeCommon for Circle {
    fn bounding_box(&self) -> [Vec2; 2] {
        [
            -Vec2::new(self.radius, self.radius),
            Vec2::new(self.radius, self.radius)
        ]
    }
}

impl RayTarget for Circle {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_circle(origin, self.radius)
    }
}

impl ShapeDebug for Circle {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        ShapeDebugData::circle(self.radius)
    }
}