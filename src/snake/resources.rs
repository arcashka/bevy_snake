use bevy::prelude::*;
use bevy::render::{render_resource::*, renderer::RenderDevice};
use std::borrow::Cow;

#[derive(Resource)]
pub struct SnakeBindGroup(pub BindGroup);

#[derive(Resource)]
pub struct SnakeBuffers {
    pub storage_buffer: Buffer,
    pub uniform_buffer: Buffer,
}

#[derive(Resource)]
pub struct SnakePipeline {
    pub bind_group_layout: BindGroupLayout,
    pub pipeline: CachedComputePipelineId,
}

impl FromWorld for SnakePipeline {
    fn from_world(world: &mut World) -> Self {
        let bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });
        let shader = world.resource::<AssetServer>().load("shaders/snake.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("main"),
        });

        Self {
            bind_group_layout,
            pipeline,
        }
    }
}
