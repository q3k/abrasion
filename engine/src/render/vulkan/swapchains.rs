use std::sync::Arc;

use vulkano::swapchain as vs;
use vulkano::image as vm;
use vulkano::format as vf;
use vulkano::sync as vy;

pub struct Swapchains<WT> {
    chain: Arc<vs::Swapchain<WT>>,
    images: Vec<Arc<vm::SwapchainImage<WT>>>,
}

impl<WT> Swapchains<WT> {
    pub fn new(binding: &super::binding::Binding<WT>) -> Self {
        let physical_device = binding.physical_device();
        let capabilities = binding.surface.capabilities(physical_device).expect("could not get capabilities");

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

        let indices = super::qfi::QueueFamilyIndices::find(&binding.surface, &physical_device).unwrap();
        let sharing: vy::SharingMode = if indices.graphics_family != indices.present_family {
            vec![&binding.graphics_queue, &binding.present_queue].as_slice().into()
        } else {
            (&binding.graphics_queue).into()
        };

        let (chain, images) = vs::Swapchain::new(
            binding.device.clone(),
            binding.surface.clone(),
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

        Self {
            chain, images
        }
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

