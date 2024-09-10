use std::ops::Range;

use bevy::{
    prelude::*,
    render::{
        render_phase::{DrawFunctionId, PhaseItem},
        render_resource::CachedComputePipelineId,
    },
    utils::nonmax::NonMaxU32,
};

pub struct Isosurface {
    pub pipeline: CachedComputePipelineId,
    pub entity: Entity,
    pub batch_range: Range<u32>,
    pub draw_function: DrawFunctionId,
    pub dynamic_offset: Option<NonMaxU32>,
}

impl PhaseItem for Isosurface {
    type SortKey = u32;
    const AUTOMATIC_BATCHING: bool = false;

    fn entity(&self) -> Entity {
        self.entity
    }

    fn sort_key(&self) -> Self::SortKey {
        self.entity.index()
    }

    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    fn dynamic_offset(&self) -> Option<bevy::utils::nonmax::NonMaxU32> {
        self.dynamic_offset
    }

    fn dynamic_offset_mut(&mut self) -> &mut Option<NonMaxU32> {
        &mut self.dynamic_offset
    }
}
