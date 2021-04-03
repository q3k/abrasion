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

use image;
use image::GenericImageView;
use vulkano::command_buffer as vcb;
use vulkano::buffer as vb;
use vulkano::device as vd;
use vulkano::format as vf;
use vulkano::image as vm;
use vulkano::sampler as vs;
use vulkano::sync::GpuFuture;
use vulkano::command_buffer::CommandBuffer;

use crate::physics::color;

/// Construct a mipmapped vulkan image from an iterator of raw data.
fn mipmapped_from_iter<P, I, F>(
    width: u32, height: u32,
    format: F,
    data: I,
    graphics_queue: Arc<vd::Queue>,
) -> Arc<vm::ImmutableImage<F>> where
    P: Send + Sync + Clone + 'static,
    I: ExactSizeIterator<Item = P>,
    F: vf::FormatDesc + vf::AcceptsPixels<P> + 'static + Send + Sync + Copy,
    vf::Format: vf::AcceptsPixels<P>,
{
    let dimensions = vm::Dimensions::Dim2d{ width, height };

    let image_usage = vm::ImageUsage {
        transfer_destination: true,
        transfer_source: true,
        sampled: true,
        ..vm::ImageUsage::none()
    };

    // Make a temporary image to generate mipmaps from. This is used to bypass
    // a bug in Vulkano's AutoCommandBufferBuilder that prevents blitting
    // inside the same image across different mipmaps. This is okay, as we plan
    // to move mipmap generation to compile-time anyway.

    let source = vb::CpuAccessibleBuffer::from_iter(
        graphics_queue.device().clone(),
        vb::BufferUsage::transfer_source(),
        false,
        data,
    ).unwrap();

    let (mipmap_source_image, mipmap_source_image_init) = vm::ImmutableImage::uninitialized(
        graphics_queue.clone().device().clone(),
        dimensions,
        format,
        vm::MipmapsCount::One,
        vm::ImageUsage {
            transfer_destination: true,
            transfer_source: true,
            sampled: true,
            ..vm::ImageUsage::none()
        },
        vm::ImageLayout::ShaderReadOnlyOptimal,
        graphics_queue.device().active_queue_families(),
    ).unwrap();

    let (image, image_init) = vm::ImmutableImage::uninitialized(
        graphics_queue.clone().device().clone(),
        dimensions,
        format,
        vm::MipmapsCount::Log2,
        image_usage,
        vm::ImageLayout::ShaderReadOnlyOptimal,
        graphics_queue.device().active_queue_families(),
    ).unwrap();

    let mut cb = vcb::AutoCommandBufferBuilder::new(graphics_queue.device().clone(), graphics_queue.family()).unwrap();

    // Transfer buffer into mipmap_source_image.
    cb.copy_buffer_to_image_dimensions(
        source, mipmap_source_image_init,
        [0, 0, 0], dimensions.width_height_depth(), 0,
        dimensions.array_layers_with_cube(), 0,
    ).unwrap();

    // Copy mip level 0 (original image) using image_init.
    cb.blit_image(
        mipmap_source_image.clone(), [0, 0, 0], [width as i32, height as i32, 1], 0, 0,
        image_init, [0, 0, 0], [width as i32, height as i32, 1], 0, 0,
        1, vs::Filter::Linear
    ).unwrap();


    // Generates all other mip levels using image.
    let img_dimensions = vm::ImageAccess::dimensions(&image);
    for mip_idx in 1..image.mipmap_levels() {
        let dest_dim = img_dimensions.mipmap_dimensions(mip_idx).unwrap();
        cb.blit_image(
            mipmap_source_image.clone(), [0, 0, 0], [width as i32, height as i32, 1], 0, 0,
            image.clone(), [0, 0, 0], [dest_dim.width() as i32, dest_dim.height() as i32, 1i32], 0, mip_idx,
            1, vs::Filter::Linear
        ).unwrap();
    }

    let future = cb.build().unwrap().execute(graphics_queue.clone()).unwrap();

    future.flush().unwrap();
    image
}

/// Represents a layout of color channels whose users (single colors and textures) can be converted
/// to Vulkan images.
pub trait ChannelLayoutVulkan {
    fn vulkan_from_image(
        image: Arc<image::DynamicImage>,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>>;

    fn vulkan_from_value(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>>;
}

impl ChannelLayoutVulkan for color::XYZ {
    fn vulkan_from_image(
        image: Arc<image::DynamicImage>,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>> {
        let (width, height) = (image.width(), image.height());
        let rgba = image.to_rgba();

        let mut xyz = Vec::new();
        for (_, _, color) in rgba.enumerate_pixels() {
            let image::Rgba([r, g, b, a]) = color;
            let r = (*r as f32) / 255.0;
            let g = (*g as f32) / 255.0;
            let b = (*b as f32) / 255.0;
            let a = (*a as f32) / 255.0;
            let (x, y, z) = color::srgb_to_cie_xyz(r, g, b);
            xyz.push(x);
            xyz.push(y);
            xyz.push(z);
            xyz.push(a);
        }

        // TODO(q3k): RGB -> CIE XYZ
        mipmapped_from_iter(width, height, vf::Format::R32G32B32A32Sfloat, xyz.into_iter(), graphics_queue)
    }

    fn vulkan_from_value(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>> {
        let (image_view, future) = vm::ImmutableImage::from_iter(
            vec!([self.x, self.y, self.z, 1.0 as f32]).into_iter(),
            vm::Dimensions::Dim2d{ width: 1, height: 1 },
            vm::MipmapsCount::One,
            vf::Format::R32G32B32A32Sfloat,
            graphics_queue.clone(),
        ).unwrap();
        future.flush().unwrap();
        image_view
    }
}

impl ChannelLayoutVulkan for color::LinearF32 {
    fn vulkan_from_image(
        image: Arc<image::DynamicImage>,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>> {
        let (width, height) = (image.width(), image.height());
        assert!(match image.color() {
            image::ColorType::L8 => true,
            image::ColorType::L16 => true,
            _ => false,
        }, "linearf32 texture must be 8-bit grayscale");
        let gray = image.to_luma();

        mipmapped_from_iter(width, height, vf::Format::R8Unorm, gray.into_raw().iter().cloned(), graphics_queue)
    }

    fn vulkan_from_value(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>> {
        let mut image = image::ImageBuffer::<image::Luma<f32>, Vec<f32>>::new(1, 1);
        image.put_pixel(0, 0, image::Luma([self.d]));

        let (image_view, future) = vm::ImmutableImage::from_iter(
            image.into_raw().iter().cloned(),
            vm::Dimensions::Dim2d{ width: 1, height: 1 },
            vm::MipmapsCount::One,
            vf::Format::R32Sfloat,
            graphics_queue.clone(),
        ).unwrap();
        
        future.flush().unwrap();
        image_view
    }
}

fn get_mip_dim(mip_idx: u32, img_dimensions: vm::ImageDimensions) -> Result<[i32; 3], String> {
    if let Some(dim) = img_dimensions.mipmap_dimensions(mip_idx) {
        if let vm::ImageDimensions::Dim2d { width, height, .. } = dim {
            Ok([width as i32, height as i32, 1])
        } else {
            Err("MipMapping: Did not get 2D image for blitting".to_string())
        }
    } else {
        Err(format!("MipMapping: image has no mip map at level {}", mip_idx).to_string())
    }
}
