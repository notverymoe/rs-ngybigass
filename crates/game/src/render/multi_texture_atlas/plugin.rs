// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::{MultiTextureAtlas, MultiTextureAtlasLoader};

pub struct PluginMultiTextureAtlas;

impl Plugin for PluginMultiTextureAtlas {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (mta_queue, mta_sync).chain());
    }
}

pub fn mta_queue(
    r_images:  Res<Assets<Image>>,
    mut q_mta: Query<(&mut MultiTextureAtlasLoader, &mut MultiTextureAtlas)>,
) {
    q_mta.iter_mut().for_each(|(mut loader, mut atlas)| {
        if loader.process_queue_required() {
            loader.process_queue(&mut atlas, &r_images);
        }
    });
}

pub fn mta_sync(
    mut r_images:  ResMut<Assets<Image>>,
    mut q_mta: Query<&mut MultiTextureAtlas>,
) {
    q_mta.iter_mut().for_each(|mut atlas| {
        if atlas.sync_required() {
            atlas.sync(&mut r_images);
        }
    });
}