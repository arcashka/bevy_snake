mod field;
mod plugins;
mod scene;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

fn main() {
    let default_plugins = DefaultPlugins;
    let default_plugins = default_plugins.set(RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        }),
    });
    let default_plugins = default_plugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Snake".into(),
            // This requires css html, body {margin: 0;height: 100%;} as explained https://github.com/bevyengine/bevy/pull/4726
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    });
    App::new()
        .add_plugins((
            default_plugins,
            WireframePlugin,
            plugins::HighlightMaterialPlugin,
            // FieldPlugin::new(IVec2 { x: 20, y: 20 }, Vec2 { x: 0.0, y: 0.0 }),
            // FoodPlugin,
        ))
        .add_event::<scene::SceneResizeEvent>()
        .insert_resource(field::FieldSettings {
            dimensions: IVec2 { x: 20, y: 20 },
            offset: Vec2 { x: 0.0, y: 0.0 },
        })
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::GREEN,
        })
        .add_systems(Startup, (scene::setup, field::setup).chain())
        .add_systems(
            FixedUpdate,
            (
                scene::window_events_listener,
                field::scene_resize_event_listener,
            ),
        )
        .run();
}
