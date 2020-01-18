use std::collections::HashSet;
use std::sync::Arc;

use vulkano::instance as vi;
use vulkano::device as vd;
use vulkano::swapchain as vs;

// A Binding to a surface, resulting in concrete device information.
pub struct Binding<WT> {
    instance: Arc<vi::Instance>,
    physical_device_ix: usize,
    pub device: Arc<vd::Device>,
    pub surface: Arc<vs::Surface<WT>>,
    pub graphics_queue: Arc<vd::Queue>,
    pub present_queue: Arc<vd::Queue>,
}

impl <WT> Binding<WT> {
    pub fn physical_device(&self) -> vi::PhysicalDevice {
        vi::PhysicalDevice::from_index(&self.instance, self.physical_device_ix).unwrap()
    }

    pub fn new(instance: &Arc<vi::Instance>, surface: &Arc<vs::Surface<WT>>) -> Self {
        let physical_device_ix = Self::pick_physical_device(instance, &surface);
        let physical_device = vi::PhysicalDevice::from_index(instance, physical_device_ix).unwrap();
        let indices = super::qfi::QueueFamilyIndices::find(&surface, &physical_device).unwrap();

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
            &required_device_extensions(),
            qf
        ).expect("could not create logical device and queues");

        let graphics_queue = queues.next().unwrap();
        let present_queue = queues.next().unwrap_or_else(|| graphics_queue.clone());

        Self {
            instance: instance.clone(),
            physical_device_ix,
            device: device.clone(),
            surface: surface.clone(),
            graphics_queue: graphics_queue.clone(),
            present_queue: present_queue.clone(),
        }
    }

    fn pick_physical_device(instance: &Arc<vi::Instance>, surface: &Arc<vs::Surface<WT>>) -> usize {
        vi::PhysicalDevice::enumerate(&instance)
            .position(|dev| Self::is_device_suitable(surface, &dev))
            .expect("could not find suitable GPU")
    }

    fn is_device_suitable(surface: &Arc<vs::Surface<WT>>, device: &vi::PhysicalDevice) -> bool {
        if !super::qfi::QueueFamilyIndices::find(surface, &device).is_some() {
            return false;
        }

        let available_extensions = vd::DeviceExtensions::supported_by_device(*device);
        let want_extensions = required_device_extensions();
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
}

fn required_device_extensions() -> vd::DeviceExtensions {
    vd::DeviceExtensions {
        khr_swapchain: true,
        .. vd::DeviceExtensions::none()
    }
}

