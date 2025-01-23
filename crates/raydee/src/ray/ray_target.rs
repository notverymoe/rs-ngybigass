// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use super::{RayCaster, RayIntersection};

pub trait RayTarget {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]>;

    fn raycast_enter(&self, origin: Vec2, ray: &RayCaster) -> Option<RayIntersection> {
        self.raycast(origin, ray).map(|[v, _]| v)
    }

    fn raycast_exit(&self, origin: Vec2, ray: &RayCaster) -> Option<RayIntersection>{
        self.raycast(origin, ray).map(|[_, v]| v)
    }
}