// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RayTarget, ShapeDebug, RayCaster, RayIntersection, ShapeDebugData, PolygonSmall, ShapeCommon};

#[derive(Debug, Clone, Copy)]
pub struct PolygonSmallRound {
    inner: PolygonSmall,
    radius: f32,
}

impl PolygonSmallRound {
    #[must_use]
    pub const fn new(polygon: PolygonSmall, radius: f32) -> Self {
        Self{inner: polygon, radius}
    }

    pub fn new_from_points(points: impl IntoIterator<Item = Vec2>, radius: f32) -> Self {
        Self::new(PolygonSmall::new_from_points(points), radius)
    }
}

impl ShapeCommon for PolygonSmallRound {
    fn bounding_box(&self) -> [Vec2; 2] {
        let bounds = self.inner.bounds;
        [
            bounds[0] - Vec2::new(self.radius, self.radius),
            bounds[1] + Vec2::new(self.radius, self.radius),
        ]
    }
}

impl RayTarget for PolygonSmallRound {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_polygon_rounded(origin, &self.inner.points, &self.inner.normals, &self.inner.lengths, self.radius)
    }
}

impl ShapeDebug for PolygonSmallRound {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        ShapeDebugData::polygon_round(  
            self.inner.points.to_vec().into_boxed_slice(),
            self.inner.normals.to_vec().into_boxed_slice(),
            self.radius,
        )
    }
}
