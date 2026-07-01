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
    let depth = length(in.world_pos.xyz);

    out.depth = in.pos.z;
    out.color = diffuse_color;

    return out;
}
