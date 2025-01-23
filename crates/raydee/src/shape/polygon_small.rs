// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use tinyvec::ArrayVec;

use crate::prelude::{RayTarget, ShapeDebug, RayCaster, RayIntersection, ShapeDebugData, ShapeCommon};

pub const POLYGON_SMALL_CAPACITY: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct PolygonSmall {
    pub(super) points:  ArrayVec<[Vec2; POLYGON_SMALL_CAPACITY]>,
    pub(super) normals: ArrayVec<[Vec2; POLYGON_SMALL_CAPACITY]>,
    pub(super) lengths: ArrayVec<[ f32; POLYGON_SMALL_CAPACITY]>,
    pub(super) bounds:  [Vec2; 2],
}

impl PolygonSmall {

    #[must_use]
    pub fn new(points: impl IntoIterator<Item = Vec2>, normals: impl IntoIterator<Item = Vec2>, lengths: impl IntoIterator<Item = f32>, bounds: [Vec2; 2]) -> Self {
        Self{
            points:  ArrayVec::from_iter(points ),
            normals: ArrayVec::from_iter(normals),
            lengths: ArrayVec::from_iter(lengths),
            bounds
        }
    }

    pub fn new_from_points(points: impl IntoIterator<Item = Vec2>) -> Self {
        let points:      ArrayVec<[Vec2; POLYGON_SMALL_CAPACITY]> = ArrayVec::from_iter(points);
        let mut normals: ArrayVec<[Vec2; POLYGON_SMALL_CAPACITY]> = ArrayVec::default();
        let mut lengths: ArrayVec<[ f32; POLYGON_SMALL_CAPACITY]> = ArrayVec::default();

        let mut min = Vec2::MAX;
        let mut max = Vec2::MIN;

        for i in 0..points.len() {
            let start = points[i];
            min = min.min(start);
            max = max.max(start);

            let end   = points[(i+1)%points.len()];
            let offset = end-start;
            let length = offset.length();
            let normal = -(offset/length).perp();
            lengths.push(length);
            normals.push(normal);
        }

        Self{points, normals, lengths, bounds: [min, max]}
    }
}

impl ShapeCommon for PolygonSmall {
    fn bounding_box(&self) -> [Vec2; 2] {
        self.bounds
    }
}

impl RayTarget for PolygonSmall {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_polygon(origin, &self.points, &self.normals, &self.lengths)
    }
}

impl ShapeDebug for PolygonSmall {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        ShapeDebugData::polygon(  
            self.points.to_vec().into_boxed_slice(),
            self.normals.to_vec().into_boxed_slice(),
        )
    }
}
