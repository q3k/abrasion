use std::sync::Arc;
use log;

use vulkano::instance as vi;
use vulkano::swapchain as vs;
use std::ops::Deref;

mod binding;
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
}

impl<WT> Instance<WT> {
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
        }
    }

    pub fn get_vulkan(&self) -> Arc<vi::Instance> {
        self.vulkan.clone()
    }

    pub fn use_surface(&mut self, surface: &Arc<vs::Surface<WT>>) {
        self.binding = Some(binding::Binding::new(&self.vulkan, &surface));
        self.swapchains = Some(swapchains::Swapchains::new(self.binding.as_ref().unwrap()));

        log::info!("Bound to Vulkan Device: {}", self.binding.as_ref().unwrap().physical_device().name());
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


