use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;

use crate::plugins::TiledMaterialExtension;

use super::Field;
use super::FieldId;
use super::FieldSettings;

pub fn setup(
    mut commands: Commands,
    settings: Res<FieldSettings>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut extended_materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, TiledMaterialExtension>>,
    >,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    let dim = settings.dimensions;
    let offset = settings.offset;
    let size = settings.size;

    let field = Field {
        dimensions: dim,
        size,
        offset,
    };

    let grass_border: Handle<Image> = asset_server.load("grass_border.png");
    let material = ExtendedMaterial::<StandardMaterial, TiledMaterialExtension> {
        base: StandardMaterial::from(Color::SALMON),
        extension: TiledMaterialExtension::new(dim, grass_border),
    };
    let material_handle = extended_materials.add(material);

    let top_size = 2.0;
    let base_size = 2.0;
    let mesh_top_handle = meshes.add(Mesh::from(shape::Box::new(size.x, top_size, size.y)));
    let mesh_base_handle = meshes.add(Mesh::from(shape::Box::new(size.x, base_size, size.y)));

    commands
        .spawn((SpatialBundle::default(), field, FieldId(0)))
        .with_children(|parent| {
            parent.spawn(MaterialMeshBundle {
                material: material_handle,
                mesh: mesh_top_handle,
                transform: Transform::from_translation(Vec3::new(
                    offset.x,
                    -top_size / 2.0,
                    offset.y,
                )),
                ..default()
            });
            parent.spawn(MaterialMeshBundle {
                material: standard_materials.add(StandardMaterial::from(Color::WHITE)),
                mesh: mesh_base_handle,
                transform: Transform::from_translation(Vec3::new(
                    offset.x,
                    -(base_size + top_size) / 2.0,
                    offset.y,
                )),
                ..default()
            });
        });
}
