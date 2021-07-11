// Copyright 2020 Sergiusz 'q3k' Bazanski <q3k@q3k.org>
//
// This file is part of Abrasion.
//
// Abrasion is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, version 3.
//
// Abrasion is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// Abrasion.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::Arc;

use vulkano::buffer as vb;
use vulkano::pipeline as vp;
use vulkano::descriptor::descriptor_set as vdd;
use vulkano::memory as vm;

use crate::vulkan::data;

pub type VulkanoPipeline = dyn vp::GraphicsPipelineAbstract + Send + Sync;
pub type VulkanoDescriptorSet = dyn vdd::DescriptorSet + Send + Sync;

pub trait Pipeline {
    fn get_pipeline(&self) -> Arc<VulkanoPipeline>;
    fn make_descriptor_set(
        &mut self,
        textures: data::Textures,
        fragment_ubo_buffer: Arc<vb::cpu_pool::CpuBufferPoolSubbuffer<data::FragmentUniformBufferObject, Arc<vm::pool::StdMemoryPool>>>,
    ) -> Arc<VulkanoDescriptorSet>;
}

