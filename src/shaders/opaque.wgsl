const AMBIENT: f32 = 0.0025;
const FADE_COLOR: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
const FOG_START: f32 = 25.0;
const FOG_END: f32 = 100.0;
const FOG_EXP: f32 = 1.0;
const VICINITY_START: f32 = 7.5;
const VICINITY_STRENGTH: f32 = 0.05;

struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) nor: vec3<f32>,
    @location(2) tex: vec2<f32>,
    @location(3) fil: f32,
    @location(4) bil: f32,
    @location(5) ao: f32,
};

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) world_pos: vec4<f32>,
    @location(1) nor: vec3<f32>,
    @location(2) tex: vec2<f32>,
    @location(3) fil: f32,
    @location(4) ao: f32,
    @location(5) ndc: vec3<f32>,
}

@group(0) @binding(0) var<uniform> view_proj: mat4x4<f32>;
@group(0) @binding(1) var<uniform> view: mat4x4<f32>;

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;

    out.pos = view_proj * vec4<f32>(in.pos, 1.0);
    out.world_pos = view * vec4<f32>(in.pos, 1.0);
    out.nor = in.nor;
    out.tex = in.tex;
    out.fil = in.fil;
    out.ao = in.ao;
    out.ndc = out.pos.xyz / out.pos.w;

    return out;
}

struct FragmentOutput {
    @builtin(frag_depth) depth: f32,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(2) var texture_atlas: texture_2d<f32>;
@group(0) @binding(3) var sample_atlas: sampler;

@fragment
fn fs_main(in: VertexOut) -> FragmentOutput {
    var out: FragmentOutput;

    let diffuse_color = textureSample(texture_atlas, sample_atlas, in.tex);
    if diffuse_color.a == 0.0 {
        discard;
    }

    let depth = length(in.world_pos.xyz);
    let fog_factor = pow(clamp((depth - FOG_START) / (FOG_END - FOG_START), 0.0, 1.0), FOG_EXP);
    let vicinity_factor = clamp(1.0 - depth / VICINITY_START, 0.0, 1.0);

    let ao = pow(in.ao, 0.0);
    var lum = pow(clamp(in.fil, AMBIENT, 1.0), 2.0);
    let vicinity_light = pow(vicinity_factor, 3.0) * VICINITY_STRENGTH;
    let shaded_color = diffuse_color * ao * (lum + vicinity_light);

    let final_color = mix(shaded_color, vec4<f32>(FADE_COLOR, 1.0), fog_factor);

    out.depth = in.pos.z;
    out.color = final_color;

    return out;
}
