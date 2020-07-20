use std::collections::HashMap;
use std::hash;

use cgmath as cgm;

use crate::render::material::Material;
use crate::render::mesh::Mesh;

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

