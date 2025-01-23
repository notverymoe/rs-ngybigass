// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{ray::{RayCaster, RayIntersection}, shape::ShapeMoving};

#[derive(Debug, Clone, Copy)]
pub struct MotionFrame {
    collider:  ShapeMoving,
    origin:    Vec2,
    direction: Vec2,
    distance:  f32,
}

impl MotionFrame {

    #[must_use]
    pub fn new_from_axis_distance(
        collider: impl Into<ShapeMoving>,
        origin: Vec2,
        direction: Vec2,
        distance:  f32,
    ) -> Self {
        Self { 
            collider: collider.into(),
            origin,
            direction,
            distance,
        }
    }

    #[must_use]
    pub fn new_from_target(
        collider: impl Into<ShapeMoving>,
        origin: Vec2,
        target: Vec2,
    ) -> Self {
        let collider = collider.into();
        let offset   = target - origin;
        let distance = offset.length_squared();

        if distance > 0.0 {
            let distance = distance.sqrt();
            Self::new_from_axis_distance(collider, origin, offset/distance, distance)
        } else {
            Self::new_from_axis_distance(collider, origin, Vec2::X, 0.0)
        }
    }

}


impl MotionFrame {

    #[must_use]
    pub fn move_to_end(self) -> Self {
        Self { 
            collider: self.collider, 
            origin: self.position_end(),
            direction: self.direction, 
            distance: 0.0
        }
    }

    #[must_use]
    pub fn move_to_hit_and_slide(self, hit: RayIntersection, hit_skin: f32) -> Self {
        if !hit.normal.is_normalized()     { bevy::log::warn!("Unnormalized hit normal, got length: {}", hit.normal.length()); }
        if !self.direction.is_normalized() { bevy::log::warn!("Unnormalized direction, got length: {}", self.direction.length()); }
        Self{
            collider:  self.collider,
            origin:    self.origin + self.direction()*(hit.distance-hit_skin),
            direction: perp_in_dir(hit.normal, self.direction),
            distance:  (self.distance-hit.distance).max(0.0)
        }
    }

}

#[allow(clippy::missing_const_for_fn)]
impl MotionFrame {

    #[must_use]
    pub fn ray_caster(self) -> RayCaster {
        RayCaster::new(self.origin, self.direction)
    }

    #[must_use]
    pub fn position_start(self) -> Vec2 {
        self.origin
    }

    #[must_use]
    pub fn position_end(self) -> Vec2 {
        self.origin + self.distance*self.direction
    }

    #[must_use]
    pub fn collider(self) -> ShapeMoving {
        self.collider
    }

    pub fn set_distance(&mut self, v: f32) {
        self.distance = v;
    }

    #[must_use]
    pub fn distance(self) -> f32 {
        self.distance
    }

    #[must_use]
    pub fn direction(self) -> Vec2 {
        self.direction
    }

}

fn perp_in_dir(n: Vec2, d: Vec2) -> Vec2 {
    let r = n.perp();
    tri_signum(r.dot(d), 1e-9) * r
}

fn tri_signum(v: f32, t: f32) -> f32 {
    if v.abs() < t { 0.0 } else { v.signum() }
}
