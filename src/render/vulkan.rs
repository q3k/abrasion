use std::sync::Arc;
use log;

use vulkano;
use vulkano::instance as vi;

struct QueueFamilyIndices {
    graphics_family: i32,
}

impl QueueFamilyIndices {
    fn new() -> Self {
        Self { graphics_family: -1 }
    }
    fn is_complete(&self) -> bool {
        self.graphics_family >= 0
    }
}
 

const VERSION: vi::Version = vi::Version { major: 1, minor: 0, patch: 0};

pub struct Instance {
    vulkan: Arc<vi::Instance>,
    physical_device_ix: usize,
    debug_callback: vi::debug::DebugCallback,
}

impl Instance {
    pub fn new(name: String) -> Self {
        let ai = vi::ApplicationInfo {
            application_name: Some(name.clone().into()),
            application_version: Some(VERSION),
            engine_name: Some(name.clone().into()),
            engine_version: Some(VERSION),
        };

        let exts = Self::required_extensions();
        let layers = ["VK_LAYER_LUNARG_standard_validation"];
        let vulkan = vi::Instance::new(Some(&ai), &exts, layers.iter().cloned()).expect("could not create vulkan instance");

        let physical_device_ix = Self::pick_physical_device(&vulkan);

        let debug_callback = Self::init_debug_callback(&vulkan);

        Self {
            vulkan,
            physical_device_ix,
            debug_callback,
        }
    }

    fn pick_physical_device(instance: &Arc<vi::Instance>) -> usize {
        vi::PhysicalDevice::enumerate(&instance)
            .position(|dev| Self::is_device_suitable(&dev))
            .expect("could not find suitable GPU")
    }

    fn required_extensions() -> vi::InstanceExtensions {
        let mut exts = vulkano_win::required_extensions();
        exts.ext_debug_report = true;
        exts
    }

    fn is_device_suitable(device: &vi::PhysicalDevice) -> bool {
        log::info!("Inspecting device {}", device.name());
        let mut indices = QueueFamilyIndices::new();
        for (i, queue_family) in device.queue_families().enumerate() {
            log::info!("Inspecting queue family {:?}", queue_family);
            if queue_family.supports_graphics() {
                log::info!("Supports graphics.");
                indices.graphics_family = i as i32
            }
    
            if indices.is_complete() {
                log::info!("Complete!");
                return true;
            }
        }
    
        false
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
            log::info!("validation layer: {:?}", msg.description);
        }).expect("could not create debug callback")
    }
}


