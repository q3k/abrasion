use std::sync::Arc;
use std::time;
use log;

use vulkano::command_buffer as vc;
use vulkano::buffer as vb;
use vulkano::instance as vi;
use vulkano::swapchain as vs;
use vulkano::framebuffer as vf;
use vulkano::pipeline as vp;
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

    surface: Option<Arc<vs::Surface<WT>>>,
    binding: Option<binding::Binding<WT>>,
    swapchains: Option<swapchains::Swapchains<WT>>,
    render_pass: Option<Arc<dyn vf::RenderPassAbstract + Send + Sync>>,
    framebuffers: Vec<Arc<dyn vf::FramebufferAbstract + Send + Sync>>,
    command_buffers: Vec<Arc<vc::AutoCommandBuffer>>,

    armed: bool,
    previous_frame_end: Option<Box<FlipFuture<WT>>>,
    fps_counter: crate::util::counter::Counter,
}

type FlipFuture<WT> = FenceSignalFuture<vs::PresentFuture<vc::CommandBufferExecFuture<vs::SwapchainAcquireFuture<WT>, Arc<vc::AutoCommandBuffer>>, WT>>;


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

            surface: None,
            binding: None,
            swapchains: None,
            render_pass: None,
            framebuffers: vec![],
            command_buffers: vec![],

            previous_frame_end: None,
            armed: false,
            fps_counter: crate::util::counter::Counter::new(time::Duration::from_millis(100)),
        }
    }

    pub fn get_vulkan(&self) -> Arc<vi::Instance> {
        self.vulkan.clone()
    }

    pub fn get_swapchain(&self) -> Arc<vs::Swapchain<WT>> {
        self.swapchains.as_ref().unwrap().chain.clone()
    }

    pub fn use_surface(&mut self, surface: &Arc<vs::Surface<WT>>) {
        self.surface = Some(surface.clone());

        self.binding = Some(binding::Binding::new(&self.vulkan, self.surface.as_ref().unwrap().clone()));
        log::info!("Bound to Vulkan Device: {}", self.binding.as_ref().unwrap().physical_device().name());

        self.arm();
    }

    fn arm(&mut self) {
        self.swapchains = Some(swapchains::Swapchains::new(self.binding.as_ref().unwrap(), self.swapchains.as_ref()));

        let device = self.binding.as_ref().unwrap().device.clone();
        let chain = self.get_swapchain();

        self.create_render_pass(chain.format());
        self.create_framebuffers();

        let render_pass = self.render_pass.as_ref().unwrap().clone();
        let pipeline = shaders::pipeline_forward(device.clone(), chain.dimensions(), render_pass);
        let buffer = vb::cpu_access::CpuAccessibleBuffer::from_iter(device.clone(),
            vb::BufferUsage::vertex_buffer(), vertices().iter().cloned()).unwrap();
        self.create_command_buffers(pipeline, buffer);

        self.previous_frame_end = None;
        self.armed = true;
    }


    // (╯°□°)╯︵ ┻━┻
    pub fn flip(&mut self) {
        match &self.previous_frame_end {
            None => (),
            Some(future) => future.wait(None).unwrap(),
        }

        if !self.armed {
            self.arm();
        }

        let chain = self.get_swapchain();
        let (image_index, acquire_future) = match vs::acquire_next_image(chain.clone(), None) {
            Ok(r) => r,
            Err(vs::AcquireError::OutOfDate) => {
                self.armed = false;
                self.previous_frame_end = None;
                return;
            },
            Err(err) => panic!("{:?}", err),
        };
        let command_buffer = self.command_buffers[image_index].clone();

        let gq = self.binding.as_ref().unwrap().graphics_queue.clone();
        let pq = self.binding.as_ref().unwrap().present_queue.clone();

        let future = acquire_future
            .then_execute(gq, command_buffer)
            .unwrap()
            .then_swapchain_present(pq, chain, image_index)
            .then_signal_fence_and_flush();

        match future {
            Ok(_) => (),
            Err(vulkano::sync::FlushError::OutOfDate) => {
                self.armed = false;
                self.previous_frame_end = None;
                return;
            },
            Err(err) => panic!("{:?}", err),
        };

        self.previous_frame_end = Some(Box::new(future.unwrap()));
        match self.fps_counter.tick() {
            Some(rate) => log::info!("FPS: {}", rate),
            None => ()
        }
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

    fn create_command_buffers(
        &mut self,
        pipeline: Arc<dyn vp::GraphicsPipelineAbstract + Send + Sync>,
        vertex_buffer: Arc<dyn vb::BufferAccess + Send + Sync>,
    ) {
        let device = self.binding.as_ref().unwrap().device.clone();
        let qf = self.binding.as_ref().unwrap().graphics_queue.family();
        self.command_buffers = self.framebuffers.iter()
            .map(|framebuffer| {
                Arc::new(vc::AutoCommandBufferBuilder::primary_simultaneous_use(device.clone(), qf)
                         .unwrap()
                         .begin_render_pass(framebuffer.clone(), false, vec![[0.0, 0.0, 0.0, 1.0].into()])
                         .unwrap()
                         .draw(pipeline.clone(), &vc::DynamicState::none(),
                            vec![vertex_buffer.clone()], (), ())
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

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn new(pos: [f32; 3], color: [f32; 3]) -> Self {
        Self { pos, color }
    }
}
vulkano::impl_vertex!(Vertex, pos, color);

fn vertices() -> [Vertex; 3] {
    [
        Vertex::new([0.0, -0.5, 0.0], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.0], [0.0, 1.0, 0.0]),
        Vertex::new([-0.5, 0.5, 0.0], [0.0, 0.0, 1.])
    ]
}
