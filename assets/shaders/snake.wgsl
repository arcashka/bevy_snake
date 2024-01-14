struct MyBuffer {
    data: array<f32>,
};

@group(0) @binding(0) var<uniform> size: f32;
@group(0) @binding(1) var<storage, read_write> buffer: MyBuffer;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    buffer.data[0] = 5.0;
    buffer.data[1] = 6.0;
    buffer.data[2] = 7.0;
    buffer.data[3] = 7.0;
    buffer.data[4] = size;
}
