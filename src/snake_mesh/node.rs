use bevy::{
    prelude::*,
    render::{
        render_graph::{self, RenderGraphContext, RenderLabel},
        render_resource::{ComputePassDescriptor, PipelineCache},
        renderer::RenderContext,
    },
};

use super::{pipelines::SnakeComputePipeline, resources::SnakeMeshInstances};

#[derive(Default)]
pub struct SnakeComputeNode;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct SnakeComputeNodeLabel;

impl render_graph::Node for SnakeComputeNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let compute_pipeline = world.resource::<SnakeComputePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        let Some(pipeline) = pipeline_cache.get_compute_pipeline(compute_pipeline.pipeline) else {
            return Ok(());
        };
        pass.set_pipeline(pipeline);

        let snakes = world.resource::<SnakeMeshInstances>();
        for (_, snake) in snakes.iter() {
            let Some(bind_group) = snake.compute_bind_group.as_ref() else {
                error!("missing snake compute bind group");
                return Ok(());
            };
            pass.set_bind_group(0, bind_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        Ok(())
    }
}
