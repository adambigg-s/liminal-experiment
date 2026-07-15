struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) tex: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in: u32) -> VertexOutput {
    var out: VertexOutput;

    let x = f32((in << 1) & 2);
    let y = f32(in & 2);

    out.pos = vec4<f32>(x * 2.0 - 1.0, y * -2.0 + 1.0, 0.0, 1.0);
    out.tex = vec2<f32>(x, y);

    return out;
}

@group(0) @binding(0) var diffuse_atlas: texture_2d<f32>;
@group(0) @binding(1) var normal_atlas: texture_2d<f32>;
@group(0) @binding(2) var specular_atlas: texture_2d<f32>;
@group(0) @binding(3) var sample_atlas: sampler;
@group(0) @binding(4) var<uniform> view_proj: mat4x4<f32>;
@group(0) @binding(5) var<uniform> view: mat4x4<f32>;
@group(0) @binding(6) var<uniform> screen_ar: f32;
@group(0) @binding(7) var<uniform> flashlight: f32;
@group(0) @binding(8) var<uniform> time: f32;

@group(1) @binding(0) var texture_post: texture_2d<f32>;
@group(1) @binding(1) var sample_post: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(texture_post, sample_post, in.tex);
    let final_color = vhs_vignette(color, in.tex);

    return final_color;
}

fn vhs_vignette(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let wobble_speed = 10.0;
    let wobble_freq = 20.0;
    let uv_shift = uv + vec2<f32>(sin(uv.y * wobble_freq + time * wobble_speed) * 0.002, 0.0);
    let dist = distance(uv, vec2<f32>(0.5, 0.5));

    let vignette = pow(smoothstep(0.75, 0.1, dist), 2.0);

    let freq_x = 800.0;
    let freq_y = 600.0;
    let noise_coord = vec2<f32>(uv_shift.x * freq_x + time * 100.0, floor(uv_shift.y * freq_y) + time * 50.0);
    let noise = rand(noise_coord) - 0.5;

    let noise_strength = 0.5;
    let noise_attenuation = pow(length(color.rgb), 2);
    let final_color = vignette * (color + (noise * noise_strength) * noise_attenuation);

    return final_color;
}

fn rand(coord: vec2<f32>) -> f32 {
    return fract(sin(dot(coord, vec2(12.239325, 78.293723))) * 2394.2343);
}
