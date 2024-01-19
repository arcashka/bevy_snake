use bevy::{
    prelude::*,
    render::{render_graph, render_resource::*, renderer::RenderContext},
};

use super::{components::SnakeBuffers, resources::SnakePipeline};

#[derive(Default)]
pub struct SnakeNode {
    snake_mesh_infos: Vec<SnakeBuffers>,
}

impl render_graph::Node for SnakeNode {
    fn update(&mut self, world: &mut World) {
        self.snake_mesh_infos.clear();
        for snake_mesh_info in world.query::<&SnakeBuffers>().iter(world) {
            info!("Node run and found mesh");
            self.snake_mesh_infos.push(snake_mesh_info.clone());
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        for snake_mesh_info in &self.snake_mesh_infos {
            info!("Node run and found mesh");
            if let Some(texture_bind_group) = snake_mesh_info.bind_group.clone() {
                let pipeline_cache = world.resource::<PipelineCache>();
                let pipeline = world.resource::<SnakePipeline>();

                let mut pass = render_context
                    .command_encoder()
                    .begin_compute_pass(&ComputePassDescriptor::default());

                pass.set_bind_group(0, &texture_bind_group, &[]);

                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(1, 1, 1);
            }
        }

        Ok(())
    }
}
