@group(0) @binding(0) var<uniform> size: f32;
@group(0) @binding(1) var<storage, read_write> vertices: array<f32>;

fn get_vertex(index: u32) -> vec3f {
  let offset = index * 3;
  return vec3f(vertices[offset],
               vertices[offset + 1],
               vertices[offset + 2]);
}

// because vec3f has 16 bytes alighnment
fn set_vertex(index: u32, value: vec3f) {
  let offset = index * 3;
  vertices[offset] = value.x;
  vertices[offset + 1] = value.y;
  vertices[offset + 2] = value.z;
}

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    set_vertex(0u, vec3<f32>(0.0, size, size));
    set_vertex(1u, vec3<f32>(0.0, -size, size));
    set_vertex(2u, vec3<f32>(0.0, -size, -size));
    set_vertex(3u, vec3<f32>(0.0, size, -size));
    set_vertex(4u, vec3<f32>(0.0, size, size));
    set_vertex(5u, vec3<f32>(0.0, -size, -size));
}

