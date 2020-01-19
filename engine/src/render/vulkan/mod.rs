use std::sync::Arc;
use log;

use vulkano::command_buffer as vc;
use vulkano::instance as vi;
use vulkano::swapchain as vs;
use vulkano::framebuffer as vf;
use vulkano::pipeline::vertex as vpv;
use vulkano::sync::{FenceSignalFuture, GpuFuture};

mod binding;
mod shaders;
mod swapchains;
mod qfi;

const VERSION: vi::Version = vi::Version { major: 1, minor: 0, patch: 0};

fn required_instance_extensions() -> vi::InstanceExtensions {
    let mut exts = vulkano_win::required_extensions();
    exts.ext_debug_report = true;
    exts
}

pub struct Instance<WT> {
    debug_callback: vi::debug::DebugCallback,
    vulkan: Arc<vi::Instance>,

    binding: Option<binding::Binding<WT>>,
    swapchains: Option<swapchains::Swapchains<WT>>,
    render_pass: Option<Arc<dyn vf::RenderPassAbstract + Send + Sync>>,
    framebuffers: Vec<Arc<dyn vf::FramebufferAbstract + Send + Sync>>,
    command_buffers: Vec<Arc<vc::AutoCommandBuffer>>,
}

impl<WT: 'static + Send + Sync> Instance<WT> {
    pub fn new(name: String) -> Self {
        let ai = vi::ApplicationInfo {
            application_name: Some(name.clone().into()),
            application_version: Some(VERSION),
            engine_name: Some(name.clone().into()),
            engine_version: Some(VERSION),
        };

        let exts = required_instance_extensions();
        let layers = ["VK_LAYER_LUNARG_standard_validation"];
        let vulkan = vi::Instance::new(Some(&ai), &exts, layers.iter().cloned()).expect("could not create vulkan instance");
        let debug_callback = Self::init_debug_callback(&vulkan);


        Self {
            debug_callback,
            vulkan,

            binding: None,
            swapchains: None,
            render_pass: None,
            framebuffers: vec![],
            command_buffers: vec![],
        }
    }

    pub fn get_vulkan(&self) -> Arc<vi::Instance> {
        self.vulkan.clone()
    }

    pub fn get_swapchain(&self) -> Arc<vs::Swapchain<WT>> {
        self.swapchains.as_ref().unwrap().chain.clone()
    }

    pub fn use_surface(&mut self, surface: &Arc<vs::Surface<WT>>) {
        self.binding = Some(binding::Binding::new(&self.vulkan, &surface));
        self.swapchains = Some(swapchains::Swapchains::new(self.binding.as_ref().unwrap()));

        log::info!("Bound to Vulkan Device: {}", self.binding.as_ref().unwrap().physical_device().name());

        let device = self.binding.as_ref().unwrap().device.clone();
        let chain = self.get_swapchain();

        self.create_render_pass(chain.format());
        let render_pass = self.render_pass.as_ref().unwrap().clone();

        let pipeline = shaders::pipeline_triangle(device, chain.dimensions(), render_pass);
        self.create_framebuffers();

        self.create_command_buffers(pipeline);
    }

    pub fn flip(&self) -> FenceSignalFuture<vs::PresentFuture<vc::CommandBufferExecFuture<vs::SwapchainAcquireFuture<WT>, Arc<vc::AutoCommandBuffer>>, WT>> {
        let chain = self.get_swapchain();
        let (image_index, acquire_future) = vs::acquire_next_image(chain.clone(), None).unwrap();
        let command_buffer = self.command_buffers[image_index].clone();

        let gq = self.binding.as_ref().unwrap().graphics_queue.clone();
        let pq = self.binding.as_ref().unwrap().present_queue.clone();

        acquire_future
            .then_execute(gq, command_buffer)
            .unwrap()
            .then_swapchain_present(pq, chain, image_index)
            .then_signal_fence_and_flush()
            .unwrap()
    }

    fn create_render_pass(&mut self, color_format: vulkano::format::Format) {
        let device = self.binding.as_ref().unwrap().device.clone();

        self.render_pass = Some(Arc::new(vulkano::single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: color_format,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap()))
    }

    fn create_framebuffers(&mut self) {
        let render_pass = self.render_pass.as_ref().unwrap().clone();

        self.framebuffers = self.swapchains.as_ref().unwrap().images.iter()
            .map(|image| {
                let fba: Arc<dyn vf::FramebufferAbstract + Send + Sync> = Arc::new(vf::Framebuffer::start(render_pass.clone())
                    .add(image.clone()).unwrap()
                    .build().unwrap());
                fba
            })
        .collect::<Vec<_>>();
    }

    fn create_command_buffers(&mut self, pipeline: Arc<shaders::ConcreteGraphicsPipeline>) {
        let device = self.binding.as_ref().unwrap().device.clone();
        let qf = self.binding.as_ref().unwrap().graphics_queue.family();
        self.command_buffers = self.framebuffers.iter()
            .map(|framebuffer| {
                let vertices = vpv::BufferlessVertices { vertices: 3, instances: 1 };
                Arc::new(vc::AutoCommandBufferBuilder::primary_simultaneous_use(device.clone(), qf)
                         .unwrap()
                         .begin_render_pass(framebuffer.clone(), false, vec![[0.0, 0.0, 0.0, 1.0].into()])
                         .unwrap()
                         .draw(pipeline.clone(), &vc::DynamicState::none(),
                            vertices, (), ())
                         .unwrap()
                         .end_render_pass()
                         .unwrap()
                         .build()
                         .unwrap())
            })
            .collect();
    }


    fn init_debug_callback(instance: &Arc<vi::Instance>) -> vi::debug::DebugCallback {
        let mt = vi::debug::MessageTypes {
            error: true,
            warning: true,
            performance_warning: true,
            information: true,
            debug: true,
        };
        vi::debug::DebugCallback::new(&instance, mt, |msg| {
            log::debug!("validation layer: {:?}", msg.description);
        }).expect("could not create debug callback")
    }
}
