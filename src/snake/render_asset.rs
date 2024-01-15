use bevy::{
    ecs::system::{
        lifetimeless::{SCommands, SRes},
        SystemParamItem,
    },
    render::{
        mesh::{GpuBufferInfo, GpuMesh, Mesh},
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{BufferDescriptor, BufferUsages, IndexFormat, PrimitiveTopology, VertexAttribute, VertexFormat},
        renderer::RenderDevice,
    },
};

use super::components::SnakeMeshBuffers;
use super::SnakeMesh;

impl RenderAsset for SnakeMesh {
    type ExtractedAsset = SnakeMesh;
    type PreparedAsset = GpuMesh;
    type Param = (SRes<RenderDevice>, SCommands);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        mesh: Self::ExtractedAsset,
        (render_device, mut commands): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
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

        commands.spawn((
            mesh,
            SnakeMeshBuffers {
                index_buffer,
                vertex_buffer,
            },
        ));

        let buffer_info = GpuBufferInfo::Indexed {
            buffer: index_buffer,
            count: 6,
            index_format: IndexFormat::Uint32,
        };

        let a = Mesh::ATTRIBUTE_POSITION;
        let attributes = [
            VertexAttribute {
                shader_location: 0,
                offset: 0,
                format: VertexFormat::Float32x3,
            },
            VertexAttribute {
                shader_location: 1,
                offset: 12,
                format: VertexFormat::Float32x3,
            }
        ];
        let mut attributes = Vec::with_capacity(2);
        let mut attribute_ids = Vec::with_capacity(2);
        let mut accumulated_offset = 0;
        for (index, data) in self.attributes.values().enumerate() {
            attribute_ids.push(data.attribute.id);
            attributes.push(VertexAttribute {
                offset: accumulated_offset,
                format: data.attribute.format,
                shader_location: index as u32,
            });
            accumulated_offset += data.attribute.format.get_size();
        }

        MeshVertexBufferLayout::new(InnerMeshVertexBufferLayout {
            layout: VertexBufferLayout {
                array_stride: accumulated_offset,
                step_mode: VertexStepMode::Vertex,
                attributes,
            },
            attribute_ids,
        })

        Ok(GpuMesh {
            vertex_buffer,
            buffer_info,
            vertex_count: 4,
            primitive_topology: PrimitiveTopology::TriangleList,
            layout:
        })
    }
}
