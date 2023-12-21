use std::f32::consts::PI;

use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;

use crate::plugins::HighlightMaterialExtension;
use crate::scene::SceneResizeEvent;

use super::Field;
use super::FieldId;
use super::FieldSettings;
use super::HighlightComponent;

pub fn setup(
    mut commands: Commands,
    settings: Res<FieldSettings>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, HighlightMaterialExtension>>>,
) {
    let dim = settings.dimensions;

    let field = Field { dimensions: dim };

    let material = ExtendedMaterial::<StandardMaterial, HighlightMaterialExtension> {
        base: StandardMaterial::from(asset_server.load("background_sky.jpg")),
        extension: HighlightMaterialExtension::new(settings.dimensions),
    };
    let material_handle = materials.add(material);

    let mesh = Mesh::from(shape::Quad::new(dim.as_vec2()));
    let mesh_handle = meshes.add(mesh);
    let field_entity = commands
        .spawn((
            MaterialMeshBundle {
                material: material_handle,
                mesh: mesh_handle,
                transform: Transform::from_translation(settings.offset.extend(0.0))
                    .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
                ..default()
            },
            field,
            HighlightComponent::new(),
            FieldId(0),
        ))
        .id();
    info!("field created: {:?}", field_entity);
}

pub fn scene_resize_event_listener(
    mut resize_events: EventReader<SceneResizeEvent>,
    mut field_query: Query<(&mut Transform, &Field)>,
) {
    for resize_event in resize_events.read() {
        let new_size = resize_event.size;
        for (mut transform, field) in field_query.iter_mut() {
            let dim = field.dimensions;
            let scale = (new_size.x / dim.x as f32).min(new_size.y / dim.y as f32);
            transform.scale = Vec3::splat(scale);
        }
    }
}
