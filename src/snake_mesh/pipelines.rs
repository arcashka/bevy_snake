use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            binding_types, BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId,
            ComputePipelineDescriptor, PipelineCache, RenderPipelineDescriptor, ShaderStages,
            SpecializedMeshPipeline, SpecializedMeshPipelineError,
        },
        renderer::RenderDevice,
    },
};

use std::{borrow::Cow, hash::Hash, num::NonZeroU64};

#[derive(Resource)]
pub struct SnakeMaterialPipeline<M: Material> {
    material_pipeline: MaterialPipeline<M>,
}

#[derive(Resource)]
pub struct SnakeComputePipeline {
    pub compute_bind_group_layout: BindGroupLayout,
    pub pipeline: CachedComputePipelineId,
}

impl<M: Material> Eq for SnakeMaterialPipelineKey<M> where M::Data: PartialEq {}

impl<M: Material> PartialEq for SnakeMaterialPipelineKey<M>
where
    M::Data: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.material_pipeline_key == other.material_pipeline_key
    }
}

impl<M: Material> Clone for SnakeMaterialPipelineKey<M>
where
    M::Data: Clone,
{
    fn clone(&self) -> Self {
        Self {
            material_pipeline_key: self.material_pipeline_key.clone(),
        }
    }
}

pub struct SnakeMaterialPipelineKey<M: Material> {
    pub material_pipeline_key: MaterialPipelineKey<M>,
}

impl<M: Material> Hash for SnakeMaterialPipelineKey<M>
where
    M::Data: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.material_pipeline_key.hash(state);
    }
}

impl<M: Material> Clone for SnakeMaterialPipeline<M> {
    fn clone(&self) -> Self {
        Self {
            material_pipeline: self.material_pipeline.clone(),
        }
    }
}

impl<M: Material> SpecializedMeshPipeline for SnakeMaterialPipeline<M>
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    type Key = SnakeMaterialPipelineKey<M>;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let descriptor = self
            .material_pipeline
            .specialize(key.material_pipeline_key, layout)?;
        info!("pipeline created");
        Ok(descriptor)
    }
}

impl<M: Material> FromWorld for SnakeMaterialPipeline<M> {
    fn from_world(world: &mut World) -> Self {
        let material_pipeline = world.resource::<MaterialPipeline<M>>();
        Self {
            material_pipeline: material_pipeline.clone(),
        }
    }
}

impl FromWorld for SnakeComputePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let compute_bind_group_layout = render_device.create_bind_group_layout(
            "snake compute bind group layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::VERTEX | ShaderStages::COMPUTE,
                (
                    binding_types::uniform_buffer::<f32>(false),
                    binding_types::storage_buffer_sized(false, NonZeroU64::new(1024)),
                ),
            ),
        );

        let shader = world
            .resource::<AssetServer>()
            .load("shaders/snake_compute.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("snake compute pipeline".into()),
            layout: vec![compute_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("main"),
        });

        SnakeComputePipeline {
            compute_bind_group_layout,
            pipeline,
        }
    }
}
