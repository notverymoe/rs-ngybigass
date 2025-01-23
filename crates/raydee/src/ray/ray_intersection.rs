// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct RayIntersection {
    pub distance: f32,
    pub point:    Vec2,
    pub normal:   Vec2,
}

impl RayIntersection {

    #[must_use]
    pub fn find_polygon_entry_exit(v: impl IntoIterator<Item = RayIntersection>) -> Option<[RayIntersection; 2]> {
        let mut entry = RayIntersection{ distance:  f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        let mut exit  = RayIntersection{ distance: -f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        for intersection in v {
            if intersection.distance < entry.distance {
                entry = intersection;
            } 

            if intersection.distance > exit.distance {
                exit = intersection;
            }
        }

        (exit.distance >= entry.distance).then_some([entry, exit])
    }

    #[must_use]
    pub fn find_polygon_entry_exit_pairs(v: impl IntoIterator<Item = [RayIntersection; 2]>) -> Option<[RayIntersection; 2]> {
        let mut entry = RayIntersection{ distance:  f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        let mut exit  = RayIntersection{ distance: -f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        for [entry_intersection, exit_intersection] in v {
            if entry_intersection.distance < entry.distance {
                entry = entry_intersection;
            } 

            if exit_intersection.distance > exit.distance {
                exit = exit_intersection;
            }
        }

        (exit.distance >= entry.distance).then_some([entry, exit])
    }
    
}