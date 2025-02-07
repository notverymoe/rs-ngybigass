// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{image::{ImageLoaderSettings, ImageSampler}, prelude::*};

use bevy_asset_ldtk::{util::ldtk_resolve_layer_position, LDTKAssetPlugin, LDTKProject};

use game::{
    collision::CollisionMap, level::spawn_player, pawn::{sync_pawn_transform, Pawn}, player::{player_move_apply, player_move_keeb, player_move_mouse, PawnPlayer}, scale::{apply_pixel_scale, PixelsPerUnit}, tilemap::{PluginTextureBank, TextureBank, TextureBankImportConfig, TextureBankMaterial}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LDTKAssetPlugin)
        .add_plugins(PluginTextureBank)
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

    mut asset_events: EventReader<AssetEvent<LDTKProject>>,

    // asset_server: Res<AssetServer>,
    assets_ldtk: Res<Assets<LDTKProject>>,    
    // mut assets_texture_asset_layouts: ResMut<Assets<TextureAtlasLayout>>,

    player: Query<&PawnPlayer>,
    despawn: Query<Entity, With<LevelDespawnFlag>>,

    map_handle: Res<GameProjectHandle>,
    mut collision_map: ResMut<CollisionMap>,
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
        for entity in &despawn {
            commands.entity(entity).despawn();
        }

        let Some(project) = assets_ldtk.get(&map_handle.0) else { return; };
        let world: &bevy_asset_ldtk::schemas::latest::World = project.worlds.first().unwrap();
        let Some(level) = world.levels.iter().find(|l| l.identifier == "level_0") else { panic!("Level data contains no 'level_0' room"); };

        // // Tilesets // //

        // let tilesets = ldtk_load_tilesets(&asset_server, &mut assets_texture_asset_layouts, project, "level/");

        // // Tile Layers // //

        // if let Some(layer) = level.layer_instances.as_ref().and_then(|v| v.iter().find(|l| l.identifier == "walls")) {
        //     let tileset = tilesets.lookup.get(&layer.tileset_def_uid.unwrap()).unwrap();
        //     for tile in &layer.auto_layer_tiles {
        //         spawn_ldtk_tile(&mut commands, layer, tile, tileset, &mut collision_map).insert(LevelDespawnFlag);
        //     }
        // }

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
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TextureBankMaterial>>,
) {
    commands.insert_resource(GameProjectHandle(asset_server.load::<LDTKProject>("level/game.ldtk")));

    let mut bank = TextureBank::new(UVec2::new(24, 24), 16, &mut images);
    bank.insert(
        "test", 
        &asset_server.load_with_settings::<Image, ImageLoaderSettings>(
            "level/walls.png",
            |s: &mut _| {
                *s = ImageLoaderSettings {
                    sampler: ImageSampler::nearest(),
                    ..ImageLoaderSettings::default()
                }
            },
        ),
        TextureBankImportConfig{
            count: UVec2::new(6, 5),
            offset: UVec2::ONE,
            spacing: UVec2::ONE*2
        }
    ).unwrap();
    let bank_entity = commands.spawn(bank);

    let material = TextureBankMaterial::new(UVec2::new(10,10), Some(bank_entity.id()));
    let mesh = material.create_quad_mesh(1.0);
    let material = materials.add(material);

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(0.0,0.0,-1.0)),
    ));
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
