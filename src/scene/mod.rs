mod systems;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use systems::setup;

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins;
        let default_plugins = default_plugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            }),
            synchronous_pipeline_compilation: false,
        });
        let default_plugins = default_plugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".into(),
                ..default()
            }),
            ..default()
        });
        app.add_plugins((default_plugins, WireframePlugin))
            .insert_resource(WireframeConfig {
                global: false,
                default_color: Color::GREEN,
            })
            .add_systems(Startup, setup);
    }
}
