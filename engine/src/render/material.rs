use std::sync::Arc;
use std::sync::Mutex;
use std::time;

use image;
use image::GenericImageView;
use vulkano::device as vd;
use vulkano::format as vf;
use vulkano::image as vm;
use vulkano::sync::GpuFuture;

use crate::physics::color;
use crate::render::vulkan::data;
use crate::util::file;

pub trait ChannelLayout {
    fn vulkan_from_image(
        image: Arc<image::DynamicImage>,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>>;

    fn vulkan_from_value(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>>;
}

impl ChannelLayout for color::XYZ {
    fn vulkan_from_image(
        image: Arc<image::DynamicImage>,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>> {
        let (width, height) = (image.width(), image.height());
        let rgba = image.to_rgba();
        // TODO(q3k): RGB -> CIE XYZ
        let (image_view, future) = vm::ImmutableImage::from_iter(
            rgba.into_raw().iter().cloned(),
            vm::Dimensions::Dim2d{ width, height },
            vf::Format::R8G8B8A8Unorm,
            graphics_queue.clone(),
        ).unwrap();

        future.flush().unwrap();
        image_view
    }

    fn vulkan_from_value(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>> {
        let mut image = image::ImageBuffer::<image::Rgba<f32>, Vec<f32>>::new(1, 1);
        image.put_pixel(0, 0, image::Rgba([self.x, self.y, self.z, 0.0]));

        let (image_view, future) = vm::ImmutableImage::from_iter(
            image.into_raw().iter().cloned(),
            vm::Dimensions::Dim2d{ width: 1, height: 1 },
            vf::Format::R32G32B32A32Sfloat,
            graphics_queue.clone(),
        ).unwrap();
        future.flush().unwrap();
        image_view
    }
}

impl ChannelLayout for color::LinearF32 {
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
        let (image_view, future) = vm::ImmutableImage::from_iter(
            gray.into_raw().iter().cloned(),
            vm::Dimensions::Dim2d{ width, height },
            vf::Format::R8G8B8A8Unorm,
            graphics_queue.clone(),
        ).unwrap();

        future.flush().unwrap();
        image_view
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
            vf::Format::R32Sfloat,
            graphics_queue.clone(),
        ).unwrap();
        
        future.flush().unwrap();
        image_view
    }
}

pub enum Texture<T: ChannelLayout> {
    Color(T),
    ImageRef(String),
}

impl<T: ChannelLayout> Texture<T> {
    fn vulkan_image(&self, graphics_queue: Arc<vd::Queue>) -> Arc<vm::ImmutableImage<vf::Format>> {
        match self {
            Texture::<T>::Color(c) => c.vulkan_from_value(graphics_queue),
            Texture::<T>::ImageRef(r) => {
                let path = &file::resource_path(r.clone());
                let img = Arc::new(image::open(path).unwrap());
                T::vulkan_from_image(img, graphics_queue)
            },
        }
    }

    pub fn from_color(color: T) -> Self {
        Texture::<T>::Color(color)
    }

    pub fn from_image(name: String) -> Self {
        Texture::<T>::ImageRef(name)
    }
}

pub struct Material {
    diffuse: Texture<color::XYZ>,
    roughness: Texture<color::LinearF32>,

    pub id: u64,
    // vulkan cache
    vulkan: Mutex<Option<data::Textures>>,
}

impl Material {
    pub fn new(
        diffuse: Texture<color::XYZ>,
        roughness: Texture<color::LinearF32>,
    ) -> Self {
        Self {
            diffuse,
            roughness,

            // TODO: use a better method
            id: time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos() as u64,
            vulkan: Mutex::new(None),
        }
    }

    pub fn vulkan_textures(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> data::Textures {
        let mut cache = self.vulkan.lock().unwrap();
        match &mut *cache {
            Some(data) => data.clone(),
            None => {
                let diffuse = self.diffuse.vulkan_image(graphics_queue.clone());
                let roughness = self.roughness.vulkan_image(graphics_queue.clone());
                let textures = data::Textures {
                    diffuse, roughness,
                };
                *cache = Some(textures.clone());
                textures
            },
        }
    }
}

pub struct PBRMaterialBuilder {
    pub diffuse: Texture<color::XYZ>,
    pub roughness: Texture<color::LinearF32>,
}

impl PBRMaterialBuilder {
    pub fn build(self) -> Material {
        Material::new(self.diffuse, self.roughness)
    }
}

