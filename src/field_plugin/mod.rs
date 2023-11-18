mod field;
mod material;

pub use field::Field;
pub use material::FieldMaterial;
pub use material::HighlightComponent;

use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResized,
};

pub struct FieldPlugin {
    settings: FieldSettings,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pos: IVec2,
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
    commands.spawn(Camera2dBundle::default());
    let window = windows.single();
    let field = Field::new(
        settings.dimensions,
        settings.offset,
        Vec2 {
            x: window.width(),
            y: window.height(),
        },
    );

    let texture_handle = asset_server.load("background_sky.jpg");
    let mesh = Mesh::from(shape::Quad::new(field.size()));
    let mesh_handle = meshes.add(mesh);
    let material = FieldMaterial::new(field.dimensions(), Some(texture_handle));
    let material_handle = materials.add(material);
    commands.spawn((
        MaterialMesh2dBundle {
            material: material_handle,
            mesh: mesh_handle.into(),
            transform: Transform::from_translation(field.translation().extend(0.0)),
            ..default()
        },
        field,
        HighlightComponent::new(),
        FieldId(0),
    ));
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
    mut query: Query<(&Mesh2dHandle, &mut Field)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in resize_events.read() {
        for (mesh_handle, mut field) in query.iter_mut() {
            let new_size = Vec2::new(event.width, event.height);
            field.resize(new_size);
            if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                *mesh = Mesh::from(shape::Quad::new(field.size()));
            }
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
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, (resize_listener, on_highlight_changed));
    }
}
