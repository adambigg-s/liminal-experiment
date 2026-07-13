const BACKROOMS_EPS: f32 = 1e-3;

const AMBIENT: f32 = 0.01;

const FADE_COLOR: vec3<f32> = vec3<f32>(0.005, 0.0, 0.0);

const FOG_EXP: f32 = 1.5;
const FOG_START: f32 = 25.0;
const FOG_END: f32 = 125.0;

const VICINITY_START: f32 = 7.5;
const VICINITY_STRENGTH: f32 = 0.05;

const BACKROOMS_LIGHT: vec4<f32> = vec4<f32>(1.0, 0.90, 0.60, 1.0);

const FL_END: f32 = 50.0;
const FL_INNER: f32 = 0.1;
const FL_OUTER: f32 = 1.5;
const FL_STRENGTH: f32 = 1.0;

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
    out.nor = (view * vec4<f32>(in.nor, 0.0)).xyz;
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
@group(0) @binding(4) var<uniform> screen_ar: f32;
@group(0) @binding(5) var<uniform> flashlight: f32;
@group(0) @binding(6) var<uniform> time: f32;

@fragment
fn fs_main(in: VertexOut) -> FragmentOutput {
    var out: FragmentOutput;

    let diffuse_color = textureSampleBias(texture_atlas, sample_atlas, in.tex, -0.25);
    if diffuse_color.a < BACKROOMS_EPS {
        discard;
    }

    let depth = length(in.world_pos.xyz);
    let fog_factor = pow(clamp((depth - FOG_START) / (FOG_END - FOG_START), 0.0, 1.0), FOG_EXP);
    let vicinity_factor = clamp(1.0 - depth / VICINITY_START, 0.0, 1.0);

    let center = vec2<f32>(in.ndc.x * screen_ar, in.ndc.y);
    let dist = length(center);
    let normal = normalize(in.nor);
    let spot_factor = pow(smoothstep(FL_OUTER, FL_INNER, dist), 3.0);
    let fl_attenuation = pow(clamp(1.0 - pow((depth / FL_END), 2.0), 0.0, 1.0), 4.0);
    let fl_dir = normalize(-in.world_pos.xyz);
    let fl_surface = max(dot(fl_dir, normal), 0.0);
    let fl_light = spot_factor * fl_attenuation * fl_surface * FL_STRENGTH * flashlight;

    let ao = pow(in.ao, 1.0);
    var lum = pow(clamp(in.fil, AMBIENT, 1.0), 2.0) * 1.5;
    let vicinity_light = pow(vicinity_factor, 3.0) * VICINITY_STRENGTH;
    let tint_mix = smoothstep(0.8, 1.0, in.fil);
    let light_color = mix(BACKROOMS_LIGHT, vec4<f32>(1.0), tint_mix);

    var total_light = (lum + vicinity_light + fl_light) * light_color;
    let shaded_color = diffuse_color * ao * total_light;

    let final_color = mix(shaded_color, vec4<f32>(FADE_COLOR, 1.0), fog_factor);

    out.depth = in.pos.z;
    out.color = final_color;

    return out;
}
