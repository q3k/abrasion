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

use std::borrow::Cow;
use std::sync::Arc;

use vulkano::buffer as vb;
use vulkano::descriptor::descriptor as vdD;
use vulkano::descriptor::descriptor_set as vdd;
use vulkano::descriptor::pipeline_layout as vdp;
use vulkano::device as vd;
use vulkano::format::Format;
use vulkano::framebuffer as vf;
use vulkano::memory as vm;
use vulkano::pipeline as vp;
use vulkano::pipeline::shader as vps;
use vulkano::pipeline::vertex as vpv;
use vulkano::sampler as vs;

use crate::mesh::Vertex;
use crate::vulkan::data;
use crate::vulkan::shaders;
use crate::vulkan::pipeline;

pub struct Forward {
    pipeline: Arc<pipeline::VulkanoPipeline>,
    descriptor_set_pool: vdd::FixedSizeDescriptorSetsPool,
    device: Arc<vd::Device>,
}

impl Forward {
    pub fn new(
        device: Arc<vd::Device>,
        viewport_dimensions: [u32; 2],
        render_pass: Arc<dyn vf::RenderPassAbstract + Send + Sync>,
    ) -> Forward {
        let vertex_shader = shaders::ShaderDefinition {
            name: "//assets/shaders/forward_vert.spv".to_string(),
            ty: vps::GraphicsShaderType::Vertex,
            inputs: vec![
                vps::ShaderInterfaceDefEntry {
                    location: 0..1, format: Format::R32G32B32Sfloat,
                    name: Some(Cow::Borrowed("pos")),
                },
                vps::ShaderInterfaceDefEntry {
                    location: 1..2, format: Format::R32G32B32Sfloat,
                    name: Some(Cow::Borrowed("normal")),
                },
                vps::ShaderInterfaceDefEntry {
                    location: 2..6, format: Format::R32G32B32A32Sfloat,
                    name: Some(Cow::Borrowed("model")),
                },
                vps::ShaderInterfaceDefEntry {
                    location: 6..7, format: Format::R32G32Sfloat,
                    name: Some(Cow::Borrowed("tex")),
                },
            ],
            outputs: vec![
                vps::ShaderInterfaceDefEntry {
                    location: 0..1, format: Format::R32G32Sfloat,
                    name: Some(Cow::Borrowed("fragTexCoord")),
                },
                vps::ShaderInterfaceDefEntry {
                    location: 1..2, format: Format::R32G32B32Sfloat,
                    name: Some(Cow::Borrowed("fragWorldPos")),
                },
                vps::ShaderInterfaceDefEntry {
                    location: 2..3, format: Format::R32G32B32Sfloat,
                    name: Some(Cow::Borrowed("fragNormal")),
                },
            ],
            uniforms: vec![],
            push_constants: vec![
                vdp::PipelineLayoutDescPcRange {
                    offset: 0,
                    size: 64usize,
                    stages: vdD::ShaderStages {
                        vertex: true,
                        ..vdD::ShaderStages::none()
                    },
                },
            ],
        }.load_into(device.clone()).expect("could not load vertex shader");

        let fragment_shader = shaders::ShaderDefinition {
            name: "//assets/shaders/forward_frag.spv".to_string(),
            ty: vps::GraphicsShaderType::Fragment,
            inputs: vec![
                vps::ShaderInterfaceDefEntry {
                    location: 0..1, format: Format::R32G32Sfloat,
                    name: Some(Cow::Borrowed("fragTexCoord")),
                },
                vps::ShaderInterfaceDefEntry {
                    location: 1..2, format: Format::R32G32B32Sfloat,
                    name: Some(Cow::Borrowed("fragWorldPos")),
                },
                vps::ShaderInterfaceDefEntry {
                    location: 2..3, format: Format::R32G32B32Sfloat,
                    name: Some(Cow::Borrowed("fragNormal")),
                },
            ],
            outputs: vec![
                vps::ShaderInterfaceDefEntry {
                    location: 0..1, format: Format::R32G32B32A32Sfloat,
                    name: Some(Cow::Borrowed("outColor")),
                }
            ],
            uniforms: vec![
                vdD::DescriptorDesc {
                    ty: vdD::DescriptorDescTy::Buffer(vdD::DescriptorBufferDesc {
                        dynamic: Some(false),
                        storage: false,
                    }),
                    array_count: 1,
                    readonly: true,
                    stages: vdD::ShaderStages {
                        fragment: true,
                        ..vdD::ShaderStages::none()
                    },
                },
                vdD::DescriptorDesc {
                    ty: vdD::DescriptorDescTy::CombinedImageSampler(vdD::DescriptorImageDesc {
                        sampled: true,
                        dimensions: vdD::DescriptorImageDescDimensions::TwoDimensional,
                        format: None,
                        multisampled: false,
                        array_layers: vdD::DescriptorImageDescArray::NonArrayed,
                    }),
                    array_count: 1,
                    readonly: true,
                    stages: vdD::ShaderStages {
                        fragment: true,
                        ..vdD::ShaderStages::none()
                    },
                },
                vdD::DescriptorDesc {
                    ty: vdD::DescriptorDescTy::CombinedImageSampler(vdD::DescriptorImageDesc {
                        sampled: true,
                        dimensions: vdD::DescriptorImageDescDimensions::TwoDimensional,
                        format: None,
                        multisampled: false,
                        array_layers: vdD::DescriptorImageDescArray::NonArrayed,
                    }),
                    array_count: 1,
                    readonly: true,
                    stages: vdD::ShaderStages {
                        fragment: true,
                        ..vdD::ShaderStages::none()
                    },
                },
            ],
            push_constants: vec![
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
                 .vertex_input(vpv::OneVertexOneInstanceDefinition::<Vertex, data::Instance>::new())
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
                 .depth_stencil(vulkano::pipeline::depth_stencil::DepthStencil::simple_depth_test())
                 .render_pass(vf::Subpass::from(render_pass.clone(), 0).unwrap())
                 .build(device.clone())
                 .unwrap())
            as Arc<pipeline::VulkanoPipeline>;

        let layout = pipeline.descriptor_set_layout(0).unwrap();
        let descriptor_set_pool = vdd::FixedSizeDescriptorSetsPool::new(layout.clone());

        Forward {
            pipeline,
            descriptor_set_pool,
            device
        }
    }
}

impl pipeline::Pipeline for Forward {
    fn get_pipeline(&self) -> Arc<pipeline::VulkanoPipeline> {
        self.pipeline.clone()
    }

    fn make_descriptor_set(
        &mut self,
        textures: &data::Textures,
        fragment_ubo_buffer: Arc<vb::cpu_pool::CpuBufferPoolSubbuffer<data::FragmentUniformBufferObject, Arc<vm::pool::StdMemoryPool>>>,
    ) -> Arc<pipeline::VulkanoDescriptorSet> {
        let image_sampler = vs::Sampler::simple_repeat_linear(self.device.clone());

        Arc::new(self.descriptor_set_pool.next()
            .add_buffer(fragment_ubo_buffer).unwrap()
            .add_sampled_image(textures.diffuse.clone(), image_sampler.clone()).unwrap()
            .add_sampled_image(textures.roughness.clone(), image_sampler.clone()).unwrap()
            .build().unwrap())
    }
}
