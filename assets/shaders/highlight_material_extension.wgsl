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
@group(1) @binding(101) var<uniform> u_HighlightListLength: i32;
@group(1) @binding(102) var<storage> u_HighlightList: array<vec2<i32>>;

fn is_highlighted(uv: vec2<f32>) -> bool {
    let flipped_uv = vec2<f32>(uv.x, 1.0 - uv.y);
    let cellIndex: vec2<i32> = vec2<i32>(flipped_uv * vec2<f32>(u_FieldSize));

    for (var i: i32 = 0; i < u_HighlightListLength; i = i + 1) {
        let highlightCell = u_HighlightList[i];
        if (cellIndex.x == highlightCell.x && cellIndex.y == highlightCell.y) {
            return true;
        }
    }
    return false;
}

@fragment
fn fragment(
    input: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(input, is_front);

    // Alpha discard
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color
    );

    var out: FragmentOutput;

    // Apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // extension
    if (is_highlighted(input.uv)) {
        out.color += vec4<f32>(0.2, 0.2, 0.2, 1.0);
    }

    // Apply in-shader post processing.
    // Ex: fog, alpha-premultiply, etc. For non-hdr cameras: tonemapping and debanding
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}

