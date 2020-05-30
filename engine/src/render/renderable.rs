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

pub struct ResourceManager {
    meshes: HashMap<u64, Mesh>,
    textures: HashMap<u64, Texture>,
}

#[derive(Copy, Clone)]
pub enum ResourceID {
    Texture(u64),
    Mesh(u64),
}

impl hash::Hash for ResourceID {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match self {
            ResourceID::Texture(i) => i.hash(state),
            ResourceID::Mesh(i) => i.hash(state),
        }
    }
}

impl PartialEq for ResourceID {
    fn eq(&self, other: &Self) -> bool {
        let this = match self {
            ResourceID::Texture(i) => i,
            ResourceID::Mesh(i) => i,
        };
        let that = match other {
            ResourceID::Texture(i) => i,
            ResourceID::Mesh(i) => i,
        };
        this == that
    }
}

impl Eq for ResourceID {}

pub enum Resource {
    Texture(Texture),
    Mesh(Mesh),
}

impl<'a> ResourceManager {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn add(&mut self, r: Resource) -> ResourceID {
        match r {
            Resource::Texture(t) => {
                let id = t.id;
                self.textures.insert(id, t);
                ResourceID::Texture(id)
            }
            Resource::Mesh(t) => {
                let id = t.id;
                self.meshes.insert(id, t);
                ResourceID::Mesh(id)
            }
        }
    }

    pub fn texture(&'a self, id: &ResourceID) -> Option<&'a Texture> {
        if let ResourceID::Texture(i) = id {
            return Some(self.textures.get(&i).unwrap());
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

pub struct Texture {
    image: Arc<image::DynamicImage>,

    id: u64,
    // vulkan cache
    vulkan: Mutex<Option<Arc<vm::ImmutableImage<vf::Format>>>>,
}
impl Texture {
    pub fn new(
        image: Arc<image::DynamicImage>,
    ) -> Self {
        Self {
            image,
            // TODO: use a better method
            id: time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos() as u64,
            vulkan: Mutex::new(None),
        }
    }
    pub fn vulkan_texture(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> Arc<vm::ImmutableImage<vf::Format>> {
        let mut cache = self.vulkan.lock().unwrap();
        match &mut *cache {
            Some(data) => data.clone(),
            None => {
                let width = self.image.width();
                let height = self.image.height();
                let image_rgba = self.image.to_rgba();
                let (image_view, future) = vm::ImmutableImage::from_iter(
                    image_rgba.into_raw().iter().cloned(),
                    vm::Dimensions::Dim2d{ width, height },
                    vf::Format::R8G8B8A8Unorm,
                    graphics_queue.clone(),
                ).unwrap();

                future.flush().unwrap();

                *cache = Some(image_view.clone());

                image_view
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
    pub texture: ResourceID,
    pub transform: cgm::Matrix4<f32>,
}

impl Renderable for Object {
    fn render_data(&self) -> Option<(ResourceID, ResourceID, &cgm::Matrix4<f32>)> {
        Some((self.mesh, self.texture, &self.transform))
    }
}
