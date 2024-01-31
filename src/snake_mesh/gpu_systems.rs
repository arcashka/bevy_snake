use std::hash::Hash;

use bevy::{
    core_pipeline::{
        core_3d::{AlphaMask3d, Opaque3d, Transmissive3d, Transparent3d},
        prepass::{DeferredPrepass, DepthPrepass, MotionVectorPrepass, NormalPrepass},
        tonemapping::{DebandDither, Tonemapping},
    },
    pbr::{
        alpha_mode_pipeline_key, environment_map::RenderViewEnvironmentMaps,
        screen_space_specular_transmission_pipeline_key, tonemapping_pipeline_key,
        MaterialBindGroupId, MaterialPipelineKey, MeshFlags, MeshPipelineKey, MeshTransforms,
        NotShadowReceiver, OpaqueRendererMethod, PreviousGlobalTransform, RenderMaterialInstances,
        RenderMaterials, ScreenSpaceAmbientOcclusionSettings, ShadowFilteringMethod,
        TransmittedShadowReceiver,
    },
    prelude::*,
    render::{
        camera::TemporalJitter,
        mesh::{InnerMeshVertexBufferLayout, MeshVertexBufferLayout},
        render_phase::{DrawFunctions, RenderPhase},
        render_resource::{
            BufferInitDescriptor, BufferUsages, PipelineCache, PrimitiveTopology,
            SpecializedMeshPipelines, VertexAttribute, VertexBufferLayout, VertexStepMode,
        },
        renderer::RenderDevice,
        view::{ExtractedView, VisibleEntities},
        Extract,
    },
};

use super::{
    components::{SnakeMesh, SnakeMeshInstance},
    draw_command::DrawSnake,
    pipeline::{SnakePipeline, SnakePipelineKey},
    resources::SnakeMeshInstances,
};

#[allow(clippy::too_many_arguments)]
pub fn queue_material_snakes<M: Material>(
    opaque_draw_functions: Res<DrawFunctions<Opaque3d>>,
    alpha_mask_draw_functions: Res<DrawFunctions<AlphaMask3d>>,
    transmissive_draw_functions: Res<DrawFunctions<Transmissive3d>>,
    transparent_draw_functions: Res<DrawFunctions<Transparent3d>>,
    material_pipeline: Res<SnakePipeline<M>>,
    mut pipelines: ResMut<SpecializedMeshPipelines<SnakePipeline<M>>>,
    pipeline_cache: Res<PipelineCache>,
    msaa: Res<Msaa>,
    render_materials: Res<RenderMaterials<M>>,
    mut snake_mesh_instances: ResMut<SnakeMeshInstances>,
    render_material_instances: Res<RenderMaterialInstances<M>>,
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        Option<&Tonemapping>,
        Option<&DebandDither>,
        Option<&ShadowFilteringMethod>,
        Has<ScreenSpaceAmbientOcclusionSettings>,
        (
            Has<NormalPrepass>,
            Has<DepthPrepass>,
            Has<MotionVectorPrepass>,
            Has<DeferredPrepass>,
        ),
        Option<&Camera3d>,
        Has<TemporalJitter>,
        Option<&Projection>,
        &mut RenderPhase<Opaque3d>,
        &mut RenderPhase<AlphaMask3d>,
        &mut RenderPhase<Transmissive3d>,
        &mut RenderPhase<Transparent3d>,
        Has<RenderViewEnvironmentMaps>,
    )>,
) where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    for (
        view,
        visible_entities,
        tonemapping,
        dither,
        shadow_filter_method,
        ssao,
        (normal_prepass, depth_prepass, motion_vector_prepass, deferred_prepass),
        camera_3d,
        temporal_jitter,
        projection,
        mut opaque_phase,
        mut alpha_mask_phase,
        mut transmissive_phase,
        mut transparent_phase,
        has_environment_maps,
    ) in &mut views
    {
        let draw_opaque_pbr = opaque_draw_functions.read().id::<DrawSnake<M>>();
        let draw_alpha_mask_pbr = alpha_mask_draw_functions.read().id::<DrawSnake<M>>();
        let draw_transmissive_pbr = transmissive_draw_functions.read().id::<DrawSnake<M>>();
        let draw_transparent_pbr = transparent_draw_functions.read().id::<DrawSnake<M>>();

        let mut view_key = MeshPipelineKey::from_msaa_samples(msaa.samples())
            | MeshPipelineKey::from_hdr(view.hdr);

        if normal_prepass {
            view_key |= MeshPipelineKey::NORMAL_PREPASS;
        }

        if depth_prepass {
            view_key |= MeshPipelineKey::DEPTH_PREPASS;
        }

        if motion_vector_prepass {
            view_key |= MeshPipelineKey::MOTION_VECTOR_PREPASS;
        }

        if deferred_prepass {
            view_key |= MeshPipelineKey::DEFERRED_PREPASS;
        }

        if temporal_jitter {
            view_key |= MeshPipelineKey::TEMPORAL_JITTER;
        }

        if has_environment_maps {
            view_key |= MeshPipelineKey::ENVIRONMENT_MAP;
        }

        if let Some(projection) = projection {
            view_key |= match projection {
                Projection::Perspective(_) => MeshPipelineKey::VIEW_PROJECTION_PERSPECTIVE,
                Projection::Orthographic(_) => MeshPipelineKey::VIEW_PROJECTION_ORTHOGRAPHIC,
            };
        }

        match shadow_filter_method.unwrap_or(&ShadowFilteringMethod::default()) {
            ShadowFilteringMethod::Hardware2x2 => {
                view_key |= MeshPipelineKey::SHADOW_FILTER_METHOD_HARDWARE_2X2;
            }
            ShadowFilteringMethod::Castano13 => {
                view_key |= MeshPipelineKey::SHADOW_FILTER_METHOD_CASTANO_13;
            }
            ShadowFilteringMethod::Jimenez14 => {
                view_key |= MeshPipelineKey::SHADOW_FILTER_METHOD_JIMENEZ_14;
            }
        }

        if !view.hdr {
            if let Some(tonemapping) = tonemapping {
                view_key |= MeshPipelineKey::TONEMAP_IN_SHADER;
                view_key |= tonemapping_pipeline_key(*tonemapping);
            }
            if let Some(DebandDither::Enabled) = dither {
                view_key |= MeshPipelineKey::DEBAND_DITHER;
            }
        }
        if ssao {
            view_key |= MeshPipelineKey::SCREEN_SPACE_AMBIENT_OCCLUSION;
        }
        if let Some(camera_3d) = camera_3d {
            view_key |= screen_space_specular_transmission_pipeline_key(
                camera_3d.screen_space_specular_transmission_quality,
            );
        }
        let rangefinder = view.rangefinder3d();
        for visible_entity in &visible_entities.entities {
            let Some(material_asset_id) = render_material_instances.get(visible_entity) else {
                continue;
            };
            let Some(snake_instance) = snake_mesh_instances.get_mut(visible_entity) else {
                continue;
            };
            let Some(material) = render_materials.get(material_asset_id) else {
                continue;
            };

            let forward = match material.properties.render_method {
                OpaqueRendererMethod::Forward => true,
                OpaqueRendererMethod::Deferred => false,
                OpaqueRendererMethod::Auto => unreachable!(),
            };

            let mut mesh_key = view_key;

            mesh_key |= MeshPipelineKey::from_primitive_topology(PrimitiveTopology::TriangleList);

            if material.properties.reads_view_transmission_texture {
                mesh_key |= MeshPipelineKey::READS_VIEW_TRANSMISSION_TEXTURE;
            }

            mesh_key |= alpha_mode_pipeline_key(material.properties.alpha_mode);

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
            let pipeline_id = pipelines.specialize(
                &pipeline_cache,
                &material_pipeline,
                SnakePipelineKey {
                    material_pipeline_key: MaterialPipelineKey {
                        mesh_key,
                        bind_group_data: material.key.clone(),
                    },
                },
                &layout,
            );
            let pipeline_id = match pipeline_id {
                Ok(id) => id,
                Err(err) => {
                    error!("{}", err);
                    continue;
                }
            };

            snake_instance.material_bind_group_id = material.get_bind_group_id();

            let distance = rangefinder
                .distance_translation(&snake_instance.transforms.transform.translation)
                + material.properties.depth_bias;
            match material.properties.alpha_mode {
                AlphaMode::Opaque => {
                    if material.properties.reads_view_transmission_texture {
                        transmissive_phase.add(Transmissive3d {
                            entity: *visible_entity,
                            draw_function: draw_transmissive_pbr,
                            pipeline: pipeline_id,
                            distance,
                            batch_range: 0..1,
                            dynamic_offset: None,
                        });
                    } else if forward {
                        opaque_phase.add(Opaque3d {
                            entity: *visible_entity,
                            draw_function: draw_opaque_pbr,
                            pipeline: pipeline_id,
                            distance,
                            batch_range: 0..1,
                            dynamic_offset: None,
                        });
                    }
                }
                AlphaMode::Mask(_) => {
                    if material.properties.reads_view_transmission_texture {
                        transmissive_phase.add(Transmissive3d {
                            entity: *visible_entity,
                            draw_function: draw_transmissive_pbr,
                            pipeline: pipeline_id,
                            distance,
                            batch_range: 0..1,
                            dynamic_offset: None,
                        });
                    } else if forward {
                        alpha_mask_phase.add(AlphaMask3d {
                            entity: *visible_entity,
                            draw_function: draw_alpha_mask_pbr,
                            pipeline: pipeline_id,
                            distance,
                            batch_range: 0..1,
                            dynamic_offset: None,
                        });
                    }
                }
                AlphaMode::Blend
                | AlphaMode::Premultiplied
                | AlphaMode::Add
                | AlphaMode::Multiply => {
                    transparent_phase.add(Transparent3d {
                        entity: *visible_entity,
                        draw_function: draw_transparent_pbr,
                        pipeline: pipeline_id,
                        distance,
                        batch_range: 0..1,
                        dynamic_offset: None,
                    });
                }
            }
        }
    }
}

pub fn prepare_buffers(
    render_device: Res<RenderDevice>,
    mut snake_mesh_instances: ResMut<SnakeMeshInstances>,
) {
    for (_, snake) in snake_mesh_instances.iter_mut() {
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
        snake.buffer = Some(buffer);
        snake.buffer_length = 6;
    }
}

pub fn extract_snakes(
    mut commands: Commands,
    mut snake_mesh_instances: ResMut<SnakeMeshInstances>,
    snake_query: Extract<
        Query<(
            Entity,
            &SnakeMesh,
            &ViewVisibility,
            &GlobalTransform,
            Option<&PreviousGlobalTransform>,
            Has<NotShadowReceiver>,
            Has<TransmittedShadowReceiver>,
        )>,
    >,
) {
    snake_mesh_instances.clear();
    for (
        entity,
        snake_mesh,
        view_visibility,
        transform,
        previous_transform,
        not_shadow_receiver,
        transmitted_receiver,
    ) in snake_query.iter()
    {
        if !view_visibility.get() {
            return;
        }
        let transform = transform.affine();
        let previous_transform = previous_transform.map(|t| t.0).unwrap_or(transform);
        let mut flags = if not_shadow_receiver {
            MeshFlags::empty()
        } else {
            MeshFlags::SHADOW_RECEIVER
        };
        if transmitted_receiver {
            flags |= MeshFlags::TRANSMITTED_SHADOW_RECEIVER;
        }
        if transform.matrix3.determinant().is_sign_positive() {
            flags |= MeshFlags::SIGN_DETERMINANT_MODEL_3X3;
        }
        let transforms = MeshTransforms {
            transform: (&transform).into(),
            previous_transform: (&previous_transform).into(),
            flags: flags.bits(),
        };
        commands.get_or_spawn(entity);
        snake_mesh_instances.insert(
            entity,
            SnakeMeshInstance {
                material_bind_group_id: MaterialBindGroupId::default(),
                size: snake_mesh.size,
                buffer: None,
                buffer_length: 0,
                transforms,
            },
        );
    }
}
