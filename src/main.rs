mod field_plugin;
mod food_plugin;
mod player_plugin;
mod system_sets;

use field_plugin::FieldPlugin;
use food_plugin::FoodPlugin;
use player_plugin::PlayerPlugin;
use system_sets::GameSystemSets;

use bevy::{
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

fn main() {
    App::new()
        .configure_sets(
            Startup,
            GameSystemSets::PlayerSetup.after(GameSystemSets::FieldSetup),
        )
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
            }),
            // FieldPlugin::new(IVec2 { x: 20, y: 20 }, Vec2 { x: 0.0, y: 0.0 }),
            // FoodPlugin,
            PlayerPlugin { speed: 3.0 },
        ))
        .run();
}
