use std::sync::Arc;

use cgmath as cgm;

use winit::{
    dpi::LogicalSize,
    window::Window,
    window::WindowBuilder,
    event_loop::EventLoop,
    event::Event,
    event::WindowEvent,
    platform::desktop::EventLoopExtDesktop,
};
use vulkano_win::VkSurfaceBuild;
use vulkano::instance as vi;
use vulkano::swapchain as vs;

pub mod vulkan;
pub mod renderable;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Renderer {
    instance: vulkan::Instance<Window>,
    events_loop: EventLoop<()>,
}

impl Renderer {
    pub fn initialize() -> Self {
        let mut instance = vulkan::Instance::new("abrasion".to_string());
        let (events_loop, surface) = Self::init_window(instance.get_vulkan());
        instance.use_surface(&surface);

        Self {
            instance,
            events_loop,
        }
    }

    fn init_window(instance: Arc<vi::Instance>) -> (EventLoop<()>, Arc<vs::Surface<Window>>) {
        let events_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .with_title("abrasion")
            .with_inner_size(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build_vk_surface(&events_loop, instance.clone())
            .expect("could not create surface");
        (events_loop, surface)
    }

    pub fn draw_frame(&mut self, view: &cgm::Matrix4<f32>, renderables: &Vec<Arc<dyn renderable::Renderable>>) {
        self.instance.flip(view, renderables);
    }

    pub fn poll_close(&mut self) -> bool {
        let mut close = false;
        // TODO(q3k): migrate to EventLoop::run
        self.events_loop.run_return(|ev, _, control_flow| {
            if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = ev {
                close = true;
            }
            *control_flow = winit::event_loop::ControlFlow::Exit;
        });
        return close;
    }
}
