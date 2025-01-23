// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::{primitives::{Circle, Rectangle}, Vec2};
use macro_attr_2018::macro_attr;
use enum_derive_2018::EnumFromInner;

use crate::prelude::*;

macro_attr! {
    #[derive(EnumFromInner!, Debug, Clone, Copy)]
    pub enum ShapeCombined {
        Circle(Circle),
        
        Rectangle(Rectangle),
        RectangleRound(RectangleRounded),

        BoxOrientedRound(BoxOrientedRound),
        BoxOrientedBoxy(BoxOrientedBoxy),
        BoxOrientedBoxyRound(BoxOrientedBoxyRound),

        RampRound(RampRound),
        RampBoxy(RampBoxy),
        RampBoxyRound(RampBoxyRound),
    }
}

impl ShapeCommon for ShapeCombined {
    fn bounding_box(&self) -> [Vec2; 2] {
        match self {
            ShapeCombined::Circle(s)    => s.bounding_box(),
            ShapeCombined::Rectangle(s) => s.bounding_box(),
            ShapeCombined::RectangleRound(s)   => s.bounding_box(),
            ShapeCombined::BoxOrientedRound(s) => s.bounding_box(),
            ShapeCombined::BoxOrientedBoxy(s)  => s.bounding_box(),
            ShapeCombined::BoxOrientedBoxyRound(s) => s.bounding_box(),
            ShapeCombined::RampRound(s)     => s.bounding_box(),
            ShapeCombined::RampBoxy(s)      => s.bounding_box(),
            ShapeCombined::RampBoxyRound(s) => s.bounding_box(),
        }
    }
}

impl ShapeDebug for ShapeCombined {
    fn get_debug_shape_data(&self) -> ShapeDebugData {
        match self {
            ShapeCombined::Circle(s)    => s.get_debug_shape_data(),
            ShapeCombined::Rectangle(s) => s.get_debug_shape_data(),
            ShapeCombined::RectangleRound(s)   => s.get_debug_shape_data(),
            ShapeCombined::BoxOrientedRound(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxOrientedBoxy(s)  => s.get_debug_shape_data(),
            ShapeCombined::BoxOrientedBoxyRound(s) => s.get_debug_shape_data(),
            ShapeCombined::RampRound(s)     => s.get_debug_shape_data(),
            ShapeCombined::RampBoxy(s)      => s.get_debug_shape_data(),
            ShapeCombined::RampBoxyRound(s) => s.get_debug_shape_data(),
        }
    }
}

impl RayTarget for ShapeCombined {
    fn raycast(&self, origin: Vec2, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        match self {
            ShapeCombined::Circle(s)    => s.raycast(origin, ray),
            ShapeCombined::Rectangle(s) => s.raycast(origin, ray),
            ShapeCombined::RectangleRound(s)   => s.raycast(origin, ray),
            ShapeCombined::BoxOrientedRound(s) => s.raycast(origin, ray),
            ShapeCombined::BoxOrientedBoxy(s)  => s.raycast(origin, ray),
            ShapeCombined::BoxOrientedBoxyRound(s) => s.raycast(origin, ray),
            ShapeCombined::RampRound(s)     => s.raycast(origin, ray),
            ShapeCombined::RampBoxy(s)      => s.raycast(origin, ray),
            ShapeCombined::RampBoxyRound(s) => s.raycast(origin, ray),
        }
    }

    fn raycast_enter(&self, origin: Vec2, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeCombined::Circle(s)    => s.raycast_enter(origin, ray),
            ShapeCombined::Rectangle(s) => s.raycast_enter(origin, ray),
            ShapeCombined::RectangleRound(s)   => s.raycast_enter(origin, ray),
            ShapeCombined::BoxOrientedRound(s) => s.raycast_enter(origin, ray),
            ShapeCombined::BoxOrientedBoxy(s)  => s.raycast_enter(origin, ray),
            ShapeCombined::BoxOrientedBoxyRound(s) => s.raycast_enter(origin, ray),
            ShapeCombined::RampRound(s)     => s.raycast_enter(origin, ray),
            ShapeCombined::RampBoxy(s)      => s.raycast_enter(origin, ray),
            ShapeCombined::RampBoxyRound(s) => s.raycast_enter(origin, ray),
        }
    }

    fn raycast_exit(&self, origin: Vec2, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeCombined::Circle(s)    => s.raycast_exit(origin, ray),
            ShapeCombined::Rectangle(s) => s.raycast_exit(origin, ray),
            ShapeCombined::RectangleRound(s)   => s.raycast_exit(origin, ray),
            ShapeCombined::BoxOrientedRound(s) => s.raycast_exit(origin, ray),
            ShapeCombined::BoxOrientedBoxy(s)  => s.raycast_exit(origin, ray),
            ShapeCombined::BoxOrientedBoxyRound(s) => s.raycast_exit(origin, ray),
            ShapeCombined::RampRound(s)     => s.raycast_exit(origin, ray),
            ShapeCombined::RampBoxy(s)      => s.raycast_exit(origin, ray),
            ShapeCombined::RampBoxyRound(s) => s.raycast_exit(origin, ray),
        }
    }
}

impl ShapeCombined {

    #[must_use]
    pub fn between_moving_and_static(a: &ShapeMoving, b: &ShapeStatic) -> Self {
        match (a, b) {
            (ShapeMoving::Circle(a),    ShapeStatic::Circle(b)         ) => Circle::new(a.radius+b.radius).into(),
            (ShapeMoving::Circle(a),    ShapeStatic::Rectangle(b)      ) => RectangleRounded::new(*b, a.radius).into(),
            (ShapeMoving::Circle(a),    ShapeStatic::BoxAlignedRound(b)) => RectangleRounded::new(b.inner, b.radius + a.radius).into(),
            (ShapeMoving::Rectangle(a), ShapeStatic::Circle(b)         ) => RectangleRounded::new(*a, b.radius).into(),
            (ShapeMoving::Rectangle(a), ShapeStatic::Rectangle(b)      ) => Rectangle{half_size: a.half_size + b.half_size}.into(),
            (ShapeMoving::Rectangle(a), ShapeStatic::BoxAlignedRound(b)) => RectangleRounded::new(Rectangle{half_size: a.half_size + b.inner.half_size}, b.radius).into(),

            (ShapeMoving::Circle(a),    ShapeStatic::BoxOriented(b)     ) => BoxOrientedRound::new(b.size, b.direction, a.radius).into(),
            (ShapeMoving::Circle(a),    ShapeStatic::BoxOrientedRound(b)) => BoxOrientedRound::new(b.size, b.direction, b.radius + a.radius).into(),
            (ShapeMoving::Rectangle(a), ShapeStatic::BoxOriented(b)     ) => BoxOrientedBoxy::new(b.size, b.direction, a.half_size).into(),
            (ShapeMoving::Rectangle(a), ShapeStatic::BoxOrientedRound(b)) => BoxOrientedBoxyRound::new(b.size, b.direction, a.half_size, b.radius).into(),

            // TODO do we need to "invert" ramps?
            // NOTE seems not?
            (ShapeMoving::Circle(a),    ShapeStatic::Ramp(b)     ) => RampRound::new(b.direction, b.length, a.radius).into(),
            (ShapeMoving::Circle(a),    ShapeStatic::RampRound(b)) => RampRound::new(b.direction, b.length, b.radius + a.radius).into(),
            (ShapeMoving::Rectangle(a), ShapeStatic::Ramp(b)     ) => RampBoxy::new(b.direction, b.length, a.half_size).into(),
            (ShapeMoving::Rectangle(a), ShapeStatic::RampRound(b)) => RampBoxyRound::new(b.direction, b.length, a.half_size, b.radius).into(),
        } 
    }

    #[must_use]
    pub fn between_moving(a: &ShapeMoving, b: &ShapeMoving) -> Self {
        match (a, b) {
            (ShapeMoving::Circle(a),    ShapeMoving::Circle(b)   ) => Circle::new(a.radius+b.radius).into(),
            (ShapeMoving::Circle(a),    ShapeMoving::Rectangle(b)) => RectangleRounded::new(*b, a.radius).into(),
            (ShapeMoving::Rectangle(a), ShapeMoving::Circle(b)   ) => RectangleRounded::new(*a, b.radius).into(),
            (ShapeMoving::Rectangle(a), ShapeMoving::Rectangle(b)) => Rectangle{half_size: a.half_size + b.half_size}.into(),
        }
    }

}