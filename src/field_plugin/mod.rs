mod field;
mod material;

pub use field::Field;
pub use material::FieldMaterial;

use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResized,
};

pub struct FieldPlugin {
    settings: FieldSettings,
}

impl FieldPlugin {
    pub fn new(dimensions: IVec2, offset: Vec2) -> Self {
        Self {
            settings: FieldSettings { dimensions, offset },
        }
    }
}

#[derive(Component)]
pub struct GridId(i32);
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

    let texture_handle = asset_server.load("grass.png");
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
        GridId(0),
    ));
}

//type FieldChangedOrAdded = Or<(Changed<Field>, Added<Field>)>;
//fn on_field_changed(
//    mut query: Query<
//        (
//            &mut Transform,
//            &Mesh2dHandle,
//            &Handle<FieldMaterial>,
//            &Field,
//        ),
//        FieldChangedOrAdded,
//    >,
//    mut meshes: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<FieldMaterial>>,
//) {
//    for (mut transform, field_mesh_handle, field_material_handle, field) in query.iter_mut() {
//        if let Some(mesh) = meshes.get_mut(&field_mesh_handle.0) {
//            *mesh = Mesh::from(shape::Quad::new(field.size()));
//        }
//        // Not actually needed
//        if let Some(material) = materials.get_mut(field_material_handle.id()) {
//            info!("material cell new size: {:?}", field.cell_size);
//            material.update_size(field.dimensions);
//        }
//    }
//}

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
            .add_systems(FixedUpdate, resize_listener);
    }
}
