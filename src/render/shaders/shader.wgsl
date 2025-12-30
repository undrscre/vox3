struct VertexIn {
    @location(0) position: vec3<i32>,
    @location(1) packed: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) packed: u32,
}
@vertex
fn vs_main(in: VertexIn) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(f32(in.position.x), f32(in.position.y), f32(in.position.z), 1.0);
    out.packed = in.packed;
    return out;
}

@fragment
fn fs_main(vin: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.,1.,1., 1.0);
}
