// Copyright 2020 Sergiusz 'q3k' Bazanski <q3k@q3k.org>
//
// This file is part of Abrasion.
//
// Abrasion is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, version 3.
//
// Abrasion is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// Abrasion.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::Arc;

use vulkano::swapchain as vs;
use vulkano::image as vm;
use vulkano::image::traits::ImageAccess;
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
            transfer_destination: true,
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

        let (chain, images) = match prev.as_ref() {
            None => vs::Swapchain::new(
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
                vs::FullscreenExclusive::Default,
                true,
                vs::ColorSpace::SrgbNonLinear,
            ),
            Some(p) => vs::Swapchain::with_old_swapchain(
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
                vs::FullscreenExclusive::Default,
                true,
                vs::ColorSpace::SrgbNonLinear,
                p.clone(),
            ),
        }.expect("could not create swap chain");

        log::info!("Swap chain: present mode {:?}, {} images", present_mode, images.len());

        let depth_format = Self::find_depth_format();
        // TODO(q3k): make configurable and check with hardware
        let sample_count = 8;

        let depth_image = vm::AttachmentImage::multisampled_with_usage(
            surface_binding.device.clone(),
            chain.dimensions(),
            sample_count,
            depth_format,
            vm::ImageUsage { depth_stencil_attachment: true, ..vm::ImageUsage::none() },
        ).unwrap();
        let render_pass = Self::create_render_pass(surface_binding, chain.format(), depth_format, sample_count);
        let framebuffers = Self::create_framebuffers(surface_binding, render_pass.clone(), images.clone(), depth_image, sample_count);

        Self {
            chain,
            images,
            render_pass,
            framebuffers,
        }
    }

    fn create_render_pass(
        surface_binding: &super::surface_binding::SurfaceBinding<WT>,
        color_format: vf::Format,
        depth_format: vf::Format,
        sample_count: u32,
    ) -> Arc<dyn vfb::RenderPassAbstract + Send + Sync> {
        let device = surface_binding.device.clone();

        Arc::new(vulkano::single_pass_renderpass!(device,
            attachments: {
                multisample_color: {
                    load: Clear,
                    store: Store,
                    format: color_format,
                    samples: sample_count,
                },
                multisample_depth: {
                    load: Clear,
                    store: DontCare,
                    format: depth_format,
                    samples: sample_count,
                    initial_layout: ImageLayout::Undefined,
                    final_layout: ImageLayout::DepthStencilAttachmentOptimal,
                },
                resolve_color: {
                    load: DontCare,
                    store: Store,
                    format: color_format,
                    samples: 1,
                    initial_layout: ImageLayout::Undefined,
                }
            },
            pass: {
                color: [multisample_color],
                depth_stencil: {multisample_depth},
                resolve: [resolve_color]
            }
        ).unwrap())
    }

    fn create_framebuffers(
        surface_binding: &super::surface_binding::SurfaceBinding<WT>,
        render_pass: Arc<dyn vfb::RenderPassAbstract + Send + Sync>,
        images: Vec<Arc<vm::SwapchainImage<WT>>>,
        depth_image: Arc<vm::AttachmentImage<vf::Format>>,
        sample_count: u32,
    ) -> Vec<Arc<dyn vfb::FramebufferAbstract + Send + Sync>> {
        let device = surface_binding.device.clone();
        images.iter()
            .map(|image| {
                let dim = image.dimensions().width_height();
                let multisample_image = vm::AttachmentImage::transient_multisampled(device.clone(), dim, sample_count, image.format()).unwrap();
                let fba: Arc<dyn vfb::FramebufferAbstract + Send + Sync> = Arc::new(vfb::Framebuffer::start(render_pass.clone())
                    .add(multisample_image.clone()).unwrap()
                    .add(depth_image.clone()).unwrap()
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

    fn find_depth_format() -> vf::Format {
        // TODO: actually do it
        vf::Format::D16Unorm
    }
}

