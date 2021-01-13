use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::iter::Iterator;
use std::cell::{Ref, RefMut, RefCell};

use crate::componentmap::{
    ComponentMap,
    ComponentMapIter,
    ComponentMapIterMut,
};
use crate::resourcemap::{
    ResourceMap,
    ResourceRef,
};
use crate::entity;
use crate::component;

pub struct ReadData<'a, T: component::Component> {
    world: &'a World,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: component::Component> ReadData<'a, T> {
    pub fn iter(&self) -> ReadDataIter<'a, T> {
        let cm = self.world.components.get(&component::component_id::<T>());
        ReadDataIter {
            phantom: PhantomData,
            iter: cm.map(|e| e.try_iter().unwrap() ),
        }
    }
}

pub struct ReadDataIter<'a, T: component::Component> {
    phantom: PhantomData<&'a T>,
    iter: Option<ComponentMapIter<'a>>,
}

impl <'a, T: component::Component> Iterator for ReadDataIter<'a, T> {
    type Item = (entity::ID, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_none() {
            return None;
        }
        match self.iter.as_mut().unwrap().iter.next() {
            None => None,
            Some((eid, component)) => {
                let component = component.as_ref();
                let component = unsafe { & *(component as *const (dyn component::Component) as *const T) };
                Some((*eid, component))
            },
        }
    }
}

pub struct ReadWriteData<'a, T: component::Component> {
    world: &'a World,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: component::Component> ReadWriteData<'a, T> {
    pub fn iter_mut(&self) -> ReadWriteDataIter<'a, T> {
        let cm = self.world.components.get(&component::component_id::<T>());
        ReadWriteDataIter {
            phantom: PhantomData,
            iter: cm.map(|e| e.try_iter_mut().unwrap() ),
        }
    }
}

pub struct ReadWriteDataIter<'a, T: component::Component> {
    phantom: PhantomData<&'a T>,
    iter: Option<ComponentMapIterMut<'a>>,
}

impl <'a, T: component::Component> Iterator for ReadWriteDataIter<'a, T> {
    type Item = (entity::ID, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_none() {
            return None;
        }
        match self.iter.as_mut().unwrap().iter.next() {
            None => None,
            Some((eid, component)) => {
                let component = component.as_mut();
                let component = unsafe { &mut *(component as *mut (dyn component::Component) as *mut T) };
                Some((*eid, component))
            },
        }
    }
}

pub struct ReadResource<'a, T: component::Resource> {
    world: &'a World,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: component::Resource> ReadResource<'a, T> {
    pub fn get(&self) -> ResourceRef<'a, T> {
        self.world.resources.get::<'a, T>().unwrap()
    }
}

pub struct World {
    components: BTreeMap<component::ID, ComponentMap>,
    resources: ResourceMap,
    next_id: entity::ID,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: BTreeMap::new(),
            resources: ResourceMap::new(),
            next_id: 1u64,
        }
    }

    pub fn new_entity(&mut self) -> entity::EntityBuilder {
        let id = self.next_id;
        self.next_id += 1;
        entity::EntityBuilder::new(self, id)
    }

    pub fn register_component_entity(
        &mut self,
        cid: component::ID,
        c: Box<dyn component::Component>,
        e: entity::Entity
    ) {
        let map = self.components.entry(cid).or_insert(ComponentMap::new());
        map.insert(e.id(), c).unwrap();
    }

    pub fn components<'a, T: component::Component>(&'a self) -> ReadData<'a, T> {
        ReadData {
            world: self,
            phantom: PhantomData,
        }
    }

    pub fn components_mut<'a, T: component::Component>(&'a self) -> ReadWriteData<'a, T> {
        ReadWriteData {
            world: self,
            phantom: PhantomData,
        }
    }

    pub fn resource<'a, T: component::Resource>(&'a self) -> ReadResource<'a, T> {
        ReadResource {
            world: self,
            phantom: PhantomData,
        }
    }

    pub fn set_resource<T: component::Resource>(&self, r: T) {
        self.resources.set(r).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::component;
    use crate::world;

    #[derive(Debug)]
    struct Position {
        x: u32,
        y: u32,
        z: u32,
    }
    impl component::Component for Position {}

    #[derive(Debug)]
    struct Name(String);
    impl component::Component for Name {}
    impl Name {
        fn new(s: &str) -> Name {
            Name(String::from(s))
        }
    }

    #[test]
    fn new_list() {
        let mut world = world::World::new();
        world.new_entity().with(Name::new("foo")).with(Position { x: 1, y: 2, z: 3 }).build();
        world.new_entity().with(Name::new("bar")).with(Position { x: 4, y: 5, z: 6 }).build();

        let mut named = world.components::<Name>().iter();
        let mut named2 = world.components::<Name>().iter();
        assert_eq!(String::from("foo"), (named.next().unwrap().1).0);
        assert_eq!(String::from("foo"), (named2.next().unwrap().1).0);
        assert_eq!(String::from("bar"), (named.next().unwrap().1).0);
        assert_eq!(String::from("bar"), (named2.next().unwrap().1).0);
    }
}
