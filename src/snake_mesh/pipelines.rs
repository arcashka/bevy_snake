use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{MaterialPipeline, MaterialPipelineKey, MeshUniform},
    prelude::*,
    render::{
        batching::GetBatchData,
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

use crate::snake_mesh::resources::SnakeMeshUniforms;

use super::{gpu_systems::DrawIndexedIndirect, resources::SnakeMeshInstances};

#[derive(Resource)]
pub struct SnakeMaterialPipeline<M: Material> {
    material_pipeline: MaterialPipeline<M>,
}

#[derive(Resource)]
pub struct SnakeComputePipeline {
    pub compute_bind_group_layout: BindGroupLayout,
    pub find_vertices_pipeline: CachedComputePipelineId,
    pub connect_vertices_pipeline: CachedComputePipelineId,
    pub prepare_indirect_buffer_pipeline: CachedComputePipelineId,
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
        Ok(descriptor)
    }
}

impl<M: Material> GetBatchData for SnakeMaterialPipeline<M> {
    type Param = SRes<SnakeMeshInstances>;
    type CompareData = ();

    type BufferData = MeshUniform;

    fn get_batch_data(
        snake_instances: &SystemParamItem<Self::Param>,
        entity: Entity,
    ) -> Option<(Self::BufferData, Option<Self::CompareData>)> {
        let snake = snake_instances.get(&entity)?;
        Some((MeshUniform::new(&snake.transforms, None), None))
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
                ShaderStages::COMPUTE,
                (
                    // Uniforms
                    binding_types::uniform_buffer::<SnakeMeshUniforms>(false),
                    // VBO
                    binding_types::storage_buffer_sized(false, NonZeroU64::new(1024)),
                    // IBO
                    binding_types::storage_buffer_sized(false, NonZeroU64::new(1024)),
                    // Cells, Intermediate buffer
                    binding_types::storage_buffer_sized(false, NonZeroU64::new(1024)),
                    // Atomics
                    binding_types::storage_buffer_sized(false, None),
                    // indirect
                    binding_types::storage_buffer::<DrawIndexedIndirect>(false),
                ),
            ),
        );

        let shader = world
            .resource::<AssetServer>()
            .load("shaders/snake_compute.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        let find_vertices_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("snake find_vertices pipeline".into()),
                layout: vec![compute_bind_group_layout.clone()],
                push_constant_ranges: Vec::new(),
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: Cow::from("find_vertices"),
            });

        let connect_vertices_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("snake connect_vertices pipeline".into()),
                layout: vec![compute_bind_group_layout.clone()],
                push_constant_ranges: Vec::new(),
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: Cow::from("connect_vertices"),
            });

        let prepare_indirect_buffer_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("snake prepare_indirect_buffer pipeline".into()),
                layout: vec![compute_bind_group_layout.clone()],
                push_constant_ranges: Vec::new(),
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: Cow::from("prepare_indirect_buffer"),
            });

        SnakeComputePipeline {
            compute_bind_group_layout,
            find_vertices_pipeline,
            connect_vertices_pipeline,
            prepare_indirect_buffer_pipeline,
        }
    }
}
