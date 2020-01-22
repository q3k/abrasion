use std::sync::Arc;
use std::time;
use log;

use vulkano::command_buffer as vc;
use vulkano::buffer as vb;
use vulkano::instance as vi;
use vulkano::swapchain as vs;
use vulkano::pipeline as vp;
use vulkano::sync::{FenceSignalFuture, GpuFuture};

mod binding;
mod data;
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

    surface_binding: Option<binding::SurfaceBinding<WT>>,
    swapchain_binding: Option<swapchains::SwapchainBinding<WT>>,
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

            surface_binding: None,
            swapchain_binding: None,
            command_buffers: vec![],

            previous_frame_end: None,
            armed: false,
            fps_counter: crate::util::counter::Counter::new(time::Duration::from_millis(1000)),
        }
    }

    pub fn get_vulkan(&self) -> Arc<vi::Instance> {
        self.vulkan.clone()
    }

    fn swapchain_binding(&self) -> &swapchains::SwapchainBinding<WT> {
        self.swapchain_binding.as_ref().unwrap()
    }

    fn surface_binding(&self) -> &binding::SurfaceBinding<WT> {
        self.surface_binding.as_ref().unwrap()
    }

    pub fn use_surface(&mut self, surface: &Arc<vs::Surface<WT>>) {
        self.surface_binding = Some(binding::SurfaceBinding::new(&self.vulkan, surface.clone()));
        log::info!("Bound to Vulkan Device: {}", self.surface_binding().physical_device().name());

        self.arm();
    }

    fn arm(&mut self) {
        self.swapchain_binding = Some(swapchains::SwapchainBinding::new(self.surface_binding(), self.swapchain_binding.as_ref()));

        let device = self.surface_binding().device.clone();
        let chain = self.swapchain_binding().chain.clone();

        let render_pass = self.swapchain_binding().render_pass.clone();
        let pipeline = shaders::pipeline_forward(device.clone(), chain.dimensions(), render_pass);

        let (vbuffer, future) = vb::immutable::ImmutableBuffer::from_iter(
            data::vertices().iter().cloned(),
            vb::BufferUsage::vertex_buffer(),
            self.surface_binding().graphics_queue.clone(),
        ).unwrap();
        future.flush().unwrap();

        let (ibuffer, future) = vb::immutable::ImmutableBuffer::from_iter(
            data::indices().iter().cloned(),
            vb::BufferUsage::index_buffer(),
            self.surface_binding().graphics_queue.clone(),
        ).unwrap();
        future.flush().unwrap();

        self.create_command_buffers(pipeline, vbuffer, ibuffer);

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

        let chain = self.swapchain_binding().chain.clone();
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

        let gq = self.surface_binding().graphics_queue.clone();
        let pq = self.surface_binding().present_queue.clone();

        let future = acquire_future
            .then_execute(gq, command_buffer)
            .unwrap()
            .then_swapchain_present(pq, chain.clone(), image_index)
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

    fn create_command_buffers(
        &mut self,
        pipeline: Arc<dyn vp::GraphicsPipelineAbstract + Send + Sync>,
        vertex_buffer: Arc<dyn vb::BufferAccess + Send + Sync>,
        index_buffer: Arc<vb::TypedBufferAccess<Content=[u16]> + Send + Sync>,
    ) {
        let device = self.surface_binding().device.clone();
        let qf = self.surface_binding().graphics_queue.family();
        self.command_buffers = self.swapchain_binding().framebuffers.iter()
            .map(|framebuffer| {
                Arc::new(vc::AutoCommandBufferBuilder::primary_simultaneous_use(device.clone(), qf)
                         .unwrap()
                         .begin_render_pass(framebuffer.clone(), false, vec![[0.0, 0.0, 0.0, 1.0].into()])
                         .unwrap()
                         .draw_indexed(pipeline.clone(), &vc::DynamicState::none(),
                            vec![vertex_buffer.clone()],
                            index_buffer.clone(),
                            (), ())
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
