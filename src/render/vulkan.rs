use std::collections::HashSet;
use std::sync::Arc;
use log;

use vulkano::instance as vi;
use vulkano::device as vd;
use vulkano::swapchain as vs;

struct QueueFamilyIndices {
    graphics_family: i32,
    present_family: i32,
}

impl QueueFamilyIndices {
    fn new() -> Self {
        Self {
            graphics_family: -1,
            present_family: -1,
        }
    }
    fn is_complete(&self) -> bool {
        self.graphics_family >= 0 && self.present_family >= 0
    }
}
 

const VERSION: vi::Version = vi::Version { major: 1, minor: 0, patch: 0};

pub struct Instance<WT> {
    debug_callback: vi::debug::DebugCallback,
    vulkan: Arc<vi::Instance>,

    physical_device_ix: Option<usize>,
    device: Option<Arc<vd::Device>>,
    surface: Option<Arc<vs::Surface<WT>>>,
    graphics_queue: Option<Arc<vd::Queue>>,
    present_queue: Option<Arc<vd::Queue>>,
}

impl<WT> Instance<WT> {
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
        let debug_callback = Self::init_debug_callback(&vulkan);


        Self {
            debug_callback,
            vulkan,

            physical_device_ix: None,
            device: None,
            surface: None,
            graphics_queue: None,
            present_queue: None,
        }
    }

    pub fn bind_surface(&mut self, surface: &Arc<vs::Surface<WT>>) {
        let physical_device_ix = Self::pick_physical_device(&self.vulkan, &surface);
        let physical_device = vi::PhysicalDevice::from_index(&self.vulkan, physical_device_ix).unwrap();
        let indices = Self::find_queue_families(&surface, &physical_device).unwrap();

        let families = [indices.graphics_family, indices.present_family];
        use std::iter::FromIterator;
        let unique_queue_families: HashSet<&i32> = HashSet::from_iter(families.iter());

        let queue_priority = 1.0;
        let qf = unique_queue_families.iter().map(|i| {
            (physical_device.queue_families().nth(**i as usize).unwrap(), queue_priority)
        });

        let (device, mut queues) = vd::Device::new(physical_device, &vd::Features::none(), &vd::DeviceExtensions::none(), qf).expect("could not create logical device and queues");

        let graphics_queue = queues.next().unwrap();
        let present_queue = queues.next().unwrap_or_else(|| graphics_queue.clone());

        self.physical_device_ix = Some(physical_device_ix);
        self.device = Some(device);
        self.surface = Some(surface.clone());
        self.graphics_queue = Some(graphics_queue);
        self.present_queue = Some(present_queue);
    }

    pub fn get_vulkan(&self) -> Arc<vi::Instance> {
        self.vulkan.clone()
    }

    fn pick_physical_device(instance: &Arc<vi::Instance>, surface: &Arc<vs::Surface<WT>>) -> usize {
        vi::PhysicalDevice::enumerate(&instance)
            .position(|dev| Self::is_device_suitable(surface, &dev))
            .expect("could not find suitable GPU")
    }

    fn required_extensions() -> vi::InstanceExtensions {
        let mut exts = vulkano_win::required_extensions();
        exts.ext_debug_report = true;
        exts
    }

    fn find_queue_families(surface: &Arc<vs::Surface<WT>>, device: &vi::PhysicalDevice) -> Option<QueueFamilyIndices> {
        let mut indices = QueueFamilyIndices::new();
        for (i, queue_family) in device.queue_families().enumerate() {
            if queue_family.supports_graphics() {
                indices.graphics_family = i as i32
            }
            if surface.is_supported(queue_family).unwrap() {
                indices.present_family = i as i32
            }
            if indices.is_complete() {
                return Some(indices);
            }
        }
        None
    }

    fn is_device_suitable(surface: &Arc<vs::Surface<WT>>, device: &vi::PhysicalDevice) -> bool {
        match Self::find_queue_families(surface, &device) {
            Some(_) => true,
            None => false
        }
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


