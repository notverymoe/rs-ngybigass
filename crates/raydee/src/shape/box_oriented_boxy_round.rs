// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, RayCaster, RayIntersection, ShapeDebug, ShapeDebugData, PolygonSmallRound, ShapeCommon};
use super::get_polygon_data_for_oriented_rect_rected;

#[derive(Debug, Clone, Copy)]
pub struct BoxOrientedBoxyRound(PolygonSmallRound);

impl BoxOrientedBoxyRound {
    #[must_use]
    pub fn new(size: Vec2, direction: Vec2, outer_size: Vec2, radius: f32) -> Self {
        Self(PolygonSmallRound::new_from_points(get_polygon_data_for_oriented_rect_rected(Vec2::ZERO, size, direction, outer_size), radius))
    }
}

impl ShapeCommon for BoxOrientedBoxyRound {
    fn bounding_box(&self) -> [Vec2; 2] {
        self.0.bounding_box()
    }
}

impl RayTarget for BoxOrientedBoxyRound {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        self.0.raycast(origin, ray)
    }
}

impl ShapeDebug for BoxOrientedBoxyRound {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        self.0.get_debug_shape_data()
    }
}
