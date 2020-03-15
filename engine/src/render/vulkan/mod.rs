use std::collections::HashMap;
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

pub mod data;
mod surface_binding;
mod pipeline;
mod qfi;
mod shaders;
mod swapchain_binding;
mod worker;

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

    workers: Vec<worker::Worker>,

    surface_binding: Option<surface_binding::SurfaceBinding<WT>>,
    swapchain_binding: Option<swapchain_binding::SwapchainBinding<WT>>,

    pipeline: Option<Box<dyn pipeline::Pipeline>>,
    uniform_pool: Option<vb::CpuBufferPool<data::UniformBufferObject>>,
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

        let layer_preferences = vec![
            vec!["VK_LAYER_KHRONOS_validation"],
            vec!["VK_LAYER_LUNARG_standard_validation"],
            vec![],
        ];


        let mut vulkan_opt: Option<Arc<vi::Instance>> = None;
        for pref in layer_preferences {
            match vi::Instance::new(Some(&ai), &exts, pref.iter().cloned()) {
                Ok(res) => {
                    log::info!("Created vulkan instance with layers {}", pref.join(", "));
                    if pref.len() == 0 {
                        log::warn!("Did not load validation layers.");
                    }
                    vulkan_opt = Some(res);
                }
                Err(err) => {
                    log::warn!("Could not create vulkan instance with layers {}: {}", pref.join(", "), err);
                }
            }
        };

        let vulkan = vulkan_opt.expect("could not create a vulkan instance");
        let debug_callback = Self::init_debug_callback(&vulkan);

        let workers = (0..4).map(|n| {
            worker::Worker::new(n)
        }).collect();

        Self {
            debug_callback,
            vulkan,

            workers,

            surface_binding: None,
            swapchain_binding: None,

            pipeline: None,
            uniform_pool: None,
            previous_frame_end: None,
            armed: false,
            fps_counter: crate::util::counter::Counter::new(time::Duration::from_millis(1000)),
        }
    }

    pub fn get_vulkan(&self) -> Arc<vi::Instance> {
        self.vulkan.clone()
    }

    fn swapchain_binding(&self) -> &swapchain_binding::SwapchainBinding<WT> {
        self.swapchain_binding.as_ref().unwrap()
    }

    fn surface_binding(&self) -> &surface_binding::SurfaceBinding<WT> {
        self.surface_binding.as_ref().unwrap()
    }

    pub fn use_surface(&mut self, surface: &Arc<vs::Surface<WT>>) {
        self.surface_binding = Some(surface_binding::SurfaceBinding::new(&self.vulkan, surface.clone()));
        log::info!("Bound to Vulkan Device: {}", self.surface_binding().physical_device().name());

        self.arm();
    }

    fn arm(&mut self) {
        self.swapchain_binding = Some(swapchain_binding::SwapchainBinding::new(self.surface_binding(), self.swapchain_binding.as_ref()));

        let device = self.surface_binding().device.clone();
        let chain = self.swapchain_binding().chain.clone();

        let render_pass = self.swapchain_binding().render_pass.clone();

        self.pipeline = Some(Box::new(pipeline::Forward::new(device.clone(), chain.dimensions(), render_pass)));
        self.uniform_pool = Some(
            vb::CpuBufferPool::new(device.clone(), vb::BufferUsage::uniform_buffer_transfer_destination())
        );
        self.previous_frame_end = None;
        self.armed = true;
    }

    fn make_graphics_commands(
        &mut self,
        renderables: &Vec<Arc<dyn renderable::Renderable>>,
    ) -> Vec<Box<vc::AutoCommandBuffer>> {

        let dimensions = self.dimensions();

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

        let mut buffers: Vec<Box<vc::AutoCommandBuffer>> = vec![];

        // Sort renderables by mesh.
        let mut meshes: HashMap<Arc<renderable::Mesh>, Vec<cgm::Matrix4<f32>>> = HashMap::new();
        for r in renderables {
            if let Some((mesh, transform)) = r.render_data() {
                let entry = meshes.entry(mesh.clone()).or_insert(vec![]);
                entry.push(transform);
            }
        }

        let device = self.surface_binding().device.clone();
        let queue = self.surface_binding().graphics_queue.clone();
        let rp = self.swapchain_binding().render_pass.clone();
        let pipeline = self.pipeline.as_ref().unwrap().get_pipeline().clone();

        for (mesh, transforms) in meshes {
            let mut builder = vc::AutoCommandBufferBuilder::secondary_graphics_one_time_submit(
                device.clone(), queue.family(), vf::Subpass::from(rp.clone(), 0).unwrap()).unwrap();

            let ubo = data::UniformBufferObject {
                view: proj * view,
            };

            let (instancebuffer, future) = vb::immutable::ImmutableBuffer::from_iter(
                transforms.iter().map(|t| { data::Instance::new(t) }),
                vb::BufferUsage::vertex_buffer(),
                queue.clone(),
            ).unwrap();
            future.flush().unwrap();

            let (vbuffer, ibuffer) = mesh.vulkan_buffers(queue.clone());
            builder = builder.draw_indexed(pipeline.clone(), &vc::DynamicState::none(),
                vec![vbuffer.clone(), instancebuffer], ibuffer.clone(), (), ubo).unwrap();

            buffers.push(Box::new(builder.build().unwrap()));
        }

        return buffers;
    }

    fn make_command_buffer(
        &mut self,
        framebuffer: Arc<dyn vf::FramebufferAbstract + Send + Sync>,
        batches: Vec<Box<vc::AutoCommandBuffer>>,
    ) -> Arc<vc::AutoCommandBuffer> {

        let device = self.surface_binding().device.clone();
        let qf = self.surface_binding().graphics_queue.family();

        let mut primary = vc::AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), qf)
                 .unwrap()
                 .begin_render_pass(framebuffer.clone(), false, vec![[0.0, 0.0, 0.0, 1.0].into()])
                 .unwrap();


        for batch in batches {
            unsafe {
                primary = primary.execute_commands(batch).unwrap();
            }
        }

        Arc::new(primary.end_render_pass().unwrap().build().unwrap())
    }

    // (╯°□°)╯︵ ┻━┻
    pub fn flip(
        &mut self,
        renderables: Vec<Arc<dyn renderable::Renderable>>,
    ) {
        // Build batch command buffer as early as possible.
        let mut batches = self.make_graphics_commands(&renderables);

        match &self.previous_frame_end {
            None => (),
            Some(future) => future.wait(None).unwrap(),
        }

        if !self.armed {
            self.arm();
            // Rearming means the batch is invalid - rebuild it.
            batches = self.make_graphics_commands(&renderables);
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
        let command_buffer = self.make_command_buffer(fb, batches);

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
        if let Some(rate) = self.fps_counter.tick() {
            log::info!("FPS: {}", rate)
        }
    }

    fn dimensions(&self) -> [f32; 2] {
        let dimensions_u32 = self.swapchain_binding().chain.dimensions();
        [dimensions_u32[0] as f32, dimensions_u32[1] as f32]
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
