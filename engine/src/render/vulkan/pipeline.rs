use std::sync::Arc;

use vulkano::buffer as vb;
use vulkano::pipeline as vp;
use vulkano::descriptor::descriptor_set as vdd;
use vulkano::memory as vm;

use crate::render::vulkan::data;

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

