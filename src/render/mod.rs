use winit::{
    dpi::LogicalSize,
    WindowEvent,
    WindowBuilder,
    EventsLoop,
    Event,
};

mod vulkan;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Renderer {
    instance: vulkan::Instance,
    events_loop: EventsLoop,
}

impl Renderer {
    pub fn initialize() -> Self {
        let instance = vulkan::Instance::new("threepy".to_string());
        let events_loop = Self::init_window();
        Self {
            instance,
            events_loop,
        }
    }

    fn init_window() -> EventsLoop {
        let events_loop = EventsLoop::new();
        let _window = WindowBuilder::new()
            .with_title("threepy")
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build(&events_loop);
        events_loop
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
