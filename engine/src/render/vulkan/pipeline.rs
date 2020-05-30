use std::sync::Arc;

use vulkano::format::Format;
use vulkano::pipeline as vp;
use vulkano::descriptor::descriptor_set as vdd;
use vulkano::image as vm;

pub type VulkanoPipeline = dyn vp::GraphicsPipelineAbstract + Send + Sync;
pub type VulkanoDescriptorSet = dyn vdd::DescriptorSet + Send + Sync;

pub trait Pipeline {
    fn get_pipeline(&self) -> Arc<VulkanoPipeline>;
    fn make_descriptor_set(&mut self, texture_image: Arc<vm::ImmutableImage<Format>>) -> Arc<VulkanoDescriptorSet>;
}

