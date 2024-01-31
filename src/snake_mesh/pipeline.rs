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

use std::hash::Hash;

#[derive(Resource)]
pub struct SnakePipeline<M: Material> {
    material_pipeline: MaterialPipeline<M>,
}

impl<M: Material> Eq for SnakePipelineKey<M> where M::Data: PartialEq {}

impl<M: Material> PartialEq for SnakePipelineKey<M>
where
    M::Data: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.material_pipeline_key == other.material_pipeline_key
    }
}

impl<M: Material> Clone for SnakePipelineKey<M>
where
    M::Data: Clone,
{
    fn clone(&self) -> Self {
        Self {
            material_pipeline_key: self.material_pipeline_key.clone(),
        }
    }
}

pub struct SnakePipelineKey<M: Material> {
    pub material_pipeline_key: MaterialPipelineKey<M>,
}

impl<M: Material> Hash for SnakePipelineKey<M>
where
    M::Data: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.material_pipeline_key.hash(state);
    }
}

impl<M: Material> Clone for SnakePipeline<M> {
    fn clone(&self) -> Self {
        Self {
            material_pipeline: self.material_pipeline.clone(),
        }
    }
}

impl<M: Material> SpecializedMeshPipeline for SnakePipeline<M>
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    type Key = SnakePipelineKey<M>;

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

impl<M: Material> FromWorld for SnakePipeline<M> {
    fn from_world(world: &mut World) -> Self {
        let material_pipeline = world.resource::<MaterialPipeline<M>>();

        info!("pipeline created from world");
        Self {
            material_pipeline: material_pipeline.clone(),
        }
    }
}
