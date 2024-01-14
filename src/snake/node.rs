use bevy::{
    prelude::*,
    render::{render_graph, render_resource::*, renderer::RenderContext},
};

use super::resources::{SnakeBindGroup, SnakePipeline};

pub struct SnakeNode;

impl render_graph::Node for SnakeNode {
    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<SnakeBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<SnakePipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        let init_pipeline = pipeline_cache
            .get_compute_pipeline(pipeline.pipeline)
            .unwrap();
        pass.set_pipeline(init_pipeline);
        pass.dispatch_workgroups(1, 1, 1);

        Ok(())
    }
}
