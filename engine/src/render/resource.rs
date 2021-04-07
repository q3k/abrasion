use std::collections::BTreeMap;
use std::cmp::Ordering;

use crate::render::{Mesh, Material, Light};

type Map<T> = BTreeMap<ResourceID<T>, T>;

pub struct Manager {
    meshes: Map<Mesh>,
    materials: Map<Material>,
    lights: Map<Light>,

    label_to_numeric: BTreeMap<String, u64>,
    counter: u64,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            meshes: BTreeMap::new(),
            materials: BTreeMap::new(),
            lights: BTreeMap::new(),

            label_to_numeric: BTreeMap::new(),
            counter: 0,
        }
    }

    fn map<T: Resource>(&self) -> &Map<T> {
        T::map(&self)
    }

    pub fn add<T: Resource, S: ToString>(&mut self, r: T, label: Option<S>) -> ResourceID<T> {
        let numeric = self.counter;
        if let Some(label) = label {
            self.label_to_numeric.insert(label.to_string(), numeric);
        }
        let id = ResourceID {
            numeric,
            phantom: std::marker::PhantomData,
        };

        self.counter += 1;
        T::map_mut(self).insert(id, r);
        id
    }

    pub fn by_label<T: Resource, S: ToString>(&self, label: S) -> Option<&T> {
        let label = label.to_string();
        let numeric = self.label_to_numeric.get(&label)?.clone();
        let rid = ResourceID {
            numeric,
            phantom: std::marker::PhantomData,
        };
        T::map(self).get(&rid)
    }
}

pub trait Resource: Sized {
    fn map(rm: &Manager) -> &Map<Self>;
    fn map_mut(rm: &mut Manager) -> &mut Map<Self>;
}

impl Resource for Light {
    fn map(rm: &Manager) -> &Map<Self> { &rm.lights }
    fn map_mut(rm: &mut Manager) -> &mut Map<Self> { &mut rm.lights }
}
impl Resource for Mesh {
    fn map(rm: &Manager) -> &Map<Self> { &rm.meshes }
    fn map_mut(rm: &mut Manager) -> &mut Map<Self> { &mut rm.meshes }
}
impl Resource for Material {
    fn map(rm: &Manager) -> &Map<Self> { &rm.materials }
    fn map_mut(rm: &mut Manager) -> &mut Map<Self> { &mut rm.materials }
}

#[derive(Debug)]
pub struct ResourceID<T: Resource> {
    numeric: u64,
    phantom: std::marker::PhantomData<T>,
}

impl <T: Resource> Clone for  ResourceID<T> {
    fn clone(&self) -> ResourceID<T> {
        ResourceID {
            numeric: self.numeric.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl <T: Resource> Copy for  ResourceID<T> {}

impl <T: Resource> ResourceID<T> {
    pub fn get(self, rm: &Manager) -> &T {
        rm.map::<T>().get(&self).unwrap()
    }
}

impl <T: Resource> Ord for ResourceID<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.numeric.cmp(&other.numeric)
    }
}

impl <T: Resource> PartialOrd for ResourceID<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <T: Resource> PartialEq for ResourceID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.numeric == other.numeric
    }
}

impl <T: Resource> Eq for ResourceID<T> {}

