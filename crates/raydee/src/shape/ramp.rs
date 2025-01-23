// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, get_polygon_data_for_ramp, ShapeCommon};

#[derive(Debug, Clone, Copy)]
pub struct Ramp {
    pub direction: Vec2,
    pub length:    f32,
}

impl Ramp {
    #[must_use]
    pub const fn new(direction: Vec2, length: f32) -> Self {
        Self{direction, length}
    }

    #[must_use]
    pub fn new_from_size(size: Vec2) -> Self {
        let length    = size.length();
        let direction = Vec2::new(size.x, -size.y)/length;
        Self{direction, length}
    }

    #[must_use]
    pub fn get_normal(&self) -> Vec2 {
        get_ramp_normal_from_dir(self.direction, self.length)
    }
}

#[must_use]
pub fn get_ramp_normal_from_dir(direction: Vec2, length: f32) -> Vec2 {
    let size = Vec2::new(direction.x, -direction.y) * length;
    if (size.x >= 0.0) == (size.y >= 0.0) {
        direction.perp()
    } else {
        -direction.perp()
    }
}

impl ShapeCommon for Ramp {
    fn bounding_box(&self) -> [Vec2; 2] {
        let h_size = Vec2::new(self.direction.x, -self.direction.y)*self.length*0.5;
        [
            h_size - h_size.abs(),
            h_size + h_size.abs(),
        ]
    }
}

impl RayTarget for Ramp {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ray.test_polygon(origin, &points, &normals, &lengths)
    }
}

impl ShapeDebug for Ramp {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        let (points, normals, _lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ShapeDebugData::polygon(
            Box::new(points), 
            Box::new(normals),
        )
    }
}