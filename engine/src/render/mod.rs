use std::sync::Arc;

use winit::{
    dpi::LogicalSize,
    Window,
    WindowEvent,
    WindowBuilder,
    EventsLoop,
    Event,
};
use vulkano_win::VkSurfaceBuild;
use vulkano::instance as vi;
use vulkano::swapchain as vs;

mod vulkan;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Renderer {
    instance: vulkan::Instance<winit::Window>,
    events_loop: EventsLoop,
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

    fn init_window(instance: Arc<vi::Instance>) -> (EventsLoop, Arc<vs::Surface<Window>>) {
        let events_loop = EventsLoop::new();
        let surface = WindowBuilder::new()
            .with_title("abrasion")
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build_vk_surface(&events_loop, instance.clone())
            .expect("could not create surface");
        (events_loop, surface)
    }

    pub fn main_loop(&mut self) {
        loop {
            let mut done = false;
            self.events_loop.poll_events(|ev| {
                if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = ev {
                    done = true
                }
            });
            if done {
                return;
            }
        }
    }
}