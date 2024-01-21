use bevy::{
    ecs::{
        query::QueryItem,
        system::{lifetimeless::SRes, SystemParamItem},
    },
    pbr::{MaterialBindGroupId, MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        batching::GetBatchData,
        mesh::MeshVertexBufferLayout,
        render_resource::{
            RenderPipelineDescriptor, SpecializedMeshPipeline, SpecializedMeshPipelineError,
        },
    },
};

use super::{components::SnakeMeshMarker, resources::SnakeMeshInstances};

#[derive(Resource)]
pub struct SnakePipeline {
    material_pipeline: MaterialPipeline<StandardMaterial>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SnakePipelineKey {
    pub material_pipeline_key: MaterialPipelineKey<StandardMaterial>,
}

impl SpecializedMeshPipeline for SnakePipeline {
    type Key = SnakePipelineKey;

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

impl FromWorld for SnakePipeline {
    fn from_world(world: &mut World) -> Self {
        let material_pipeline = world.resource::<MaterialPipeline<StandardMaterial>>();

        info!("pipeline created from world");
        Self {
            material_pipeline: material_pipeline.clone(),
        }
    }
}

impl GetBatchData for SnakePipeline {
    type Param = SRes<SnakeMeshInstances>;
    type Query = Entity;
    type QueryFilter = With<SnakeMeshMarker>;
    type CompareData = (MaterialBindGroupId, f32);
    type BufferData = f32;

    fn get_batch_data(
        snake_mesh_instances: &SystemParamItem<Self::Param>,
        entity: &QueryItem<Self::Query>,
    ) -> (Self::BufferData, Option<Self::CompareData>) {
        let instance = snake_mesh_instances
            .get(entity)
            .expect("Failed to find Snake Mesh Instance");
        (
            instance.size,
            Some((instance.material_bind_group_id, instance.size)),
        )
    }
}
