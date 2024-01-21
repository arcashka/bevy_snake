use bevy::{
    prelude::*,
    render::{
        render_phase::{CachedRenderPipelinePhaseItem, DrawFunctionId, PhaseItem},
        render_resource::CachedRenderPipelineId,
    },
    utils::nonmax::NonMaxU32,
};

use std::ops::Range;

pub struct SnakePhaseItem {
    pub entity: Entity,
    pub draw_function: DrawFunctionId,
    pub pipeline: CachedRenderPipelineId,
    pub batch_range: Range<u32>,
    pub dynamic_offset: Option<NonMaxU32>,
}

impl PhaseItem for SnakePhaseItem {
    type SortKey = usize;

    #[inline]
    fn entity(&self) -> Entity {
        self.entity
    }

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        self.pipeline.id()
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    #[inline]
    fn dynamic_offset(&self) -> Option<NonMaxU32> {
        self.dynamic_offset
    }

    #[inline]
    fn dynamic_offset_mut(&mut self) -> &mut Option<NonMaxU32> {
        &mut self.dynamic_offset
    }
}

impl CachedRenderPipelinePhaseItem for SnakePhaseItem {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}
