use std::sync::Arc;

use vulkano::instance as vi;
use vulkano::swapchain as vs;

pub struct QueueFamilyIndices {
    pub graphics_family: i32,
    pub present_family: i32,
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


    pub fn find<WT>(surface: &Arc<vs::Surface<WT>>, device: &vi::PhysicalDevice) -> Option<Self> {
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
}
