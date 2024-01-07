use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;

use super::resources::Field;
use crate::plugins::TiledMaterialExtension;

pub fn setup(
    mut commands: Commands,
    field: Res<Field>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut extended_materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, TiledMaterialExtension>>,
    >,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("field setup called");
    info!("field size: {:?}", field.size());
    info!("field offset: {:?}", field.offset());
    info!("field cell_size: {:?}", field.cell_size());
    info!("field dim: {:?}", field.dim());
    let grass_border: Handle<Image> = asset_server.load("grass_border.png");
    let material = ExtendedMaterial::<StandardMaterial, TiledMaterialExtension> {
        base: StandardMaterial::from(Color::SALMON),
        extension: TiledMaterialExtension::new(field.dim(), grass_border),
    };
    let material_handle = extended_materials.add(material);

    let top_size = 2.0;
    let base_size = 2.0;
    let mesh_top_handle = meshes.add(Mesh::from(shape::Box::new(
        field.size().x,
        top_size,
        field.size().y,
    )));
    let mesh_base_handle = meshes.add(Mesh::from(shape::Box::new(
        field.size().x,
        base_size,
        field.size().y,
    )));

    commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            parent.spawn(MaterialMeshBundle {
                material: material_handle,
                mesh: mesh_top_handle,
                transform: Transform::from_translation(Vec3::new(
                    field.offset().x,
                    -top_size / 2.0,
                    field.offset().y,
                )),
                ..default()
            });
            parent.spawn(MaterialMeshBundle {
                material: standard_materials.add(StandardMaterial::from(Color::WHITE)),
                mesh: mesh_base_handle,
                transform: Transform::from_translation(Vec3::new(
                    field.offset().x,
                    -(base_size + top_size) / 2.0,
                    field.offset().y,
                )),
                ..default()
            });
        });
}
