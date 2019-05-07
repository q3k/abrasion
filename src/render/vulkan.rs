use std::collections::HashSet;
use std::sync::Arc;
use log;

use vulkano::instance as vi;
use vulkano::device as vd;
use vulkano::swapchain as vs;
use vulkano::image as vm;
use vulkano::format as vf;
use vulkano::sync as vy;

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

    swap_chain: Option<Arc<vs::Swapchain<WT>>>,
    swap_chain_images: Option<Vec<Arc<vm::SwapchainImage<WT>>>>,
}

impl<WT> Instance<WT> {
    pub fn new(name: String) -> Self {
        let ai = vi::ApplicationInfo {
            application_name: Some(name.clone().into()),
            application_version: Some(VERSION),
            engine_name: Some(name.clone().into()),
            engine_version: Some(VERSION),
        };

        let exts = Self::required_instance_extensions();
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

            swap_chain: None,
            swap_chain_images: None,
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

        let (device, mut queues) = vd::Device::new(
            physical_device,
            &vd::Features::none(),
            &Self::required_device_extensions(),
            qf
        ).expect("could not create logical device and queues");

        let graphics_queue = queues.next().unwrap();
        let present_queue = queues.next().unwrap_or_else(|| graphics_queue.clone());

        self.physical_device_ix = Some(physical_device_ix);
        self.device = Some(device.clone());
        self.surface = Some(surface.clone());
        self.graphics_queue = Some(graphics_queue.clone());
        self.present_queue = Some(present_queue.clone());

        let capabilities = surface.capabilities(physical_device).expect("could not get capabilities");
        let surface_format = Self::choose_swap_surface_format(&capabilities.supported_formats);
        let present_mode = Self::choose_swap_present_mode(capabilities.present_modes);
        let extent = Self::choose_swap_extent(&capabilities);

        let mut image_count = capabilities.min_image_count + 1;
        if let Some(max_image_count) = capabilities.max_image_count {
            if image_count > max_image_count {
                image_count = max_image_count;
            }
        }

        let image_usage = vm::ImageUsage {
            color_attachment: true,
            .. vm::ImageUsage::none()
        };

        let sharing: vy::SharingMode = if indices.graphics_family != indices.present_family {
            vec![&graphics_queue, &present_queue].as_slice().into()
        } else {
            (&graphics_queue).into()
        };

        let (swap_chain, images) = vs::Swapchain::new(
            device.clone(),
            surface.clone(),
            image_count,
            surface_format.0,
            extent,
            1,
            image_usage,
            sharing,
            capabilities.current_transform,
            vs::CompositeAlpha::Opaque,
            present_mode,
            true,
            None,
        ).expect("could not create swap chain");

        self.swap_chain = Some(swap_chain);
        self.swap_chain_images = Some(images);
    }

    pub fn get_vulkan(&self) -> Arc<vi::Instance> {
        self.vulkan.clone()
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

    fn pick_physical_device(instance: &Arc<vi::Instance>, surface: &Arc<vs::Surface<WT>>) -> usize {
        vi::PhysicalDevice::enumerate(&instance)
            .position(|dev| Self::is_device_suitable(surface, &dev))
            .expect("could not find suitable GPU")
    }

    fn is_device_suitable(surface: &Arc<vs::Surface<WT>>, device: &vi::PhysicalDevice) -> bool {
        if !Self::find_queue_families(surface, &device).is_some() {
            return false;
        }

        let available_extensions = vd::DeviceExtensions::supported_by_device(*device);
        let want_extensions = Self::required_device_extensions();
        if available_extensions.intersection(&want_extensions) != want_extensions {
            return false;
        }

        let capabilities = surface.capabilities(*device).expect("could not get device capabilities");
        if capabilities.supported_formats.is_empty() {
            return false;
        }
        if !capabilities.present_modes.iter().next().is_some() {
            return false;
        }

        true
    }

    fn required_device_extensions() -> vd::DeviceExtensions {
        vd::DeviceExtensions {
            khr_swapchain: true,
            .. vd::DeviceExtensions::none()
        }
    }

    fn required_instance_extensions() -> vi::InstanceExtensions {
        let mut exts = vulkano_win::required_extensions();
        exts.ext_debug_report = true;
        exts
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

    fn choose_swap_surface_format(available_formats: &[(vf::Format, vs::ColorSpace)]) -> (vf::Format, vs::ColorSpace) {
        *available_formats.iter()
            .find(|(format, color_space)|
                *format == vf::Format::B8G8R8A8Unorm && *color_space == vs::ColorSpace::SrgbNonLinear
            )
            .unwrap_or_else(|| &available_formats[0])
    }

    fn choose_swap_present_mode(available_present_modes: vs::SupportedPresentModes) -> vs::PresentMode {
        if available_present_modes.mailbox {
            vs::PresentMode::Mailbox
        } else if available_present_modes.immediate {
            vs::PresentMode::Immediate
        } else {
            vs::PresentMode::Fifo
        }
    }

    fn choose_swap_extent(capabilities: &vs::Capabilities) -> [u32; 2] {
        capabilities.current_extent.expect("could not get current extent")
    }
}


