use std::sync::Arc;
use std::time;
use log;

use cgmath as cgm;
use vulkano::buffer as vb;
use vulkano::command_buffer as vc;
use vulkano::framebuffer as vf;
use vulkano::instance as vi;
use vulkano::swapchain as vs;
use vulkano::sync::{FenceSignalFuture, GpuFuture};

mod binding;
pub mod data;
mod pipeline;
mod qfi;
mod shaders;
mod swapchains;

use crate::render::renderable;

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

    pipeline: Option<Box<dyn pipeline::Pipeline>>,
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

            pipeline: None,
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

        self.pipeline = Some(Box::new(pipeline::Forward::new(device.clone(), chain.dimensions(), render_pass)));
        self.previous_frame_end = None;
        self.armed = true;
    }


    // (╯°□°)╯︵ ┻━┻
    pub fn flip(
        &mut self,
        render_data: Vec<renderable::Data>,
    ) {
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

        let fb = self.swapchain_binding().framebuffers[image_index].clone();
        let command_buffer = self.make_command_buffer(fb, render_data);

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

    fn dimensions(&self) -> [f32; 2] {
        let dimensions_u32 = self.swapchain_binding().chain.dimensions();
        [dimensions_u32[0] as f32, dimensions_u32[1] as f32]
    }

    fn make_command_buffer(
        &mut self,
        framebuffer: Arc<dyn vf::FramebufferAbstract + Send + Sync>,
        render_data: Vec<renderable::Data>,
    ) -> Arc<vc::AutoCommandBuffer> {
        let device = self.surface_binding().device.clone();
        let qf = self.surface_binding().graphics_queue.family();
        let dimensions = self.dimensions();
        let mut c = vc::AutoCommandBufferBuilder::primary_simultaneous_use(device.clone(), qf)
                 .unwrap()
                 .begin_render_pass(framebuffer.clone(), false, vec![[0.0, 0.0, 0.0, 1.0].into()])
                 .unwrap();

        let view = cgm::Matrix4::look_at(
            cgm::Point3::new(2.0, 2.0, 2.0),
            cgm::Point3::new(0.0, 0.0, 0.0),
            cgm::Vector3::new(0.0, 0.0, 1.0)
        );
        let proj = cgm::perspective(
            cgm::Rad::from(cgm::Deg(45.0)),
            dimensions[0] / dimensions[1],
            0.1,
            10.0
        );

        for d in render_data {
            let (vbuffer, future) = vb::immutable::ImmutableBuffer::from_iter(
                d.vertices.iter().cloned(),
                vb::BufferUsage::vertex_buffer(),
                self.surface_binding().graphics_queue.clone(),
            ).unwrap();
            future.flush().unwrap();

            let (ibuffer, future) = vb::immutable::ImmutableBuffer::from_iter(
                d.indices.iter().cloned(),
                vb::BufferUsage::index_buffer(),
                self.surface_binding().graphics_queue.clone(),
            ).unwrap();
            future.flush().unwrap();


            let ubo = data::UniformBufferObject {
                model: d.transform.clone(),
                view: view.clone(),
                proj: proj.clone(),
            };
            let ds = self.pipeline.as_mut().unwrap().make_descriptor_set(ubo);
            let pipeline = self.pipeline.as_ref().unwrap().get_pipeline();
            c = c.draw_indexed(pipeline, &vc::DynamicState::none(),
                vec![vbuffer.clone()],
                ibuffer.clone(),
                ds,
                ()).unwrap();
        }


        Arc::new(c.end_render_pass().unwrap()
            .build().unwrap())
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
