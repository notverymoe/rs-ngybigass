// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, ShapeCommon};

#[derive(Debug, Clone, Copy)]
pub struct BoxOrientedRound {
    pub size:      Vec2,
    pub direction: Vec2,
    pub radius:    f32,
}

impl BoxOrientedRound {
    #[must_use]
    pub const fn new(size: Vec2, direction: Vec2, radius: f32) -> Self {
        Self{size, direction, radius}
    }
}

impl ShapeCommon for BoxOrientedRound {
    fn bounding_box(&self) -> [Vec2; 2] {
        let bound_x = Vec2::new( self.size.x,  self.size.y).rotate(self.direction).abs();
        let bound_y = Vec2::new(-self.size.x,  self.size.y).rotate(self.direction).abs();
        let size = bound_x.max(bound_y);
        [
            -Vec2::new(size.x + self.radius, size.y + self.radius),
            Vec2::new(size.x + self.radius, size.y + self.radius),
        ]
    }
}

impl RayTarget for BoxOrientedRound {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        let points = [
            origin + Vec2::new( self.size.x,  self.size.y).rotate(self.direction),
            origin + Vec2::new(-self.size.x,  self.size.y).rotate(self.direction),
            origin + Vec2::new(-self.size.x, -self.size.y).rotate(self.direction),
            origin + Vec2::new( self.size.x, -self.size.y).rotate(self.direction),
        ];

        let normals = [
            self.direction.perp(),
            -self.direction,
            -self.direction.perp(),
            self.direction
        ];

        let lengths = [
            2.0*self.size.x,
            2.0*self.size.y,
            2.0*self.size.x,
            2.0*self.size.y
        ];

        ray.test_polygon_rounded_at_origin(&points, &normals, &lengths, self.radius)
    }
}

impl ShapeDebug for BoxOrientedRound {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        ShapeDebugData::polygon_round(
            Box::new([
                Vec2::new( self.size.x,  self.size.y).rotate(self.direction),
                Vec2::new(-self.size.x,  self.size.y).rotate(self.direction),
                Vec2::new(-self.size.x, -self.size.y).rotate(self.direction),
                Vec2::new( self.size.x, -self.size.y).rotate(self.direction),
            ]), 
            Box::new([
                 self.direction.perp(),
                -self.direction,
                -self.direction.perp(),
                 self.direction
            ]),
            self.radius,
        )
    }
}
