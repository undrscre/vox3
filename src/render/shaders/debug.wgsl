struct Camera { view_proj: mat4x4<f32> };
@group(0) @binding(0) var<uniform> camera: Camera;

struct VertexIn {
    @location(0) position: vec3<i32>,
    @location(1) packed: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct UnpackedData {
    normal: vec3<f32>,
    block_type: u32
};

@vertex
fn dvs_main(in: VertexIn) -> VertexOutput {
    var out: VertexOutput;
    if (in.packed == 1) {
        out.color = vec3<f32>(0.0, 1.0, 0.0); 
    } else if (in.packed == 2) {
        out.color = vec3<f32>(1.0, 0.0, 0.0);
    } else {
        out.color = vec3<f32>(1.0, 1.0, 1.0);
    }

    out.position = camera.view_proj * vec4<f32>(vec3<f32>(in.position), 1.0);
    return out;
}

@fragment
fn dfs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let result = in.color;
    return vec4<f32>(result, 1.0);
}