mod components;
mod gpu_systems;
mod node;
mod resources;
mod systems;

use bevy::{
    prelude::*,
    render::{render_graph::RenderGraph, Render, RenderApp, RenderSet},
};

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_systems(Startup, systems::setup);

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            (
                gpu_systems::prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
                gpu_systems::create_buffers.in_set(RenderSet::PrepareAssets),
            ),
        );

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("game_of_life", node::SnakeNode);
        render_graph.add_node_edge(
            "game_of_life",
            bevy::render::main_graph::node::CAMERA_DRIVER,
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<resources::SnakePipeline>();
    }
}
