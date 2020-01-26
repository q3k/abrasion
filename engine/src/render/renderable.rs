use std::sync::Arc;

use cgmath as cgm;

use crate::render::vulkan::data;

pub trait Renderable {
    fn data(&self) -> Option<Data> {
        None
    }
}

#[derive(Clone)]
pub struct Data {
    pub vertices: Arc<Vec<data::Vertex>>,
    pub indices: Arc<Vec<u16>>,
    pub transform: cgm::Matrix4<f32>,
}

pub struct Mesh {
    pub vertices: Arc<Vec<data::Vertex>>,
    pub indices: Arc<Vec<u16>>,
    pub transform: cgm::Matrix4<f32>,
}

impl Renderable for Mesh {
    fn data(&self) -> Option<Data> {
        Some(Data {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
            transform: self.transform.clone(),
        })
    }
}
