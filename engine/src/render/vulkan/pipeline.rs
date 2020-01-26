use std::borrow::Cow;
use std::sync::Arc;

use vulkano::buffer as vb;
use vulkano::descriptor::descriptor_set as vdd;
use vulkano::device as vd;
use vulkano::format::Format;
use vulkano::framebuffer as vf;
use vulkano::pipeline as vp;
use vulkano::pipeline::shader as vps;

use crate::render::vulkan::data;
use crate::render::vulkan::shaders;

type VulkanoPipeline = dyn vp::GraphicsPipelineAbstract + Send + Sync;
type VulkanoDescriptorSet = dyn vdd::DescriptorSet + Send + Sync;

pub trait Pipeline {
    fn get_pipeline(&self) -> Arc<VulkanoPipeline>;
    fn make_descriptor_set(&mut self, ubo: data::UniformBufferObject) -> Arc<VulkanoDescriptorSet>;
}

pub struct Forward {
    device: Arc<vd::Device>,

    pipeline: Arc<VulkanoPipeline>,
    descriptor_set_pool: vdd::FixedSizeDescriptorSetsPool<Arc<VulkanoPipeline>>,
}

impl Forward {
    pub fn new(
        device: Arc<vd::Device>,
        viewport_dimensions: [u32; 2],
        render_pass: Arc<dyn vf::RenderPassAbstract + Send + Sync>,
    ) -> Forward {
        let vertex_shader = shaders::ShaderDefinition {
            name: "forward_vert.spv".to_string(),
            ty: vps::GraphicsShaderType::Vertex,
            inputs: vec![
                vps::ShaderInterfaceDefEntry { location: 0..1, format: Format::R32G32B32Sfloat, name: Some(Cow::Borrowed("pos")) },
                vps::ShaderInterfaceDefEntry { location: 1..2, format: Format::R32G32B32Sfloat, name: Some(Cow::Borrowed("color")) },
            ],
            outputs: vec![
                vps::ShaderInterfaceDefEntry { location: 0..1, format: Format::R32G32B32Sfloat, name: Some(Cow::Borrowed("fragColor")) }
            ],
        }.load_into(device.clone()).expect("could not load vertex shader");

        let fragment_shader = shaders::ShaderDefinition {
            name: "forward_frag.spv".to_string(),
            ty: vps::GraphicsShaderType::Fragment,
            inputs: vec![
                vps::ShaderInterfaceDefEntry { location: 0..1, format: Format::R32G32B32Sfloat, name: Some(Cow::Borrowed("fragColor")) }
            ],
            outputs: vec![
                vps::ShaderInterfaceDefEntry { location: 0..1, format: Format::R32G32B32A32Sfloat, name: Some(Cow::Borrowed("outColor")) }
            ],
        }.load_into(device.clone()).expect("could not load fragment shader");

        let dimensions = [viewport_dimensions[0] as f32, viewport_dimensions[1] as f32];
        let viewport = vp::viewport::Viewport {
            origin: [0.0, 0.0],
            dimensions,
            depth_range: 0.0 .. 1.0,
        };

        // Counter-clockwise facing triangles - this is because geometry data is left-handed, and
        // the vertex shader performs a handedness flip by doing .y *= -1 on emitted vertices. To
        // keep geomtry-space triangles clockwise after this transformation, the pipeline must be
        // set to treat counter-clockwise triangles as front-facing.  An alternative would be to
        // fully embrace the vulkan coordinate system, including geometry - however this goes
        // against most existing software and practices.  This might bite us in the ass at some
        // point in the future.
        let pipeline = Arc::new(vp::GraphicsPipeline::start()
                 .vertex_input_single_buffer::<data::Vertex>()
                 .vertex_shader(vertex_shader.entry_point(), ())
                 .triangle_list()
                 .primitive_restart(false)
                 .viewports(vec![viewport])
                 .fragment_shader(fragment_shader.entry_point(), ())
                 .depth_clamp(false)
                 .polygon_mode_fill()
                 .line_width(1.0)
                 .cull_mode_back()
                 .front_face_counter_clockwise()
                 .blend_pass_through()
                 .render_pass(vf::Subpass::from(render_pass.clone(), 0).unwrap())
                 .build(device.clone())
                 .unwrap())
            as Arc<VulkanoPipeline>;

        let descriptor_set_pool = vdd::FixedSizeDescriptorSetsPool::new(pipeline.clone(), 0);

        Forward {
            device,

            pipeline,
            descriptor_set_pool
        }
    }
}

impl Pipeline for Forward {
    fn get_pipeline(&self) -> Arc<VulkanoPipeline> {
        self.pipeline.clone()
    }

    fn make_descriptor_set(&mut self, ubo: data::UniformBufferObject) -> Arc<VulkanoDescriptorSet> {
        let buffer = vb::CpuAccessibleBuffer::from_data(
            self.device.clone(),
            vb::BufferUsage::uniform_buffer_transfer_destination(),
            ubo,
        ).unwrap();

        Arc::new(self.descriptor_set_pool.next()
            .add_buffer(buffer).unwrap()
            .build().unwrap())
    }
}
