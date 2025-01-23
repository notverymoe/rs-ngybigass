// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{math::primitives::Rectangle, prelude::Vec2};

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebugData, ShapeDebug, ShapeCommon};

#[derive(Debug, Clone, Copy)]
pub struct RectangleRounded {
    pub inner:  Rectangle,
    pub radius: f32,
}

impl RectangleRounded {
    #[must_use]
    pub const fn new(inner: Rectangle, radius: f32) -> Self {
        Self{inner, radius}
    }
}

impl ShapeCommon for RectangleRounded {
    fn bounding_box(&self) -> [Vec2; 2] {
        let half_size = self.inner.half_size + self.radius;
        [-half_size, half_size]
    }
}

impl RayTarget for RectangleRounded {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_rect_rounded(origin, self.inner.half_size, self.radius)
    }
}

impl ShapeDebug for RectangleRounded {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        ShapeDebugData::polygon_round( 
            Box::new([
                Vec2::new( self.inner.half_size.x,  self.inner.half_size.y),
                Vec2::new(-self.inner.half_size.x,  self.inner.half_size.y),
                Vec2::new(-self.inner.half_size.x, -self.inner.half_size.y),
                Vec2::new( self.inner.half_size.x, -self.inner.half_size.y),
            ]), 
            Box::new([
                 Vec2::Y,
                -Vec2::X,
                -Vec2::Y,
                 Vec2::X
            ]),
            self.radius,
        )
    }
}

