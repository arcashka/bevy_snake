mod material;

pub use material::FieldMaterial;
pub use material::HighlightComponent;

use crate::system_sets::GameSystemSets;

use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResized,
};

pub struct FieldPlugin {
    settings: FieldSettings,
}

#[derive(Component)]
pub struct Field {
    pub dimensions: IVec2,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub pos: IVec2,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: IVec2::new(x, y),
        }
    }

    pub fn i(&self) -> i32 {
        self.pos.x
    }

    pub fn j(&self) -> i32 {
        self.pos.y
    }
}

impl FieldPlugin {
    pub fn new(dimensions: IVec2, offset: Vec2) -> Self {
        Self {
            settings: FieldSettings { dimensions, offset },
        }
    }
}

#[derive(Component, Clone, Copy, Eq, PartialEq, Debug)]
pub struct FieldId(pub i32);
fn setup(
    mut commands: Commands,
    windows: Query<&Window>,
    settings: Res<FieldSettings>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FieldMaterial>>,
) {
    info!("creating field");
    commands.spawn(Camera2dBundle::default());
    let window = windows.single();
    let dim = settings.dimensions;
    let scale = (window.width() / dim.x as f32).min(window.height() / dim.y as f32);

    let field = Field { dimensions: dim };

    let texture_handle = asset_server.load("background_sky.jpg");
    let material = FieldMaterial::new(settings.dimensions, Some(texture_handle));
    let material_handle = materials.add(material);

    let mesh = Mesh::from(shape::Quad::new(dim.as_vec2()));
    let mesh_handle = meshes.add(mesh);
    let field_entity = commands
        .spawn((
            MaterialMesh2dBundle {
                material: material_handle,
                mesh: mesh_handle.into(),
                transform: Transform::from_translation(settings.offset.extend(0.0))
                    .with_scale(Vec3::splat(scale)),
                ..default()
            },
            field,
            HighlightComponent::new(),
            FieldId(0),
        ))
        .id();
    info!("field created: {:?}", field_entity);
}

type HighlightChanged = Or<(Changed<HighlightComponent>, Added<HighlightComponent>)>;
fn on_highlight_changed(
    mut query: Query<(&Handle<FieldMaterial>, &HighlightComponent), HighlightChanged>,
    mut materials: ResMut<Assets<FieldMaterial>>,
) {
    for (field_material_handle, highlight) in query.iter_mut() {
        if let Some(material) = materials.get_mut(field_material_handle.id()) {
            material.set_highlighted(highlight.highlighted());
        }
    }
}

fn resize_listener(
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<(&mut Transform, &Field)>,
) {
    for event in resize_events.read() {
        for (mut transform, field) in query.iter_mut() {
            let dim = field.dimensions;
            let scale = (event.width / dim.x as f32).min(event.height / dim.y as f32);
            transform.scale = Vec3::splat(scale);
        }
    }
}

#[derive(Resource, Clone, Copy)]
pub struct FieldSettings {
    pub dimensions: IVec2,
    pub offset: Vec2,
}

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings)
            .add_plugins(Material2dPlugin::<FieldMaterial>::default())
            .add_systems(Startup, setup.in_set(GameSystemSets::FieldSetup))
            .add_systems(FixedUpdate, (resize_listener, on_highlight_changed));
    }
}
