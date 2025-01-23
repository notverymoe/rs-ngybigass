// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use crate::{ray::{RayIntersection, RayTarget}, shape::{ShapeCombined, ShapeStatic}};

use super::MotionFrame;

#[derive(Debug, Clone, Copy)]
pub struct MotionQuery {
    motion: MotionFrame,
    skin_distance: f32,
    hit: Option<RayIntersection>,
    distance_max: f32,
}

impl MotionQuery {

    #[must_use] 
    pub fn new(motion: MotionFrame, skin_distance: f32) -> Self {
        Self {
            motion,
            skin_distance,
            hit: None,
            distance_max: motion.distance()
        }
    }

    pub fn test<'a>(&mut self, collider_origin: Vec2, collider_shape: impl Into<&'a ShapeStatic>) {
        let caster = self.motion.ray_caster();
        let combined = ShapeCombined::between_moving_and_static(&self.motion.collider().shrunk_by(self.skin_distance), collider_shape.into());
        if let Some(hit) = combined.raycast_enter(collider_origin, &caster) {
            if hit.distance < self.distance_max && hit.distance >= -self.skin_distance && hit.normal.dot(self.motion.direction()) < 0.0 {
                self.distance_max = hit.distance;
                self.hit          = Some(hit);
            }
        }
    }

    #[must_use]
    pub fn result(&self) -> MotionFrame {
        if let Some(hit) = self.hit {
            self.motion.move_to_hit_and_slide(hit, self.skin_distance)
        } else {
            self.motion.move_to_end()
        }
    }

}

impl MotionQuery {

    #[must_use]
    pub const fn motion(&self) -> &MotionFrame {
        &self.motion
    }

    #[must_use]
    pub const fn skin_distance(&self) -> f32 {
        self.skin_distance
    }

}
