// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use raydee::prelude::{MotionFrame, MotionQuery, ShapeCommon, ShapeMoving};

use crate::environment::collision::CollisionMap;

use super::Pawn;

#[derive(Debug, Clone, Copy)]
pub enum PawnMoveTarget {
    Relative(Vec2),
    Absolute(Vec2),
}

impl PawnMoveTarget {
    #[allow(clippy::missing_const_for_fn)] // Clippy please, vec2::add isn't const
    #[must_use]
    pub fn into_absolute(self, origin: Vec2) -> Vec2 {
        match self {
            PawnMoveTarget::Relative(v) => origin + v,
            PawnMoveTarget::Absolute(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PawnMove {
    pub entity: Entity,
    pub target: PawnMoveTarget,
    pub speed:  f32,
}

impl PawnMove {

    #[must_use]
    pub fn get_target(self, origin: Vec2) -> Vec2 {        
        let target = self.target.into_absolute(origin); 
        if self.speed > 0.0 {
            let delta = target - origin;
            let dist  = delta.length();
            if dist < self.speed {
                target
            } else {
                let dir = delta/dist;
                origin + self.speed*dir
            }
        } else {
            target
        }
    }

}

impl PawnMove {

    #[must_use]
    pub const fn absolute(entity: Entity, target: Vec2) -> Self {
        Self { entity, target: PawnMoveTarget::Absolute(target), speed: 0.0 }
    }
    
    #[must_use]
    pub const fn relative(entity: Entity, target: Vec2) -> Self {
        Self { entity, target: PawnMoveTarget::Relative(target), speed: 0.0 }
    }

    #[must_use]
    pub const fn with_entity(mut self, entity: Entity) -> Self {
        self.entity = entity;
        self
    }

    #[must_use]
    pub const fn with_target(mut self, target: PawnMoveTarget) -> Self {
        self.target = target;
        self
    }

    #[must_use]
    pub const fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn do_deferred(self, commands: &mut Commands) {
        commands.run_system_cached_with(move_pawn, self);
    }

}

fn move_pawn(
    In(action): In<PawnMove>,
    mut q_pawns: Query<&mut Pawn>,
    r_colliders: Res<CollisionMap>,
) {
    let mut pawn = q_pawns.get_mut(action.entity).unwrap();
    let target = action.get_target(pawn.origin); 
    let result = solve_motion(4, 1e-4, &r_colliders, pawn.layers(), pawn.collider, pawn.origin, target);
    pawn.origin = result.position_start();

    // let distance = result.position_start().distance(pawn.collider.origin);
    // if distance > movement.distance()*1.01 {
    //     let m_dist = movement.distance();
    //     bevy::log::warn!("Resolved move displacement exceeds original movement: {distance} | {m_dist}");
    // }

}


fn solve_motion(
    iteration_limit: usize,
    skin_distance: f32,
    colliders: &CollisionMap,
    layers: u16,
    collider: impl Into<ShapeMoving>,
    origin: Vec2,
    target: Vec2
) -> MotionFrame {

    let mut curr_motion = MotionFrame::new_from_target(collider, origin, target);
    let mut iter_remaining = if curr_motion.distance() > 0.0 { iteration_limit } else { 1 };

    while curr_motion.distance() > 0.0 {
        if iter_remaining == 0 {
            bevy::log::debug!("Exceeded movement solve iteration limit");
            break;
        }
        iter_remaining -= 1;

        let bbox  = curr_motion.collider().bounding_box();
        let start = curr_motion.position_start();
        let end   = curr_motion.position_end();

        let min = start.min(end)+bbox[0];
        let max = start.max(end)+bbox[1];

        let mut query = MotionQuery::new(curr_motion, skin_distance);

        (0..16).filter(|i| (layers & (i << 1)) == (i << 1))
            .filter_map(|i| colliders.try_get(i as usize))
            .for_each(|l| l.visit(min, max, |v| v.iter().for_each(|e| query.test(e.origin, &e.collider))));

        let mut next_motion = query.result();
        next_motion.set_distance(next_motion.distance() * Vec2::dot(curr_motion.direction(), next_motion.direction()).abs());
        curr_motion = next_motion;
    }
    
    curr_motion
}