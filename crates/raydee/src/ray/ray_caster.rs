// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use super::RayIntersection;

pub struct RayCaster {
    origin:        Vec2,
    origin_dp:     [f32; 2],
    direction:     Vec2,
    direction_inv: Vec2,
}

impl RayCaster {

    #[must_use]
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        Self{
            origin, 
            origin_dp: [direction.dot(origin), direction.perp_dot(origin)],
            direction,
            direction_inv: Vec2::new(1.0/direction.x, 1.0/direction.y),
        }
    }

    #[must_use]
    pub fn new_x(origin: Vec2, dir: f32) -> Self {
        Self {
            origin,
            origin_dp: [dir * origin.x, dir * origin.y],
            direction: Vec2::new(dir, 0.0),
            direction_inv: Vec2::new(dir, f32::INFINITY),
        }
    }

    #[must_use]
    pub fn new_y(origin: Vec2, dir: f32) -> Self {
        Self {
            origin,
            origin_dp: [dir*origin.y, -dir*origin.x],
            direction: Vec2::new(0.0, dir),
            direction_inv: Vec2::new(f32::INFINITY, dir),
        }
    }

    #[must_use]
    pub const fn origin(&self) -> Vec2 {
        self.origin
    }

    #[must_use]
    pub const fn direction(&self) -> Vec2 {
        self.direction
    }

    #[must_use]
    pub fn stepped_back(self, amount: f32) -> Self {
        Self{
            origin: self.origin - self.direction*amount, 
            origin_dp: [self.origin_dp[0] - amount, self.origin_dp[1] - amount],
            direction: self.direction,
            direction_inv: self.direction_inv,
        }
    }

    #[must_use]
    pub fn offet_by(self, offset: Vec2) -> Self {
        let origin = self.origin + offset;
        Self{
            origin, 
            origin_dp: [self.direction.dot(origin), self.direction.perp_dot(origin)],
            direction: self.direction,
            direction_inv: self.direction_inv,
        }
    }

}

// ///////////////////// //
// // Raytest Circles // //
// ///////////////////// //

impl RayCaster {

    #[must_use]
    pub fn test_circle(&self, origin: Vec2, radius: f32) -> Option<[RayIntersection; 2]> {
        let ray_dp = self.offset_origin_dp(origin);
        RayCaster::calc_circle_center_offset(ray_dp, radius).map(|offset| {
            let distances = [-offset - ray_dp[0], offset - ray_dp[0]];
            let points  = distances.map(|d| self.origin + self.direction*d);
            let normals = points.map(|v| Vec2::normalize(v-origin));
    
            [
                RayIntersection{distance: distances[0], point: points[0], normal: normals[0]},
                RayIntersection{distance: distances[1], point: points[1], normal: normals[1]},
            ]
        })
    }

    #[must_use]
    pub fn test_circle_enter(&self, origin: Vec2, radius: f32) -> Option<RayIntersection> {
        let ray_dp = self.offset_origin_dp(origin);
        RayCaster::calc_circle_center_offset(ray_dp, radius).map(|offset| {

            let distance = -offset - ray_dp[0];
            let point  = self.origin + self.direction*distance;
            let normal = (point-origin).normalize();
    
            RayIntersection{distance, point, normal}
        })
    }

    #[must_use]
    pub fn test_circle_exit(&self, origin: Vec2, radius: f32) -> Option<RayIntersection> {
        let ray_dp = self.offset_origin_dp(origin);
        RayCaster::calc_circle_center_offset(ray_dp, radius).map(|offset| {

            let distance = offset - ray_dp[0];
            let point  = self.origin + self.direction*distance;
            let normal = (point-origin).normalize();
    
            RayIntersection{distance, point, normal}
        })
    }

    fn offset_origin_dp(&self, origin: Vec2) ->[f32; 2] {
        [
            self.origin_dp[0] - self.direction.dot(origin),
            self.origin_dp[1] - self.direction.perp_dot(origin)
        ]
    }

    fn calc_circle_center_offset(ray_dp: [f32; 2], radius: f32) -> Option<f32> {
        if radius < ray_dp[1].abs() { 
            None 
        } else {
            Some(radius*(1.0-(ray_dp[1]/radius).powi(2)).sqrt())
        }
    }

}

// /////////////////// //
// // Raytest Rects // //
// /////////////////// //

impl RayCaster {

    #[must_use]
    pub fn test_rect(&self, origin: Vec2, size: Vec2) -> Option<[RayIntersection; 2]> {
        let min = origin - size;
        let max = origin + size;

        let mut t = [-f32::INFINITY, f32::INFINITY];

        for d in 0..2 {
            t = Self::test_rect_minmax(d, self.origin, self.direction_inv, min, max, t);
        }

        (t[0] < t[1]).then(|| {
            let points  = t.map(|t| self.origin + self.direction*t);
            let normals = points.map(|p| Self::find_rect_normal_at(p - origin, size));
            [
                RayIntersection{distance: t[0], point: points[0], normal: normals[0]},
                RayIntersection{distance: t[1], point: points[1], normal: normals[1]},
            ]
        })
    }

    #[must_use]
    pub fn test_rect_enter(&self, origin: Vec2, size: Vec2) -> Option<RayIntersection> {
        self.test_rect(origin, size).map(|[v, _]| v)
    }

    #[must_use]
    pub fn test_rect_exit(&self, origin: Vec2, size: Vec2) -> Option<RayIntersection>{
        self.test_rect(origin, size).map(|[_, v]| v)
    }

    #[must_use]
    pub fn test_rect_rounded(&self, origin: Vec2, size: Vec2, radius: f32) -> Option<[RayIntersection; 2]> {
        // OPT axis aligned
        self.test_polygon_rounded_at_origin(
            &[
                origin + Vec2::new( size.x,  size.y),
                origin + Vec2::new(-size.x,  size.y),
                origin + Vec2::new(-size.x, -size.y),
                origin + Vec2::new( size.x, -size.y),
            ],
            &[
                 Vec2::Y,
                -Vec2::X,
                -Vec2::Y,
                 Vec2::X,
            ],
            &[
                2.0*size.x,
                2.0*size.y,
                2.0*size.x,
                2.0*size.y
            ],
            radius
        )
    }

    #[must_use]
    pub fn test_rect_rounded_enter(&self, origin: Vec2, size: Vec2, radius: f32) -> Option<RayIntersection> {
        // OPT sided polygon test
        self.test_rect_rounded(origin, size, radius).map(|[v, _]| v)
    }

    #[must_use]
    pub fn test_rect_rounded_exit(&self, origin: Vec2, size: Vec2, radius: f32) -> Option<RayIntersection>{
        // OPT sided polygon test
        self.test_rect_rounded(origin, size, radius).map(|[_, v]| v)
    }

    fn test_rect_minmax(
        idx: usize, 
        origin: Vec2, 
        direction_inv: Vec2, 
        min: Vec2, 
        max: Vec2, 
        t: [f32; 2]
    ) -> [f32; 2] {
        let t1 = (min[idx] - origin[idx]) * direction_inv[idx];
        let t2 = (max[idx] - origin[idx]) * direction_inv[idx];
        [
            f32::min(f32::max(t1, t[0]), f32::max(t2, t[0])),
            f32::max(f32::min(t1, t[1]), f32::min(t2, t[1])),
        ]
    }
    
    fn find_rect_normal_at(point: Vec2, size: Vec2) -> Vec2 {
        let pnt_abs = point.abs();
        let dist_x = pnt_abs.x - size.x; 
        let dist_y = pnt_abs.y - size.y;

        // TODO OPT make this better

        // +/- XY Quad
        if dist_x >= 0.0 && dist_y >= 0.0 {
            return Vec2::new(
                point.x.signum() * core::f32::consts::FRAC_1_SQRT_2,
                point.y.signum() * core::f32::consts::FRAC_1_SQRT_2,
            );
        }

        // +Y Quad
        if dist_x <= 0.0 && dist_y >= 0.0 {
            return Vec2::new(0.0, point.y.signum());
        }

        // +X Quad
        if dist_x >= 0.0 && dist_y <= 0.0 {
            return Vec2::new(point.x.signum(), 0.0);
        }

        let pnt_scl  = pnt_abs/size;

        // -Y Quad
        if pnt_scl.y > pnt_scl.x {
            return Vec2::new(0.0, point.y.signum());
        }

        // -X Quad
        if pnt_scl.x > pnt_scl.y {
            return Vec2::new(point.x.signum(), 0.0);
        }

        // Inside && x == y
        Vec2::new(
            point.x.signum() * core::f32::consts::FRAC_1_SQRT_2,
            point.y.signum() * core::f32::consts::FRAC_1_SQRT_2,
        )
    }

}

// ////////////////////// //
// // Raytest Polygons // //
// ////////////////////// //

impl RayCaster {

    #[must_use]
    pub fn test_polygon(&self, origin: Vec2, points: &[Vec2], normals: &[Vec2], lengths: &[f32]) -> Option<[RayIntersection; 2]> {
        RayIntersection::find_polygon_entry_exit((0..points.len()).filter_map(|i| self.test_line_opt(origin + points[i], normals[i].perp(), lengths[i])))
    }

    #[must_use]
    pub fn test_polygon_rounded(&self, origin: Vec2, points: &[Vec2], normals: &[Vec2], lengths: &[f32], radius: f32) -> Option<[RayIntersection; 2]> {
        RayIntersection::find_polygon_entry_exit((0..points.len()).flat_map(|i| {
            let point  = origin + points[i];
            let segment = self.test_line_opt(point + normals[i]*radius, normals[i].perp(), lengths[i]);
            if let Some([c_a, c_b]) = self.test_circle(point, radius) {
                [segment, Some(c_a), Some(c_b)]
            } else {
                [segment, None, None]
            }
        }).flatten())
    }

    #[must_use]
    pub fn test_polygon_rounded_at_origin(&self, points: &[Vec2], normals: &[Vec2], lengths: &[f32], radius: f32) -> Option<[RayIntersection; 2]> {
        RayIntersection::find_polygon_entry_exit((0..points.len()).flat_map(|i| {
            let point  = points[i];
            let segment = self.test_line_opt(point + normals[i]*radius, normals[i].perp(), lengths[i]);
            if let Some([c_a, c_b]) = self.test_circle(point, radius) {
                [segment, Some(c_a), Some(c_b)]
            } else {
                [segment, None, None]
            }
        }).flatten())
    }

    #[must_use]
    pub fn test_polygon_at_origin(&self, points: &[Vec2], normals: &[Vec2], lengths: &[f32]) -> Option<[RayIntersection; 2]> {
        RayIntersection::find_polygon_entry_exit((0..points.len()).filter_map(|i| self.test_line_opt(points[i], normals[i].perp(), lengths[i])))
    }

}

// /////////////////// //
// // Raytest Lines // //
// /////////////////// //

impl RayCaster {

    #[must_use]
    pub fn test_line(&self, from: Vec2, to: Vec2) -> Option<RayIntersection> {
        let offset = to - from;
        let len = offset.length();
        let dir = offset/len;
        self.test_line_opt(from, dir, len)
    }

    #[must_use]
    pub fn test_line_opt(&self, from: Vec2, dir: Vec2, len: f32) -> Option<RayIntersection> {
        self.calc_ray_intersection_dp(from, dir).and_then(|[distance, p]| (p >= 0.0 && p <= len).then(|| 
            RayIntersection {
                distance, 
                point:  self.origin + self.direction*distance, 
                normal: -dir.perp() 
            }
        ))
    }

    #[must_use]
    pub fn test_line_infinite(&self, from: Vec2, to: Vec2) -> Option<RayIntersection> {
        let dir = (to - from).normalize();
        self.test_line_infinite_opt(from, dir)
    }

    #[must_use]
    pub fn test_line_infinite_opt(&self, from: Vec2, dir: Vec2) -> Option<RayIntersection> {
        self.calc_ray_intersection_dp(from, dir).map(|[distance, _]| RayIntersection {
            distance, 
            point: self.origin + self.direction*distance, 
            normal: -dir.perp() 
        })
    }

    #[must_use]
    fn calc_ray_intersection_dp(&self, other_origin: Vec2, other_dir: Vec2) -> Option<[f32; 2]> {
        let inv_pdp = 1.0/self.direction.perp_dot(other_dir);
        (inv_pdp != f32::INFINITY).then(|| [
                    other_dir.perp_dot( self.origin - other_origin) * inv_pdp,
            -self.direction.perp_dot(other_origin -  self.origin) * inv_pdp,
        ])
    }

}