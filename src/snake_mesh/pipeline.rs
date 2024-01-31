use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            RenderPipelineDescriptor, SpecializedMeshPipeline, SpecializedMeshPipelineError,
        },
    },
};

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
