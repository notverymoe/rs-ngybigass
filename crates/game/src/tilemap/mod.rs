// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{asset::load_internal_asset, prelude::*, sprite::Material2dPlugin};

mod tilemap_entry;
pub use tilemap_entry::*;

mod texture_bank;
pub use texture_bank::*;

mod texture_bank_import;
pub use texture_bank_import::*;

mod texture_bank_identifier;
pub use texture_bank_identifier::*;

mod texture_bank_entry;
pub use texture_bank_entry::*;

mod material;
pub use material::*;

pub struct PluginTextureBank;

impl Plugin for PluginTextureBank {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, HANDLE_SHADER_TEXTURE_BANK, "material.wgsl", Shader::from_wgsl);
        app
            .add_systems(PostUpdate, sync_texture_bank_textures)
            .init_asset::<TextureBankMaterial>()
            .add_plugins(Material2dPlugin::<TextureBankMaterial>::default());
    }
}

pub fn sync_texture_bank_textures(
    mut r_materials: ResMut<Assets<TextureBankMaterial>>,
    mut r_images:    ResMut<Assets<Image>>,
    mut q_banks:     Query<&mut TextureBank>,
) {
    for (material, bank) in r_materials.iter_mut().filter_map(|(_, material)| material.bank_entity.map(|v| (material, v))) {
        let mut bank = q_banks.get_mut(bank).unwrap();
        if bank.has_pending_queue() {
            bank.process_queue(&r_images);
        }
        if bank.needs_sync() {
            bank.sync_images(&mut r_images);
        }
        if material.bank_textures.is_none() {
            material.bank_textures = Some(bank.handles().to_vec().into_boxed_slice());
        }
    }
}
