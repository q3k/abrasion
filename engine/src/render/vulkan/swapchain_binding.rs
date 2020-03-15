use std::sync::Arc;

use vulkano::swapchain as vs;
use vulkano::image as vm;
use vulkano::format as vf;
use vulkano::framebuffer as vfb;
use vulkano::sync as vy;

pub struct SwapchainBinding<WT> {
    pub chain: Arc<vs::Swapchain<WT>>,
    pub images: Vec<Arc<vm::SwapchainImage<WT>>>,
    pub render_pass: Arc<dyn vfb::RenderPassAbstract + Send + Sync>,
    pub framebuffers: Vec<Arc<dyn vfb::FramebufferAbstract + Send + Sync>>,
}

impl<WT: 'static + Send + Sync> SwapchainBinding<WT> {
    pub fn new(
        surface_binding: &super::surface_binding::SurfaceBinding<WT>,
        previous: Option<&SwapchainBinding<WT>>
    ) -> Self {
        let physical_device = surface_binding.physical_device();
        let capabilities = surface_binding.surface.capabilities(physical_device).expect("could not get capabilities");

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

        let indices = super::qfi::QueueFamilyIndices::find(&surface_binding.surface, &physical_device).unwrap();
        let sharing: vy::SharingMode = if indices.graphics_family != indices.present_family {
            vec![&surface_binding.graphics_queue, &surface_binding.present_queue].as_slice().into()
        } else {
            (&surface_binding.graphics_queue).into()
        };

        let prev = match previous {
            None => None,
            Some(p) => Some(p.chain.clone()),
        };

        let (chain, images) = vs::Swapchain::new(
            surface_binding.device.clone(),
            surface_binding.surface.clone(),
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
            prev.as_ref(),
        ).expect("could not create swap chain");

        log::info!("Swap chain: present mode {:?}, {} images", present_mode, images.len());

        let render_pass = Self::create_render_pass(surface_binding, chain.format());
        let framebuffers = Self::create_framebuffers(render_pass.clone(), images.clone());

        Self {
            chain,
            images,
            render_pass,
            framebuffers,
        }
    }

    fn create_render_pass(
        surface_binding: &super::surface_binding::SurfaceBinding<WT>,
        color_format: vulkano::format::Format,
    ) -> Arc<dyn vfb::RenderPassAbstract + Send + Sync> {
        let device = surface_binding.device.clone();

        Arc::new(vulkano::single_pass_renderpass!(device,
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: color_format,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap())
    }

    fn create_framebuffers(
        render_pass: Arc<dyn vfb::RenderPassAbstract + Send + Sync>,
        images: Vec<Arc<vm::SwapchainImage<WT>>>,
    ) -> Vec<Arc<dyn vfb::FramebufferAbstract + Send + Sync>> {
        images.iter()
            .map(|image| {
                let fba: Arc<dyn vfb::FramebufferAbstract + Send + Sync> = Arc::new(vfb::Framebuffer::start(render_pass.clone())
                    .add(image.clone()).unwrap()
                    .build().unwrap());
                fba
            })
        .collect::<Vec<_>>()
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
        //vs::PresentMode::Fifo
    }

    fn choose_swap_extent(capabilities: &vs::Capabilities) -> [u32; 2] {
        capabilities.current_extent.expect("could not get current extent")
    }

}

