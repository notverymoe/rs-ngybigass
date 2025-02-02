// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, window::PrimaryWindow};

use crate::environment::pawn::{Pawn, PawnMove};

#[derive(Debug, Clone, Copy, Component)]
pub struct PawnPlayer {
    pub action_movement: Option<PawnMove>,

    pub move_speed_min: f32,
    pub move_speed_max: f32
}

impl PawnPlayer {
    pub fn set_move_target_and_retain_max_speed(&mut self, movement: PawnMove) {
        self.action_movement = self.action_movement.map_or_else(
            || Some(movement), 
            |existing| Some(movement.with_speed(movement.speed.max(existing.speed)))
        );
    }
}

#[derive(Component)]
pub struct CameraPlayer;

pub fn player_move_keeb(
    mut q_players: Query<(Entity, &mut PawnPlayer)>,
    r_time: Res<Time>,
    r_buttons: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;
    if r_buttons.pressed(KeyCode::KeyW) { direction += Vec2::Y; }
    if r_buttons.pressed(KeyCode::KeyS) { direction -= Vec2::Y; }
    if r_buttons.pressed(KeyCode::KeyD) { direction += Vec2::X; }
    if r_buttons.pressed(KeyCode::KeyA) { direction -= Vec2::X; }

    if let Some(direction) = direction.try_normalize() {
        let factor = if r_buttons.pressed(KeyCode::ShiftLeft) || r_buttons.pressed(KeyCode::ShiftRight) { 1.0 } else { 0.0 };
        q_players.iter_mut().for_each(|(entity, mut player)| {
            let move_speed = r_time.delta_secs() * f32::lerp(player.move_speed_min, player.move_speed_max, factor);
            player.set_move_target_and_retain_max_speed(PawnMove::relative(entity, direction).with_speed(move_speed));
        }); 
    }
}

pub fn player_move_mouse(
    mut q_players: Query<(Entity, &Pawn, &mut PawnPlayer)>,

    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraPlayer>>,

    r_time: Res<Time>,
    r_buttons: Res<ButtonInput<MouseButton>>,
) {
    if !r_buttons.pressed(MouseButton::Left) && !r_buttons.pressed(MouseButton::Right) { return; }

    let window = q_window.single();
    if let Ok((camera, camera_transform)) = q_camera.get_single() {
        if let Some(target) = try_get_cursor_world_position(window, camera, camera_transform) {
            q_players.iter_mut().for_each(|(entity, pawn, mut player)| {
                let factor = reverse_lerp(target.distance(pawn.origin()), 0.2, 2.0);
                let move_speed = r_time.delta_secs() * f32::lerp(player.move_speed_min, player.move_speed_max, factor);
                player.set_move_target_and_retain_max_speed(PawnMove::absolute(entity, target).with_speed(move_speed));
            }); 
        }
    }
}

pub fn player_move_apply(
    mut commands: Commands,
    mut q_players: Query<&mut PawnPlayer>,
) {
    q_players.iter_mut().for_each(|mut player| {
        if let Some(action_movement) = core::mem::take(&mut player.action_movement) {
            action_movement.do_deferred(&mut commands);
        }
    });
}

#[must_use]
pub fn try_get_cursor_world_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform
) -> Option<Vec2> {
    window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
}

#[must_use]
pub fn reverse_lerp(value: f32, start: f32, end: f32) -> f32 {
    ((value - start)/(end - start)).clamp(0.0, 1.0)
}