// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{core_pipeline::core_2d::Camera2d, ecs::system::Commands, math::Vec2, render::view::InheritedVisibility, transform::components::Transform};

use crate::{pawn::Pawn, player::{CameraPlayer, PawnPlayer}, scale::CameraPixelScaler};

pub fn spawn_player(
    commands: &mut Commands,
    position: Vec2,
) {
    commands.spawn((
        PawnPlayer{
            action_movement: None,
            move_speed_max: 4.0*3.6,
            move_speed_min: 1.4
        },
        Pawn::new(position, 1.0, 0x01),
        Transform::IDENTITY,
        InheritedVisibility::VISIBLE
    )).with_child((
        Camera2d,
        CameraPlayer,
        CameraPixelScaler{
            size_target_units: Vec2::ONE*15.0
        }
    ));
}