use bevy::{
    ecs::system::{
        lifetimeless::{SCommands, SRes},
        SystemParamItem,
    },
    prelude::*,
    render::{
        mesh::{GpuBufferInfo, GpuMesh, InnerMeshVertexBufferLayout, Mesh, MeshVertexBufferLayout},
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            BufferDescriptor, BufferUsages, IndexFormat, PrimitiveTopology, VertexAttribute,
            VertexBufferLayout, VertexStepMode,
        },
        renderer::RenderDevice,
    },
};

use super::components::SnakeBuffers;
use super::SnakeMesh;

impl RenderAsset for SnakeMesh {
    type ExtractedAsset = SnakeMesh;
    type PreparedAsset = GpuMesh;
    type Param = (SRes<RenderDevice>, SCommands);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        info!("extract asset called");
        self.clone()
    }

    fn prepare_asset(
        mesh: Self::ExtractedAsset,
        (render_device, ref mut commands): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        info!("prepare asset called");
        let vertex_buffer = render_device.create_buffer(&BufferDescriptor {
            size: 1024,
            mapped_at_creation: false,
            label: None,
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX,
        });
        let index_buffer = render_device.create_buffer(&BufferDescriptor {
            size: 1024,
            mapped_at_creation: false,
            label: None,
            usage: BufferUsages::STORAGE | BufferUsages::INDEX,
        });

        info!("buffers created");
        commands.spawn((
            mesh,
            SnakeBuffers {
                index_buffer: index_buffer.clone(),
                vertex_buffer: vertex_buffer.clone(),
                uniform_buffer: None,
                bind_group: None,
            },
        ));
        info!("SnakeBuffers spawned");

        let buffer_info = GpuBufferInfo::Indexed {
            buffer: index_buffer,
            count: 6,
            index_format: IndexFormat::Uint32,
        };

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
        info!("layout created");

        Ok(GpuMesh {
            vertex_buffer,
            buffer_info,
            vertex_count: 4,
            primitive_topology: PrimitiveTopology::TriangleList,
            morph_targets: None,
            layout,
        })
    }
}
