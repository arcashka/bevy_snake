#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

@group(1) @binding(100) var<uniform> u_FieldSize: vec2<i32>;
@group(1) @binding(101) var tile_texture: texture_2d<f32>;
@group(1) @binding(102) var tile_sampler: sampler;

@fragment
fn fragment(
    input: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(input, is_front);

    let new_uv_x = (input.uv.x * f32(u_FieldSize.x)) % 1.0;
    let new_uv_y = (input.uv.y * f32(u_FieldSize.y)) % 1.0;
    let new_uv = vec2<f32>(new_uv_x, new_uv_y);
    let color = textureSample(tile_texture, tile_sampler, new_uv);

    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color
    );
    let is_top = all(pbr_input.world_normal == vec3<f32>(0.0, 1.0, 0.0));
    let count_pixel = color.a > 0.9;
    if (is_top && count_pixel) {
        pbr_input.material.base_color = color;
    }

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}


