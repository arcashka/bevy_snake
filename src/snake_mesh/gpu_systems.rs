use bevy::{
    pbr::{
        MaterialBindGroupId, MaterialPipelineKey, MeshPipelineKey, RenderMaterialInstances,
        RenderMaterials,
    },
    prelude::*,
    render::{
        mesh::{InnerMeshVertexBufferLayout, MeshVertexBufferLayout},
        render_phase::{DrawFunctions, RenderPhase},
        render_resource::{
            BufferInitDescriptor, BufferUsages, PipelineCache, SpecializedMeshPipelines,
            VertexAttribute, VertexBufferLayout, VertexStepMode,
        },
        renderer::RenderDevice,
        view::{ExtractedView, VisibleEntities},
        Extract,
    },
};

use super::{
    components::{SnakeMesh, SnakeMeshBuffer, SnakeMeshInstance, SnakeMeshMarker},
    draw_command::DrawSnake,
    phase_item::SnakePhaseItem,
    pipeline::{SnakePipeline, SnakePipelineKey},
    resources::SnakeMeshInstances,
};

#[allow(clippy::too_many_arguments)]
pub fn queue_snake_meshes(
    snake_mesh_draw_function: Res<DrawFunctions<SnakePhaseItem>>,
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        &mut RenderPhase<SnakePhaseItem>,
    )>,
    mut pipelines: ResMut<SpecializedMeshPipelines<SnakePipeline>>,
    pipeline_cache: Res<PipelineCache>,
    snake_pipeline: Res<SnakePipeline>,
    mut snake_mesh_instances: ResMut<SnakeMeshInstances>,
    render_material_instances: Res<RenderMaterialInstances<StandardMaterial>>,
    render_materials: Res<RenderMaterials<StandardMaterial>>,
) {
    let layout = MeshVertexBufferLayout::new(InnerMeshVertexBufferLayout::new(
        [Mesh::ATTRIBUTE_POSITION.id].into(),
        VertexBufferLayout {
            array_stride: Mesh::ATTRIBUTE_POSITION.format.size(),
            step_mode: VertexStepMode::Vertex,
            attributes: [VertexAttribute {
                shader_location: 0,
                offset: 0,
                format: Mesh::ATTRIBUTE_POSITION.format,
            }]
            .into(),
        },
    ));
    let draw_snake_mesh = snake_mesh_draw_function.read().id::<DrawSnake>();
    for (view, visible_entities, mut phase) in &mut views {
        let view_key = MeshPipelineKey::from_hdr(view.hdr);
        for visible_entity in &visible_entities.entities {
            info!("Visible entity: {:?}", visible_entity);
            let Some(snake_mesh_instance) = snake_mesh_instances.get_mut(visible_entity) else {
                continue;
            };
            info!("got snake mesh instance");
            let Some(material_asset_id) = render_material_instances.get(visible_entity) else {
                continue;
            };
            info!("got material asset id");
            let Some(material) = render_materials.get(material_asset_id) else {
                continue;
            };
            info!("got material");
            snake_mesh_instance.material_bind_group_id = material.get_bind_group_id();
            let pipeline = pipelines
                .specialize(
                    &pipeline_cache,
                    &snake_pipeline,
                    SnakePipelineKey {
                        material_pipeline_key: MaterialPipelineKey {
                            mesh_key: view_key,
                            bind_group_data: material.key.clone(),
                        },
                    },
                    &layout,
                )
                .unwrap();
            info!("actually added snake phase item");
            phase.add(SnakePhaseItem {
                entity: *visible_entity,
                draw_function: draw_snake_mesh,
                pipeline,
                batch_range: 0..1,
                dynamic_offset: None,
            })
        }
    }
}

pub fn prepare_buffers(
    mut commands: Commands,
    query: Query<Entity, With<SnakeMeshMarker>>,
    render_device: Res<RenderDevice>,
) {
    for entity in query.iter() {
        let data = vec![
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(0.5, -0.5, 0.0),
            Vec3::new(0.5, 0.5, 0.0),
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(0.5, 0.5, 0.0),
            Vec3::new(-0.5, 0.5, 0.0),
        ];
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Snake vertex data buffer"),
            contents: bytemuck::cast_slice(data.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        commands.entity(entity).insert(SnakeMeshBuffer {
            buffer,
            length: data.len(),
        });
    }
}

pub fn extract_snakes(
    mut commands: Commands,
    mut snake_mesh_instances: ResMut<SnakeMeshInstances>,
    snake_query: Extract<Query<(Entity, &SnakeMesh)>>,
) {
    snake_mesh_instances.clear();
    for (entity, snake_mesh) in snake_query.iter() {
        info!("Extracting entity: {:?}", entity);
        commands.get_or_spawn(entity).insert(SnakeMeshMarker);
        snake_mesh_instances.insert(
            entity,
            SnakeMeshInstance {
                material_bind_group_id: MaterialBindGroupId::default(),
                size: snake_mesh.size,
            },
        );
    }
}

pub fn extract_snake_camera_phases(
    mut commands: Commands,
    cameras_3d: Extract<Query<(Entity, &Camera), With<Camera3d>>>,
) {
    for (entity, camera) in &cameras_3d {
        if camera.is_active {
            commands
                .get_or_spawn(entity)
                .insert((RenderPhase::<SnakePhaseItem>::default(),));
        }
    }
}
