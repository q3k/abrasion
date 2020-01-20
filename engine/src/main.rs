use log;
use env_logger;

mod render;
mod util;

fn main() {
    env_logger::init();
    log::info!("Starting...");
    let mut renderer = render::Renderer::initialize();
    renderer.main_loop();
}
