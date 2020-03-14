use std::cell::RefCell;
use std::sync::Arc;
use std::rc::Rc;

use cgmath as cgm;
use vulkano::device as vd;
use vulkano::buffer as vb;
use vulkano::sync::{FenceSignalFuture, GpuFuture};

use crate::render::vulkan::data;

pub trait Renderable {
    fn data(&self) -> Option<Data> {
        None
    }
}

#[derive(Clone)]
struct VulkanData {
    vbuffer: Arc<vb::ImmutableBuffer<[data::Vertex]>>,
    ibuffer: Arc<vb::ImmutableBuffer<[u16]>>,
}

#[derive(Clone)]
pub struct Data {
    vertices: Rc<Vec<data::Vertex>>,
    indices: Rc<Vec<u16>>,
    transform: cgm::Matrix4<f32>,

    vulkan: RefCell<Option<VulkanData>>,
}

impl Data {
    pub fn new(
        vertices: Rc<Vec<data::Vertex>>,
        indices: Rc<Vec<u16>>,
        transform: cgm::Matrix4<f32>,
    ) -> Data {
        Data {
            vertices, indices, transform,
            vulkan: RefCell::new(None),
        }
    }

    pub fn vulkan_buffers(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> (
        Arc<vb::ImmutableBuffer<[data::Vertex]>>,
        Arc<vb::ImmutableBuffer<[u16]>>,
    ) {
        let mut cache = self.vulkan.borrow_mut();
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

    pub fn get_transform(&self) -> cgm::Matrix4<f32> {
        self.transform.clone()
    }
}

pub struct Mesh {
    pub vertices: Rc<Vec<data::Vertex>>,
    pub indices: Rc<Vec<u16>>,
    pub transform: cgm::Matrix4<f32>,
}

impl Renderable for Mesh {
    fn data(&self) -> Option<Data> {
        Some(Data::new(self.vertices.clone(), self.indices.clone(), self.transform.clone()))
    }
}
