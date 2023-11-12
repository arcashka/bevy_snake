#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(0) var<uniform> u_FieldSize: vec2<i32>;
@group(1) @binding(1) var<uniform> u_HighlightListLength: i32;
@group(1) @binding(2) var<storage> u_HighlightList: array<vec2<i32>>;

@group(1) @binding(3) var base_color_texture: texture_2d<f32>;
@group(1) @binding(4) var base_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    // WGPU and bevy have coordinate system
    let flipped_uv = vec2<f32>(uv.x, 1.0 - uv.y);
    let cellIndex: vec2<i32> = vec2<i32>(flipped_uv * vec2<f32>(u_FieldSize));

    var isHighlighted: bool = false;
    for (var i: i32 = 0; i < u_HighlightListLength; i = i + 1) {
        let highlightCell = u_HighlightList[i];
        if (cellIndex.x == highlightCell.x && cellIndex.y == highlightCell.y) {
            isHighlighted = true;
            break;
        }
    }

    var color: vec4<f32>;
    if (isHighlighted) {
        color = vec4<f32>(0.2, 0.2, 0.2, 1.0);
    } else {
        color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    return color + (textureSample(base_color_texture, base_color_sampler, mesh.uv));
}
