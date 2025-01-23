// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, window::PrimaryWindow};

use bevy_asset_ldtk::{LDTKAssetPlugin, LDTKProject};
use raydee::prelude::{render_shape_debug_data_2d, DebugDrawOptions, ShapeDebug};

use game::{
    environment::{
        collision::CollisionMap, 
        pawn::{sync_pawn_transform, Pawn, PawnMove}
    }, 
    scale::{PixelsPerUnit, apply_pixel_scale, CameraPixelScaler},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LDTKAssetPlugin)
        .insert_resource(PixelsPerUnit(24.0))
        .insert_resource(CollisionMap::default())
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, setup_map)
        .add_systems(Update, (
            player_move_keeb, 
            player_move_mouse, 
            player_move_apply
        ).chain())
        .add_systems(PostUpdate, sync_pawn_transform)
        .add_systems(PostUpdate, collision_render_debug)
        .add_systems(PostUpdate, apply_pixel_scale)
        .run();
}

#[derive(Debug, Resource)]
pub struct GameProjectHandle(Handle<LDTKProject>);

fn setup_map(    
    player: Query<&PawnPlayer>,
    mut commands: Commands,
    mut asset_events: EventReader<AssetEvent<LDTKProject>>,
    assets: Res<Assets<LDTKProject>>,
    map_handle: Res<GameProjectHandle>,
    mut collision_map: ResMut<CollisionMap>
) {
    let player_spawned = !player.is_empty();
    let mut player_spawned_now = false;

    if asset_events.read().any(|ev| map_handle.0.id() == *match ev {
        AssetEvent::Added { id } => id,
        AssetEvent::Modified { id } => id,
        AssetEvent::Removed { id } => id,
        AssetEvent::Unused { id } => id,
        AssetEvent::LoadedWithDependencies { id } => id,
    }) {
        collision_map.clear();
        if let Some(project) = assets.get(&map_handle.0) {
            let world: &bevy_asset_ldtk::schemas::latest::World = project.worlds.first().unwrap();

            if let Some(level) = world.levels.iter().find(|l| l.identifier == "level_0") {

                if let Some(layer) = level.layer_instances.as_ref().and_then(|v| v.iter().find(|l| l.identifier == "walls")) {
                    for tile in &layer.auto_layer_tiles {
                        collision_map.get_mut(0).insert(
                            Vec2::new(
                                ((layer.px_total_offset_x + tile.px[0]) as f32)/24.0, 
                                -((layer.px_total_offset_y + tile.px[1]) as f32)/24.0
                            ),
                            Rectangle::from_size(Vec2::ONE),
                            None
                        );
                    }
                }

                if let Some(layer) = level.layer_instances.as_ref().and_then(|v| v.iter().find(|l| l.identifier == "entities")) {
                    for entity in &layer.entity_instances {
                        if entity.identifier == "spawn_player" {
                            if player_spawned || player_spawned_now {
                                if !player_spawned_now {
                                    bevy::log::warn!("Loaded additional spawn point.");
                                }
                            } else {
                                player_spawned_now = true;
                                commands.spawn((
                                    PawnPlayer{
                                        action_movement: None,
                                        move_speed_max: 4.0*3.6,
                                        move_speed_min: 1.4
                                    },
                                    Pawn::new(
                                        Vec2::new(
                                            ((layer.px_total_offset_x + entity.px[0]) as f32)/24.0, 
                                            -((layer.px_total_offset_y + entity.px[1]) as f32)/24.0
                                        ), 
                                        1.0, 
                                        0x01
                                    ),
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
                        }
                    }
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(GameProjectHandle(asset_server.load::<LDTKProject>("level/game.ldtk")));
}

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
struct CameraPlayer;

fn player_move_keeb(
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

fn player_move_mouse(
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

fn player_move_apply(
    mut commands: Commands,
    mut q_players: Query<&mut PawnPlayer>,
) {
    q_players.iter_mut().for_each(|mut player| {
        if let Some(action_movement) = core::mem::take(&mut player.action_movement) {
            action_movement.do_deferred(&mut commands);
        }
    });
}

fn reverse_lerp(value: f32, start: f32, end: f32) -> f32 {
    ((value - start)/(end - start)).clamp(0.0, 1.0)
}

fn try_get_cursor_world_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform
) -> Option<Vec2> {
    window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
}

fn collision_render_debug(
    q_players: Query<&Pawn, With<PawnPlayer>>,
    r_colliders: Res<CollisionMap>,
    mut gizmos: Gizmos
) {
    gizmos.grid_2d(Vec2::ONE*0.5, UVec2::new(100, 100), Vec2::ONE, LinearRgba::BLUE);

    q_players.iter().for_each(|pawn| { 
        gizmos.circle_2d(
            pawn.origin(),
            pawn.radius(),
            LinearRgba::rgb(1.0, 0.0, 0.0)
        );
    }); 

    r_colliders.iter().for_each(|layer| { 
        layer.iter().for_each(|e| render_shape_debug_data_2d(
            &mut gizmos, 
            e.origin,
            &e.collider.get_debug_shape_data(), 
            DebugDrawOptions::coloured(LinearRgba::GREEN)
                .with_draw_normals(false)
                .with_draw_normals_calculated(false)
        ));


        // layer.iter().for_each(|e| {
        //     let bounds = e.collider.bounding_box();
        //     let offset = 0.5*(bounds[0] + bounds[1]);
        //     render_shape_debug_data_2d(
        //         &mut gizmos, 
        //         e.origin+offset,
        //         &Rectangle::from_corners(bounds[0], bounds[1]).get_debug_shape_data(), 
        //         DebugDrawOptions::coloured(LinearRgba::rgb(1.0, 1.0, 0.0))
        //             .with_draw_normals(false)
        //             .with_draw_normals_calculated(false)
        //     )
        // });
    }); 

    // q_players.iter().for_each(|pawn| { 
    //     r_colliders.iter().for_each(|(_, layer)| { 
    //         layer.iter().for_each(|e| render_shape_debug_data_2d(
    //             &mut gizmos, 
    //             &ShapeCombined::between_moving_and_static(&pawn.collider().into(), &e.collider).get_debug_shape_data(), 
    //             DebugDrawOptions::coloured(LinearRgba::WHITE)
    //                 .with_draw_normals(true)
    //                 .with_draw_normals_calculated(true)
    //         ));
    //     }); 
    // }); 
}
