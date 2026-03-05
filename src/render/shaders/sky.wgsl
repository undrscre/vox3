struct Camera { 
    view_proj: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    pos: vec3<f32>,
};

@group(0) @binding(0) var<uniform> camera: Camera;

struct SkyOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) view_dir: vec3<f32>,
}
@vertex
fn svs_main(@builtin(vertex_index) v_id: u32) -> SkyOutput {
    var out: SkyOutput;
    
    // yolooo
    var pos = array<vec2<f32>, 4>(
        vec2<f32>(-1.0,  1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>( 1.0, -1.0)
    );
    out.clip_position = vec4<f32>(pos[v_id], 0.0, 1.0);

    let world_pos = camera.inv_view_proj * out.clip_position;
    out.view_dir = normalize((world_pos.xyz / world_pos.w) - camera.pos.xyz);

    return out;
}

@fragment
fn sfs_main(in: SkyOutput) -> @location(0) vec4<f32> {
    let dir = normalize(in.view_dir);

    let horizon = vec4<f32>(0.7, 0.8, 0.9, 1.0);
    let zenith = vec4<f32>(0.1, 0.4, 0.8, 1.0);

    let factor = clamp(dir.y, 0.0, 1.0);
    let sky_color = mix(horizon, zenith, pow(factor, .35));
    
    let sun_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let sun_dot = -dot(dir, sun_dir);
    
    let sun_mask = step(0.9925, sun_dot);
    let glow_mask = step(0.98, sun_dot); 

    let sun_core_color = vec3<f32>(1.0, 1.0, 0.9);
    let glow_boost = vec3<f32>(0.15, 0.2, 0.3) / 10; 
    
    var final_rgb = sky_color.rgb;

    final_rgb += glow_boost * glow_mask; 
    final_rgb = mix(final_rgb, sun_core_color, sun_mask);

    return vec4<f32>(final_rgb, 1.0);
}
