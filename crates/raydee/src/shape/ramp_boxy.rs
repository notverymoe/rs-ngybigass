// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, get_polygon_data_for_ramp_boxy, PolygonSmall, ShapeCommon};

#[derive(Debug, Clone, Copy)]
pub struct RampBoxy(PolygonSmall);

impl RampBoxy {
    #[must_use]
    pub fn new(direction: Vec2, length: f32, size: Vec2) -> Self {
        let (points, normals, lengths) = get_polygon_data_for_ramp_boxy(direction, length, size);
        let (min, max) = points.iter().fold((Vec2::MAX, Vec2::MIN), |p, &c| (p.0.min(c), p.0.max(c)));

        Self(PolygonSmall::new(points, normals, lengths, [min, max]))
    }
}

impl ShapeCommon for RampBoxy {
    fn bounding_box(&self) -> [Vec2; 2] {
        self.0.bounding_box()
    }
}

impl RayTarget for RampBoxy {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        self.0.raycast(origin, ray)
    }
}

impl ShapeDebug for RampBoxy {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        self.0.get_debug_shape_data()
    }
}
