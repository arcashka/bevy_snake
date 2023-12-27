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

    let field = Field { dimensions: dim };

    let grass_border: Handle<Image> = asset_server.load("grass_border.png");
    let material = ExtendedMaterial::<StandardMaterial, TiledMaterialExtension> {
        base: StandardMaterial::from(Color::SALMON),
        extension: TiledMaterialExtension::new(settings.dimensions, grass_border),
    };
    let material_handle = extended_materials.add(material);

    let top_size = 2.0;
    let base_size = 2.0;
    let mesh_top = Mesh::from(shape::Box::new(dim.x as f32, top_size, dim.y as f32));
    let mesh_base = Mesh::from(shape::Box::new(dim.x as f32, base_size, dim.y as f32));
    let mesh_top_handle = meshes.add(mesh_top);
    let mesh_base_handle = meshes.add(mesh_base);

    commands
        .spawn((SpatialBundle::default(), field, FieldId(0)))
        .with_children(|parent| {
            parent.spawn(MaterialMeshBundle {
                material: material_handle,
                mesh: mesh_top_handle,
                transform: Transform::from_translation(Vec3::new(
                    settings.offset.x,
                    -top_size / 2.0,
                    settings.offset.y,
                )),
                //.with_rotation(Quat::from_rotation_z(PI / 2.0)),
                ..default()
            });
            parent.spawn(MaterialMeshBundle {
                material: standard_materials.add(StandardMaterial::from(Color::WHITE)),
                mesh: mesh_base_handle,
                transform: Transform::from_translation(Vec3::new(
                    settings.offset.x,
                    -(base_size + top_size) / 2.0,
                    settings.offset.y,
                )),
                //.with_rotation(Quat::from_rotation_x(PI / 2.0)),
                ..default()
            });
        });
    //     commands.spawn(MaterialMeshBundle {
    //         material: standard_materials.add(StandardMaterial::from(Color::WHITE)),
    //         mesh: mesh_base_handle,
    //         transform: Transform::from_translation(Vec3::new(
    //             settings.offset.x,
    //             -0.75,
    //             settings.offset.y,
    //         ))
    //         .with_rotation(Quat::from_rotation_x(0.0)),
    //         ..default()
    //     });
}
