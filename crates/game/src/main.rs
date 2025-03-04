// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{image::{ImageLoaderSettings, ImageSampler}, prelude::*};

use bevy_asset_aseprite::AsepriteAssetPlugin;
use bevy_asset_ldtk::{accessors::LdtkRoot, LDTKAssetPlugin, LDTKProject};

use game::{
    collision::CollisionMap,
    pawn::{sync_pawn_transform, Pawn},
    player::{player_move_apply, player_move_keeb, player_move_mouse, CameraPlayer, PawnPlayer},
    render::{MultiTextureAtlasBuilder, MultiTextureAtlasLoader, PluginMultiTextureAtlas, PluginTilemapMaterial, TilemapMaterial, TilemapMaterialSync},
    scale::{apply_pixel_scale, CameraPixelScaler, PixelsPerUnit},
};

// TODO Tilemap tile scale
// TODO Reuse existing atlas
// TODO Convience methods for ppu convertion
// TODO Convience methods for offset to center

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LDTKAssetPlugin)
        .add_plugins(AsepriteAssetPlugin)
        .add_plugins(PluginMultiTextureAtlas)
        .add_plugins(PluginTilemapMaterial)
        .insert_resource(ClearColor(Srgba::hex("111122").unwrap().into()))
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

    r_ppu: Res<PixelsPerUnit>,
    r_map_active: Res<GameProjectHandle>,
    mut r_collision_map: ResMut<CollisionMap>,

    mut r_images: ResMut<Assets<Image>>,
    mut r_meshes: ResMut<Assets<Mesh>>,
    mut r_materials: ResMut<Assets<TilemapMaterial>>,
) {
    let player_spawned = !q_player.is_empty();
    let mut player_spawned_now = false;

    if !ev_asset_ldtk_events.read().any(|ev| r_map_active.0.id() == *match ev {
        AssetEvent::Added    { .. } => return false,
        AssetEvent::Modified { id } => id,
        AssetEvent::Removed  { .. } => return false,
        AssetEvent::Unused   { .. } => return false,
        AssetEvent::LoadedWithDependencies { id } => id,
    }) { return; } 
    
    bevy::log::info!("[Level] Load start");

    r_collision_map.clear();
    for entity in &q_despawn {
        commands.entity(entity).despawn();
    }

    let Some(project) = r_assets_ldtk.get(&r_map_active.0) else { return; };
    let project = LdtkRoot::wrap(project);
    let world   = project.get_world(0).unwrap();
    let Some(level) = world.levels().find(|l| l.identifier() == "level_0") else { panic!("Level data contains no 'level_0' room"); };

    // // Tilesets // //

    let (atlas, mut loader) = MultiTextureAtlasBuilder::new(UVec2::new(24, 24)).build_with_loader(&mut r_images);
    let mut tileset_entity = commands.spawn((atlas, LevelDespawnFlag));
    let tileset_entity_id = tileset_entity.id();

    ldtk_load_tilesets(&r_asset_server, &mut loader, &project, "level/");

    // // Tile Layers // //

    if let Some(layer) = level.layers().find(|l| l.identifier() == "walls") {

        let mut tilemap = TilemapMaterial::new((layer.size_px().as_vec2()/(layer.size_grid() as f32)).ceil().as_uvec2(), None);

        if let Some(tileset) = layer.tileset_uid().and_then(|uid| loader.get(uid)) {
            for tile in layer.auto_layer_tiles() {

                // TODO check for non-round tile positions and warn
                let pos_local = tile.offset_local_px()/IVec2::splat(layer.size_grid());
                let pos_world = tile.offset_px()/IVec2::splat(layer.size_grid());
                let identifer = tileset.slots[tile.tileset_index()];
                
                tilemap.set_tile(pos_local.as_uvec2(), Some(identifer), tile.flip_x(), tile.flip_y());

                if layer.tileset_def().unwrap().has_enum_tag(tile.uid(), "solid") {
                    r_collision_map.get_mut(0).insert(pos_world.as_vec2() + Vec2::new(0.5, 0.5), Rectangle::from_size(Vec2::ONE), None);
                }
            }
        }

        let mesh = tilemap.create_quad_mesh(1.0);
        let material = r_materials.add(tilemap);
        tileset_entity.commands_mut().spawn((
            TilemapMaterialSync::new(tileset_entity_id, &material),
            Mesh2d(r_meshes.add(mesh)),
            MeshMaterial2d(material),
            Transform::from_translation((layer.offset_px().as_vec2()/r_ppu.0).extend(1.0)),
            LevelDespawnFlag
        ));
    }

    tileset_entity.insert(loader);

    // // Entity Layers // //

    if let Some(layer) = level.layers().find(|l| l.identifier() == "entities") {
        for entity in layer.entities() {
            if entity.identifier() == "spawn_player" {
                if player_spawned || player_spawned_now {
                    if !player_spawned_now {
                        bevy::log::warn!("Loaded additional spawn point.");
                    }
                } else {
                    player_spawned_now = true;
                    spawn_player(&mut commands, &r_asset_server, entity.offset_px().as_vec2()/r_ppu.0 + Vec2::splat(0.5));
                }
            }
        }
    }

    bevy::log::info!("[Level] Load complete");
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameProjectHandle(asset_server.load::<LDTKProject>("level/test.ldtk")));
}

pub fn spawn_player(
    commands: &mut Commands,
    assets: &AssetServer,
    position: Vec2
) {
    commands.spawn((
        PawnPlayer{
            action_movement: None,
            move_speed_max: 4.0*3.6,
            move_speed_min: 1.4
        },
        Sprite{
            image: assets.load_with_settings(
                "player.png", 
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                }
            ),
            custom_size: Some(Vec2::ONE),
            ..default()
        },
        Pawn::new(position, 1.0, 0x01),
        Transform::from_translation(position.extend(0.0)),
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
    root: &LdtkRoot,
    root_path: &str,
) {
    for tileset in root.tilesets() {
        if let Some(rel_path) = &tileset.rel_path() {
            loader.insert(
                tileset.uid(), 
                asset_server.load::<Image>([root_path, rel_path].join("")), 
                tileset.count(), 
                tileset.padding(),
                tileset.spacing(),
            );
        }
    }
}