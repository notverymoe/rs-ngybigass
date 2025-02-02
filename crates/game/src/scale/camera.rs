// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, render::camera::ScalingMode};

use super::{ppu_scale_to_fit, PixelsPerUnit};

#[derive(Debug, Clone, Copy, Component)]
pub struct CameraPixelScaler {
    pub size_target_units: Vec2,
}

pub fn apply_pixel_scale(
    mut q_cameras: Query<(&mut Projection, &Camera, &CameraPixelScaler, &GlobalTransform)>, 
    r_ppu: Res<PixelsPerUnit>,
) {
    q_cameras.iter_mut().for_each(|(mut projection, camera, scaler, _transform)| {
        if let Projection::Orthographic(projection) = projection.as_mut() {
            if let Some(viewport_size) = camera.physical_viewport_size() {
                let size = ppu_scale_to_fit(
                    *r_ppu, 
                    scaler.size_target_units, 
                    viewport_size.as_vec2()
                );

                // projection.viewport_origin = Vec2::new(0.5, 0.5) + ((transform.translation().truncate() * r_ppu.0).round() - (transform.translation().truncate() * r_ppu.0))/viewport_size.as_vec2();
                projection.scaling_mode = ScalingMode::AutoMin { min_width: size.x, min_height: size.y }
            }
        }
    });
}
