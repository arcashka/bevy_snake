use bevy::{
    ecs::system::{lifetimeless::Read, SystemParamItem},
    log::*,
    pbr::{SetMaterialBindGroup, SetMeshBindGroup, SetMeshViewBindGroup},
    prelude::StandardMaterial,
    render::render_phase::{
        PhaseItem, RenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
    },
};

use super::components::SnakeMeshBuffer;

pub type DrawSnake = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMaterialBindGroup<StandardMaterial, 1>,
    SetMeshBindGroup<2>,
    DrawSnakeMesh,
);

pub struct DrawSnakeMesh;

impl<P: PhaseItem> RenderCommand<P> for DrawSnakeMesh {
    type Param = ();
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<SnakeMeshBuffer>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        buffer: &'w SnakeMeshBuffer,
        _param: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        info!("render called");
        pass.set_vertex_buffer(0, buffer.buffer.slice(..));
        pass.draw(0..buffer.length as u32, 0..1);
        RenderCommandResult::Success
    }
}
