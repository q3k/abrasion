use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::iter::Iterator;

use crate::componentmap::{
    ComponentMap,
    ComponentMapIter,
    ComponentMapIterMut,
};
use crate::globalmap::{
    GlobalMap,
    GlobalRef,
    GlobalRefMut,
};
use crate::entity;
use crate::component;

pub struct ReadComponent<'a, T: component::Component> {
    world: &'a World,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: component::Component> ReadComponent<'a, T> {
    pub fn iter(&self) -> ReadComponentIter<'a, T> {
        let cm = self.world.components.get(&component::component_id::<T>());
        ReadComponentIter {
            phantom: PhantomData,
            iter: cm.map(|e| e.try_iter().unwrap() ),
        }
    }
}

pub struct ReadComponentIter<'a, T: component::Component> {
    phantom: PhantomData<&'a T>,
    iter: Option<ComponentMapIter<'a>>,
}

impl <'a, T: component::Component> Iterator for ReadComponentIter<'a, T> {
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

pub struct ReadWriteComponent<'a, T: component::Component> {
    world: &'a World,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: component::Component> ReadWriteComponent<'a, T> {
    pub fn iter_mut(&self) -> ReadWriteComponentIter<'a, T> {
        let cm = self.world.components.get(&component::component_id::<T>());
        ReadWriteComponentIter {
            phantom: PhantomData,
            iter: cm.map(|e| e.try_iter_mut().unwrap() ),
        }
    }
}

pub struct ReadWriteComponentIter<'a, T: component::Component> {
    phantom: PhantomData<&'a T>,
    iter: Option<ComponentMapIterMut<'a>>,
}

impl <'a, T: component::Component> Iterator for ReadWriteComponentIter<'a, T> {
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

pub struct ReadGlobal<'a, T: component::Global> {
    world: &'a World,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: component::Global> ReadGlobal<'a, T> {
    pub fn get(&self) -> GlobalRef<'a, T> {
        self.world.globals.get::<T>().unwrap()
    }
}

pub struct ReadWriteGlobal<'a, T: component::Global> {
    world: &'a World,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: component::Global> ReadWriteGlobal<'a, T> {
    pub fn get(&self) -> GlobalRefMut<'a, T> {
        self.world.globals.get_mut::<T>().unwrap()
    }
}

pub struct World {
    components: BTreeMap<component::ID, ComponentMap>,
    globals: GlobalMap,
    next_id: entity::ID,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: BTreeMap::new(),
            globals: GlobalMap::new(),
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

    pub fn components<'a, T: component::Component>(&'a self) -> ReadComponent<'a, T> {
        ReadComponent {
            world: self,
            phantom: PhantomData,
        }
    }

    pub fn components_mut<'a, T: component::Component>(&'a self) -> ReadWriteComponent<'a, T> {
        ReadWriteComponent {
            world: self,
            phantom: PhantomData,
        }
    }

    pub fn global<'a, T: component::Global>(&'a self) -> ReadGlobal<'a, T> {
        ReadGlobal {
            world: self,
            phantom: PhantomData,
        }
    }

    pub fn global_mut<'a, T: component::Global>(&'a self) -> ReadWriteGlobal<'a, T> {
        ReadWriteGlobal {
            world: self,
            phantom: PhantomData,
        }
    }

    pub fn set_global<T: component::Global>(&self, r: T) {
        self.globals.set(r).unwrap();
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
