use std::sync::Arc;

use image;
use image::GenericImageView;

use vulkano::image as vm;
use vulkano::format as vf;
use vulkano::sync::GpuFuture;

pub fn load_texture<WT: 'static + Send + Sync>(
    surface_binding: &super::surface_binding::SurfaceBinding<WT>,
    name: String,
) -> Arc<vm::ImmutableImage<vf::Format>> {
    let path = &crate::util::file::resource_path(name);

    let image = image::open(path).unwrap();
    let width = image.width();
    let height = image.height();

    let image_rgba = image.to_rgba();
    let (image_view, future) = vm::ImmutableImage::from_iter(
        image_rgba.into_raw().iter().cloned(),
        vm::Dimensions::Dim2d{ width, height },
        vf::Format::R8G8B8A8Unorm,
        surface_binding.graphics_queue.clone()
    ).unwrap();

    future.flush().unwrap();

    image_view
}

