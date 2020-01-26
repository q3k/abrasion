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
pub struct Data {
    vertices: Rc<Vec<data::Vertex>>,
    indices: Rc<Vec<u16>>,
    transform: cgm::Matrix4<f32>,

    vbuffer: Option<Arc<vb::ImmutableBuffer<[data::Vertex]>>>,
    ibuffer: Option<Arc<vb::ImmutableBuffer<[u16]>>>,
}

impl Data {
    pub fn new(
        vertices: Rc<Vec<data::Vertex>>,
        indices: Rc<Vec<u16>>,
        transform: cgm::Matrix4<f32>,
    ) -> Data {
        Data {
            vertices, indices, transform,
            vbuffer: None,
            ibuffer: None,
        }
    }

    pub fn vulkan_buffers(
        &mut self,
        graphics_queue: Arc<vd::Queue>,
    ) -> (
        Arc<vb::ImmutableBuffer<[data::Vertex]>>,
        Arc<vb::ImmutableBuffer<[u16]>>,
    ) {

        let vbuffer = match &mut self.vbuffer {
            Some(v) => v.clone(),
            None => {
                let (vbuffer, future) = vb::immutable::ImmutableBuffer::from_iter(
                    self.vertices.iter().cloned(),
                    vb::BufferUsage::vertex_buffer(),
                    graphics_queue.clone(),
                ).unwrap();
                future.flush().unwrap();
                self.vbuffer = Some(vbuffer.clone());
                vbuffer.clone()
            },
        };

        let ibuffer = match &mut self.ibuffer {
            Some(v) => v.clone(),
            None => {
                let (ibuffer, future) = vb::immutable::ImmutableBuffer::from_iter(
                    self.indices.iter().cloned(),
                    vb::BufferUsage::index_buffer(),
                    graphics_queue.clone(),
                ).unwrap();
                future.flush().unwrap();
                self.ibuffer = Some(ibuffer.clone());
                ibuffer.clone()
            },
        };

        (vbuffer, ibuffer)
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
