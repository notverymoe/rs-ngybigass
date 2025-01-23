// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, get_polygon_data_for_ramp, ShapeCommon};

#[derive(Debug, Clone, Copy)]
pub struct RampRound {
    pub direction: Vec2,
    pub length:    f32,
    pub radius:    f32,
}

impl RampRound {
    #[must_use]
    pub const fn new(direction: Vec2, length: f32, radius: f32) -> Self {
        Self{direction, length, radius}
    }

    #[must_use]
    pub fn get_normal(&self) -> Vec2 {
        let size = Vec2::new(self.direction.x, -self.direction.y) * self.length;
        if (size.x >= 0.0) == (size.y >= 0.0) {
            self.direction.perp()
        } else {
            -self.direction.perp()
        }
    }
}

impl ShapeCommon for RampRound {
    fn bounding_box(&self) -> [Vec2; 2] {
        let h_size = Vec2::new(self.direction.x, -self.direction.y)*self.length*0.5;
        let origin = h_size+h_size.signum()*self.radius;
        [
            origin + (h_size.abs() + self.radius),
            origin - (h_size.abs() + self.radius),
        ]
    }
}

impl RayTarget for RampRound {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ray.test_polygon_rounded(origin, &points, &normals, &lengths, self.radius)
    }
}

impl ShapeDebug for RampRound {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        let (points, normals, _lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ShapeDebugData::polygon_round( 
            Box::new(points), 
            Box::new(normals),
            self.radius,
        )
    }
}
