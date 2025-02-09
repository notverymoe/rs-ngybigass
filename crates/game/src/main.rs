// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use bevy_asset_ldtk::{schemas::v1_5_3::LdtkJson, util::{ldtk_resolve_layer_position, ldtk_resolve_layer_tile_position}, LDTKAssetPlugin, LDTKProject};

use game::{
    collision::CollisionMap,
    pawn::{sync_pawn_transform, Pawn},
    player::{player_move_apply, player_move_keeb, player_move_mouse, CameraPlayer, PawnPlayer},
    render::{MultiTextureAtlasBuilder, MultiTextureAtlasLoader, PluginMultiTextureAtlas},
    scale::{apply_pixel_scale, CameraPixelScaler, PixelsPerUnit}, tilemap::{PluginTilemapMaterial, TilemapMaterial, TilemapMaterialSync}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LDTKAssetPlugin)
        .add_plugins(PluginMultiTextureAtlas)
        .add_plugins(PluginTilemapMaterial)
        // .insert_resource(ClearColor(Color::BLACK))
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

#[derive(Debug, Component)]
pub struct LevelDespawnFlag;

fn setup_map(    
    mut commands: Commands,

    mut ev_asset_ldtk_events: EventReader<AssetEvent<LDTKProject>>,

    r_asset_server: Res<AssetServer>,
    r_assets_ldtk: Res<Assets<LDTKProject>>,    

    q_player: Query<&PawnPlayer>,
    q_despawn: Query<Entity, With<LevelDespawnFlag>>,

    r_map_active: Res<GameProjectHandle>,
    mut r_collision_map: ResMut<CollisionMap>,

    mut r_images: ResMut<Assets<Image>>,
    mut r_meshes: ResMut<Assets<Mesh>>,
    mut r_materials: ResMut<Assets<TilemapMaterial>>,
) {
    let player_spawned = !q_player.is_empty();
    let mut player_spawned_now = false;

    if ev_asset_ldtk_events.read().any(|ev| r_map_active.0.id() == *match ev {
        AssetEvent::Added { id } => id,
        AssetEvent::Modified { id } => id,
        AssetEvent::Removed { id } => id,
        AssetEvent::Unused { id } => id,
        AssetEvent::LoadedWithDependencies { id } => id,
    }) {
        r_collision_map.clear();
        for entity in &q_despawn {
            commands.entity(entity).despawn();
        }

        let Some(project) = r_assets_ldtk.get(&r_map_active.0) else { return; };
        let world: &bevy_asset_ldtk::schemas::latest::World = project.worlds.first().unwrap();
        let Some(level) = world.levels.iter().find(|l| l.identifier == "level_0") else { panic!("Level data contains no 'level_0' room"); };

        // // Tilesets // //

        let (atlas, mut loader) = MultiTextureAtlasBuilder::new(UVec2::new(24,24)).build_with_loader(&mut r_images);
        let mut tileset_entity = commands.spawn(atlas);
        let tileset_entity_id = tileset_entity.id();

        ldtk_load_tilesets(&r_asset_server, &mut loader, project, "level/");

        // // Tile Layers // //

        if let Some(layer) = level.layer_instances.as_ref().and_then(|v| v.iter().find(|l| l.identifier == "walls")) {

            let mut tilemap = TilemapMaterial::new(UVec2::new(layer.c_wid as u32, layer.c_hei as u32), None);

            if let Some(tileset) = layer.tileset_def_uid.and_then(|uid| loader.get(uid)) {
                for tile in &layer.auto_layer_tiles {

                    let position  = ldtk_resolve_layer_tile_position(layer, &tile.px);
                    let identifer = tileset.slots[tile.t as usize];
                    let flip      = tile.f;
                    
                    tilemap.set_tile(position, Some(identifer), flip & 0x01 != 0, flip & 0x02 != 0);
                    r_collision_map.get_mut(0).insert(position.as_vec2() + Vec2::ONE*0.5, Rectangle::from_size(Vec2::ONE), None);
                }
            }

            let mesh = tilemap.create_quad_mesh(1.0);
            let material = r_materials.add(tilemap);
            tileset_entity.commands_mut().spawn((
                TilemapMaterialSync::new(tileset_entity_id, &material),
                Mesh2d(r_meshes.add(mesh)),
                MeshMaterial2d(material),
                Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ));
        }

        tileset_entity.insert(loader);

        // // Entity Layers // //

        if let Some(layer) = level.layer_instances.as_ref().and_then(|v| v.iter().find(|l| l.identifier == "entities")) {
            for entity in &layer.entity_instances {
                if entity.identifier == "spawn_player" {
                    if player_spawned || player_spawned_now {
                        if !player_spawned_now {
                            bevy::log::warn!("Loaded additional spawn point.");
                        }
                    } else {
                        player_spawned_now = true;
                        spawn_player(
                            &mut commands, 
                            ldtk_resolve_layer_position(layer, &entity.px)
                        );
                    }
                }
            }
        }

    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameProjectHandle(asset_server.load::<LDTKProject>("level/game.ldtk")));
}

fn collision_render_debug(
    q_players: Query<&Pawn, With<PawnPlayer>>,
    mut gizmos: Gizmos
) {
    q_players.iter().for_each(|pawn| {
        gizmos.circle_2d(
            pawn.origin(),
            pawn.radius(),
            LinearRgba::rgb(1.0, 0.0, 0.0)
        );
    });
}

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

pub fn ldtk_load_tilesets(
    asset_server: &AssetServer,
    loader: &mut MultiTextureAtlasLoader,
    project: &LdtkJson,
    root_path: &str,
) {
    let defs: &bevy_asset_ldtk::schemas::latest::Definitions = &project.defs;
    for tileset in &defs.tilesets {
        if let Some(rel_path) = &tileset.rel_path {
            loader.insert(
                tileset.uid, 
                asset_server.load::<Image>([root_path, rel_path].join("")), 
                UVec2::new(tileset.c_wid   as u32, tileset.c_hei   as u32), 
                UVec2::new(tileset.padding as u32, tileset.padding as u32),
                UVec2::new(tileset.spacing as u32, tileset.spacing as u32),
            );
        }
    }
}