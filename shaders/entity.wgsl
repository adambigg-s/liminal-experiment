struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) nor: vec3<f32>,
    @location(2) tex: vec2<f32>,
};

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) nor: vec3<f32>,
    @location(1) tex: vec2<f32>,
};

@group(0) @binding(0) var<uniform> view_proj: mat4x4<f32>;
@group(0) @binding(1) var<uniform> view: mat4x4<f32>;

@group(1) @binding(0) var<uniform> model: mat4x4<f32>;

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;

    out.pos = view_proj * model * vec4<f32>(in.pos, 1.0);
    out.nor = (view * vec4<f32>(in.nor, 1.0)).xyz;
    out.tex = in.tex;

    return out;
}

@group(0) @binding(2) var texture_atlas: texture_2d<f32>;
@group(0) @binding(3) var sample_atlas: sampler;
@group(0) @binding(4) var<uniform> screen_ar: f32;
@group(0) @binding(5) var<uniform> flashlight: f32;
@group(0) @binding(6) var<uniform> time: f32;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let color = textureSample(texture_atlas, sample_atlas, in.tex);

    return color;
}

