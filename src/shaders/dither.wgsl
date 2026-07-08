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

@group(0) @binding(0) var texture_post: texture_2d<f32>;
@group(0) @binding(1) var sample_post: sampler;

const BAYER_DITHER: mat4x4<f32> = mat4x4<f32>(
    0.0 / 16.0, 8.0 / 16.0, 2.0 / 16.0, 10.0 / 16.0,
    12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0, 6.0 / 16.0,
    3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0,
    15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0,
);

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(texture_post, sample_post, in.tex);

    let coord = vec2<u32>(in.pos.xy);
    let x = coord.x / 2 % 4;
    let y = coord.y / 2 % 4;
    let dither = BAYER_DITHER[x][y];

    let spread = 0.05;
    let dither_noise = spread * (dither - 0.5);
    let adjusted_color = color.rgb + vec3<f32>(dither_noise);

    let color_depth = 128.0;
    let quantized_color = floor(adjusted_color * color_depth) / color_depth;

    let final_color = vec4<f32>(quantized_color, 1.0);

    return final_color;
}
