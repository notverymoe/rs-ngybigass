// Copyright 2025 Natalie Baker // AGPLv3 //

// Copyright 2023 Natalie Baker // AGPLv3 //

use core::num::NonZeroU32;

use bevy::{asset::RenderAssetUsages, ecs::system::{lifetimeless::SRes, SystemParamItem}, pbr::{MaterialPipeline, MaterialPipelineKey}, prelude::*, render::{ mesh::{MeshVertexBufferLayoutRef, PrimitiveTopology}, render_asset::RenderAssets, render_resource::{AsBindGroup, AsBindGroupError, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingResources, BindingType, BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages, PreparedBindGroup, RenderPipelineDescriptor, SamplerBindingType, ShaderRef, ShaderStages, SpecializedMeshPipelineError, TextureSampleType, TextureViewDimension, UnpreparedBindGroup}, renderer::RenderDevice, texture::{FallbackImage, GpuImage}}, sprite::{Material2d, Material2dKey}};

use super::TextureBankIdentifier;

pub const HANDLE_SHADER_TEXTURE_BANK: Handle<Shader> = Handle::weak_from_u128(4_708_015_359_337_029_742);

#[derive(Asset, TypePath, Debug, Clone)]
pub struct TextureBankMaterial {
    size: UVec2,
    pub(super) bank_entity:   Option<Entity>,
    pub(super) bank_textures: Option<Box<[Handle<Image>]>>,
    pub(super) tile_data:     Vec<u32>,
}

impl TextureBankMaterial {
    const MAX_BANK_TEXTURES:    usize      = 16;
    const MAX_BANK_TEXTURES_NZ: NonZeroU32 = if let Some(v) = NonZeroU32::new(Self::MAX_BANK_TEXTURES as u32) { v } else { panic!("Unreachable.") };

    #[must_use] 
    pub fn new(
        size: UVec2, 
        bank_entity: Option<Entity>
    ) -> Self {
        let mut tile_data = vec![1; (size.x*size.y+2) as usize];
        tile_data[0] = size.x;
        tile_data[1] = size.y;
        Self { size, bank_entity, bank_textures: None, tile_data }
    }

    #[must_use] 
    pub fn create_quad_mesh(&self, depth: f32) -> Mesh {
        let size = self.size.as_vec2();
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
            Vec3::new(    0.0,   0.0, depth),
            Vec3::new(size.x,    0.0, depth),
            Vec3::new(   0.0, size.y, depth),
            Vec3::new(size.x, size.y, depth),
        ]);
        mesh
    }

}

impl TextureBankMaterial {

    pub fn set_tile(&mut self, position: UVec2, identifer: TextureBankIdentifier, flip_x: bool, flip_y: bool) {
        let data = ((flip_y as u32) << 17) | ((flip_x as u32) << 16) | (identifer.to_raw() as u32);
        self.tile_data[(2 + (position.x + position.y*self.size.x)) as usize] = data;
    }

}

impl AsBindGroup for TextureBankMaterial {
    type Data = ();
    type Param = (
        SRes<FallbackImage>,
        SRes<RenderAssets<GpuImage>>
    );

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        param: &mut SystemParamItem<'_, '_, Self::Param>,
    ) -> Result<PreparedBindGroup<Self::Data>, AsBindGroupError> {

        let (fallback_image, image_assets) = param;

        // Get bank textures to bind
        let fallback_image = &fallback_image.d2_array;
        let mut sampler = &fallback_image.sampler;
        let mut textures = [&*fallback_image.texture_view; Self::MAX_BANK_TEXTURES];
        if let Some(bank_textures) = &self.bank_textures {
            for (i, handle) in bank_textures.iter().take(Self::MAX_BANK_TEXTURES).enumerate() {
                if let Some(image) = image_assets.get(handle) {
                    textures[i] = &*image.texture_view;
                    sampler = &image.sampler;
                }
            }
        }

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor{
            label:    Some("tile_data"),
            contents: bytemuck::cast_slice(self.tile_data.as_slice()),
            usage:    BufferUsages::STORAGE,
        });


        // Create binding
        let bind_group = render_device.create_bind_group(
            "layer_material_bind_group", 
            layout, 
            &[
                BindGroupEntry{
                    binding: 0,
                    resource: BindingResource::TextureViewArray(textures.as_slice())
                },
                BindGroupEntry{
                    binding: 1,
                    resource: BindingResource::Sampler(sampler)
                },
                BindGroupEntry{
                    binding: 2,
                    resource: BindingResource::Buffer(BufferBinding{
                        buffer: &buffer,
                        offset: 0,
                        size: None
                    })
                },
            ]
        );

        Ok(PreparedBindGroup {
            bindings: BindingResources(vec![]),
            bind_group,
            data: (),
        })

    }

    fn bind_group_layout_entries(
        _render_device: &RenderDevice,
        _force_no_bindless: bool,
    ) -> Vec<BindGroupLayoutEntry>
    where
        Self: Sized
    {
        vec![
            BindGroupLayoutEntry{
                binding: 0,
                count: Some(Self::MAX_BANK_TEXTURES_NZ),
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture{
                    multisampled: false,
                    sample_type: TextureSampleType::Float{ filterable: true },
                    view_dimension: TextureViewDimension::D2Array
                }
            },
            BindGroupLayoutEntry{
                binding: 1,
                count: None,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
            },
            BindGroupLayoutEntry{
                binding: 2,
                count: None,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer{
                    ty: BufferBindingType::Storage{ read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            },
        ]
    }

    fn unprepared_bind_group(
        &self,
        _layout: &BindGroupLayout,
        _render_device: &RenderDevice,
        _param: &mut SystemParamItem<'_, '_, Self::Param>,
        _force_no_bindless: bool,
    ) -> Result<UnpreparedBindGroup<Self::Data>, AsBindGroupError> {
        #![allow(clippy::panic_in_result_fn, clippy::missing_panics_doc)]
        panic!("bindless texture arrays can't be owned")
    }
}

impl Material2d for TextureBankMaterial {
    fn vertex_shader() -> ShaderRef {
        HANDLE_SHADER_TEXTURE_BANK.into()
    }

    fn fragment_shader() -> ShaderRef {
        HANDLE_SHADER_TEXTURE_BANK.into()
    }
    
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![
            layout.0.get_layout(&[Mesh::ATTRIBUTE_POSITION.at_shader_location(0)])?,
        ];
        Ok(())
    }
}

impl Material for TextureBankMaterial {
    fn vertex_shader() -> ShaderRef {
        HANDLE_SHADER_TEXTURE_BANK.into()
    }

    fn fragment_shader() -> ShaderRef {
        HANDLE_SHADER_TEXTURE_BANK.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![
            layout.0.get_layout(&[Mesh::ATTRIBUTE_POSITION.at_shader_location(0)])?,
        ];
        Ok(())
    }
}
