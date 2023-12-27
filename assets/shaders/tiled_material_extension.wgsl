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

fn modulo_euclidean(a: f32, b: f32) -> f32 {
	var m = a % b;
	if (m < 0.0) {
		if (b < 0.0) {
			m -= b;
		} else {
			m += b;
		}
	}
	return m;
}

@fragment
fn fragment(
    input: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(input, is_front);

    let new_uv_x = modulo_euclidean(input.uv.x * f32(u_FieldSize.x), 1.0);
    let new_uv_y = modulo_euclidean(input.uv.y * f32(u_FieldSize.y), 1.0);
    let new_uv = vec2<f32>(new_uv_x, new_uv_y);
    let color = textureSample(tile_texture, tile_sampler, new_uv);

    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color
    );
    if (color.a > 0.9) {
        pbr_input.material.base_color = color;
    }

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}


