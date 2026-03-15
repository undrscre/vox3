struct Camera { 
    view_proj: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    pos: vec3<f32>,
};

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<storage, read> chunk_offsets: array<vec4<f32>>;
@group(2) @binding(0) var t_diffuse: texture_2d_array<f32>;
@group(2) @binding(1) var s_diffuse: sampler;

struct VertexIn {
    @builtin(instance_index) draw_id: u32,
    @location(0) packed: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
    @location(3) block_type: u32,
};

struct UnpackedData {
    pos: vec3<f32>,
    normal: vec3<f32>,
    block_type: u32,
    uv: vec2<f32>,
};

fn unpack_information(packed: u32) -> UnpackedData {
    let x = f32(packed & 0x3Fu);
    let y = f32((packed >> 6u) & 0x3Fu);
    let z = f32((packed >> 12u) & 0x3Fu);
    let dir = (packed >> 18u) & 0x7u;
    let block_type = (packed >> 21u) & 0x1FFu;
    let uv_id = (packed >> 30u) & 0x3u;

    var normal: vec3<f32>;
    switch dir {
        case 0u: { normal = vec3<f32>(1.0, 0.0, 0.0); }  // +x
        case 1u: { normal = vec3<f32>(-1.0, 0.0, 0.0); } // -x
        case 2u: { normal = vec3<f32>(0.0, 1.0, 0.0); }  // +y
        case 3u: { normal = vec3<f32>(0.0, -1.0, 0.0); } // -y
        case 4u: { normal = vec3<f32>(0.0, 0.0, 1.0); }  // +z
        case 5u: { normal = vec3<f32>(0.0, 0.0, -1.0); } // -z
        default: { normal = vec3<f32>(0.0, 0.0, 0.0); }
    }

    let uvs = array<vec2<f32>, 4>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 1.0)
    );

    return UnpackedData(
        vec3<f32>(x, y, z), 
        normal, 
        block_type, 
        uvs[uv_id]
    );
}

fn hash3(p: vec3<f32>) -> f32 {
    var p3 = fract(p * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yxz + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

@vertex
fn vs_main(in: VertexIn) -> VertexOutput {
    var out: VertexOutput;
    let data = unpack_information(in.packed);

    let world_offset = chunk_offsets[in.draw_id].xyz;
    let world_pos = (data.pos + world_offset);

    out.position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    
    out.world_pos = world_pos - data.normal * 0.001; 
    out.normal = data.normal;
    out.block_type = data.block_type;
    out.uv = data.uv;

    return out;
}

var<private> ambient_strength: f32 = 0.5;
var<private> ambient_light: vec3<f32> = vec3<f32>(0.4, 0.4, 0.8);

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.uv, i32(in.block_type));
    
    if (tex_color.a < 0.1) { discard; }

    let block_pos = floor(in.world_pos);
    let factor = 1.0 - 0.25 * hash3(block_pos);

    let sun_direction = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let normal = normalize(in.normal);
    let diff = max(dot(normal, sun_direction), 0.0);

    let ambient = ambient_light * ambient_strength;
    
    // final color = texture * grit * (lighting)
    let result = (tex_color.rgb * factor) * (ambient + diff);

    return vec4<f32>(result, 1.0);
}