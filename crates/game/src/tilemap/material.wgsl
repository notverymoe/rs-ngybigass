
#import bevy_render::view::View

#import bevy_sprite::mesh2d_functions

@group(0) @binding(0) var<uniform> view: View;

@group(2) @binding(0) var textures: binding_array<texture_2d_array<f32>>;
@group(2) @binding(1) var texture_sampler: sampler;
@group(2) @binding(2) var<storage, read> tile_data: array<u32>;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local_position: vec2<f32>,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let model          = mesh2d_functions::get_world_from_local(in.instance_index);
    out.clip_position  = mesh2d_functions::mesh2d_position_local_to_clip(model, vec4<f32>(in.position, 1.0));
    out.local_position = in.position.xy;
    return out;
}

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    // Check tile is in bounds
    // TODO OPT are we better off pinning UV to max and clamping x/y coord?
    if(u32(in.local_position.x) >= tile_data[0]) {
        discard;
    }
    if(u32(in.local_position.y) >= tile_data[1]) {
        discard;
    }

    // Tile Data
    var data = tile_data[2 + (u32(in.local_position.x) + u32(in.local_position.y) * tile_data[0])];

    // UV
    var base_uv = in.local_position.xy % 1.0;
    var flip_x  = (data & 0x10000) != 0;
    var flip_y  = (data & 0x20000) != 0;
    var uv = vec2<f32>(
        select(base_uv.x, 1.0 - base_uv.x, flip_x),
        select(base_uv.y, 1.0 - base_uv.y, flip_y)
    );

    // Output
    return textureSampleBank(data, uv);
}

fn textureSampleBank(id: u32, uv: vec2<f32>) -> vec4<f32> {
    var bank_id = (id >> 8) & 0x0F;
    var slot_id =  id       & 0xFF;
    return textureSample(textures[bank_id], texture_sampler, uv, slot_id);
}
