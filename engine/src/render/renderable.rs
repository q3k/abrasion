use std::hash;
use std::sync::Arc;
use std::sync::Mutex;
use std::time;

use cgmath as cgm;
use vulkano::device as vd;
use vulkano::buffer as vb;
use vulkano::sync::GpuFuture;

use crate::render::vulkan::data;

pub trait Renderable {
    fn render_data(&self) -> Option<(Arc<Mesh>, cgm::Matrix4<f32>)> {
        None
    }
}

struct VulkanData {
    vbuffer: Arc<vb::ImmutableBuffer<[data::Vertex]>>,
    ibuffer: Arc<vb::ImmutableBuffer<[u16]>>,
}

pub struct Mesh {
    vertices: Arc<Vec<data::Vertex>>,
    indices: Arc<Vec<u16>>,

    id: u64,
    // vulkan buffers cache
    vulkan: Mutex<Option<VulkanData>>,
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

                *cache = Some(VulkanData {
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

pub struct Object {
    pub mesh: Arc<Mesh>,
    pub transform: cgm::Matrix4<f32>,
}

impl Renderable for Object {
    fn render_data(&self) -> Option<(Arc<Mesh>, cgm::Matrix4<f32>)> {
        Some((self.mesh.clone(), self.transform.clone()))
    }
}
