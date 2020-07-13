use std::collections::HashMap;
use std::hash;
use std::sync::Arc;
use std::sync::Mutex;
use std::time;

use cgmath as cgm;
use image;
use image::GenericImageView;
use vulkano::device as vd;
use vulkano::buffer as vb;
use vulkano::sync::GpuFuture;
use vulkano::format as vf;
use vulkano::image as vm;

use crate::render::vulkan::data;
use crate::physics::color;
use crate::util::file;

pub struct ResourceManager {
    meshes: HashMap<u64, Mesh>,
    materials: HashMap<u64, Material>,
}

#[derive(Copy, Clone)]
pub enum ResourceID {
    Material(u64),
    Mesh(u64),
}

impl hash::Hash for ResourceID {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match self {
            ResourceID::Material(i) => i.hash(state),
            ResourceID::Mesh(i) => i.hash(state),
        }
    }
}

impl PartialEq for ResourceID {
    fn eq(&self, other: &Self) -> bool {
        let this = match self {
            ResourceID::Material(i) => i,
            ResourceID::Mesh(i) => i,
        };
        let that = match other {
            ResourceID::Material(i) => i,
            ResourceID::Mesh(i) => i,
        };
        this == that
    }
}

impl Eq for ResourceID {}

pub enum Resource {
    Material(Material),
    Mesh(Mesh),
}

impl<'a> ResourceManager {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
            materials: HashMap::new(),
        }
    }

    pub fn add_material(&mut self, t: Material) -> ResourceID {
        let id = t.id;
        self.materials.insert(id, t);
        ResourceID::Material(id)
    }

    pub fn add_mesh(&mut self, t: Mesh) -> ResourceID {
        let id = t.id;
        self.meshes.insert(id, t);
        ResourceID::Mesh(id)
    }

    pub fn material(&'a self, id: &ResourceID) -> Option<&'a Material> {
        if let ResourceID::Material(i) = id {
            return Some(self.materials.get(&i).unwrap());
        }
        return None
    }

    pub fn mesh(&'a self, id: &ResourceID) -> Option<&'a Mesh> {
        if let ResourceID::Mesh(i) = id {
            return Some(self.meshes.get(&i).unwrap());
        }
        return None
    }
}

pub struct Material {
    diffuse: ImageRefOrColor<color::XYZ>,
    roughness: ImageRefOrColor<color::LinearF32>,

    id: u64,
    // vulkan cache
    vulkan: Mutex<Option<data::Textures>>,
}

impl Material {
    pub fn new(
        diffuse: ImageRefOrColor<color::XYZ>,
        roughness: ImageRefOrColor<color::LinearF32>,
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

pub struct Mesh {
    vertices: Arc<Vec<data::Vertex>>,
    indices: Arc<Vec<u16>>,

    id: u64,
    // vulkan buffers cache
    vulkan: Mutex<Option<MeshVulkanData>>,
}

struct MeshVulkanData {
    vbuffer: Arc<vb::ImmutableBuffer<[data::Vertex]>>,
    ibuffer: Arc<vb::ImmutableBuffer<[u16]>>,
}

impl Mesh {
    pub fn new(
        vertices: Arc<Vec<data::Vertex>>,
        indices: Arc<Vec<u16>>,
    ) -> Self {
        Self {
            vertices, indices,
            // TODO: use a better method
            id: time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos() as u64,
            vulkan: Mutex::new(None),
        }
    }

    pub fn get_id(&self) -> u64 { self.id }

    pub fn vulkan_buffers(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> (
        Arc<vb::ImmutableBuffer<[data::Vertex]>>,
        Arc<vb::ImmutableBuffer<[u16]>>,
    ) {
        let mut cache = self.vulkan.lock().unwrap();
        match &mut *cache {
            Some(data) => (data.vbuffer.clone(), data.ibuffer.clone()),
            None => {
                let (vbuffer, vfuture) = vb::immutable::ImmutableBuffer::from_iter(
                    self.vertices.iter().cloned(),
                    vb::BufferUsage::vertex_buffer(),
                    graphics_queue.clone(),
                ).unwrap();
                let (ibuffer, ifuture) = vb::immutable::ImmutableBuffer::from_iter(
                    self.indices.iter().cloned(),
                    vb::BufferUsage::index_buffer(),
                    graphics_queue.clone(),
                ).unwrap();
                vfuture.flush().unwrap();
                ifuture.flush().unwrap();

                *cache = Some(MeshVulkanData {
                    vbuffer: vbuffer.clone(),
                    ibuffer: ibuffer.clone(),
                });

                (vbuffer.clone(), ibuffer.clone())
            },
        }
    }
}

impl hash::Hash for Mesh {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Mesh {}

pub trait Renderable {
    fn render_data(&self) -> Option<(ResourceID, ResourceID, &cgm::Matrix4<f32>)> {
        None
    }
}

pub struct Object {
    pub mesh: ResourceID,
    pub material: ResourceID,
    pub transform: cgm::Matrix4<f32>,
}

impl Renderable for Object {
    fn render_data(&self) -> Option<(ResourceID, ResourceID, &cgm::Matrix4<f32>)> {
        Some((self.mesh, self.material, &self.transform))
    }
}

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

pub enum ImageRefOrColor<T: ChannelLayout> {
    Color(T),
    ImageRef(ImageRef),
}

impl<T: ChannelLayout> ImageRefOrColor<T> {
    fn vulkan_image(&self, graphics_queue: Arc<vd::Queue>) -> Arc<vm::ImmutableImage<vf::Format>> {
        match self {
            ImageRefOrColor::<T>::Color(c) => c.vulkan_from_value(graphics_queue),
            ImageRefOrColor::<T>::ImageRef(r) => T::vulkan_from_image(r.load(), graphics_queue),
        }
    }

    pub fn color(color: T) -> Self {
        ImageRefOrColor::<T>::Color(color)
    }

    pub fn image(name: String) -> Self {
        ImageRefOrColor::<T>::ImageRef(ImageRef{ name })
    }
}

pub struct ImageRef {
    name: String,
}

impl ImageRef {
    fn load (&self) -> Arc<image::DynamicImage> {
        let path = &file::resource_path(self.name.clone());
        Arc::new(image::open(path).unwrap())
    }
}

