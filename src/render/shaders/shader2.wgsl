struct Camera { view_proj: mat4x4<f32> };
@group(0) @binding(0) var<uniform> camera: Camera;

struct VertexIn {
    @location(0) position: vec3<i32>,
    @location(1) packed: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct UnpackedData {
    normal: vec3<f32>,
    block_type: u32
};

fn unpack_information(packed: u32) -> UnpackedData {
    let block_type = packed & 0xFFu;
    let dir = (packed >> 8u) & 0xFFu;
    
    var normal: vec3<f32>;
    switch dir {
        case 0u: { normal = vec3<f32>(1.0, 0.0, 0.0); }
        case 1u: { normal = vec3<f32>(-1.0, 0.0, 0.0); }
        case 2u: { normal = vec3<f32>(0.0, 1.0, 0.0); }
        case 3u: { normal = vec3<f32>(0.0, -1.0, 0.0); }
        case 4u: { normal = vec3<f32>(0.0, 0.0, 1.0); }
        case 5u: { normal = vec3<f32>(0.0, 0.0, -1.0); }
        default: { normal = vec3<f32>(0.0, 0.0, 0.0); }
    }
    
    return UnpackedData(normal, block_type);
}

fn hash3(p: vec3<f32>) -> f32 {
    var p3 = fract(p * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yxz + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

@vertex
fn vs_main(in: VertexIn) -> VertexOutput {
    var out: VertexOutput;
    var data: UnpackedData = unpack_information(in.packed);

    out.position = camera.view_proj * vec4<f32>(vec3<f32>(in.position), 1.0);
    out.world_pos = vec3<f32>(in.position) - data.normal * 0.001;
    out.normal = data.normal;

    return out;
}

@fragment
fn fs_main(vin: VertexOutput) -> @location(0) vec4<f32> {
    let block_pos = floor(vin.world_pos);
    let factor = 0.50 + 0.50 * hash3(block_pos);
    
    let color = vec3<f32>(0.5,0.5,0.5);
    return vec4<f32>(color * factor, 1.0);
}
