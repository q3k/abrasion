use log;
use env_logger;

mod render;

fn main() {
    env_logger::init();
    log::info!("Starting...");
    let mut renderer = render::Renderer::initialize();
    renderer.main_loop();
}
