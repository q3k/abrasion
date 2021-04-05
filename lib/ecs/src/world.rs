use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::iter::Iterator;
use std::cell::RefCell;

use crate::componentmap::{
    AccessError,
    Ref as ComponentMapRef,
    RefMut as ComponentMapRefMut,
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

    pub fn get(&self, e: entity::ID) -> Result<ComponentMapRef<'a, T>, AccessError> {
        // TODO(q3k): fix the unwrap
        let cm = self.world.components.get(&component::component_id::<T>()).unwrap();
        unsafe {
            cm.get(e)
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

    pub fn get_mut(&self, e: entity::ID) -> Result<ComponentMapRefMut<'a, T>, AccessError> {
        // TODO(q3k): fix the unwrap
        let cm = self.world.components.get(&component::component_id::<T>()).unwrap();
        unsafe {
            cm.get_mut(e)
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

/// ReadWriteAll gives access to all components/entities/globals within a world. Using it in a
/// system means that no other system can run in parallel, and limits performance. This should only
/// be used when absolutely necessary (eg. for scripting systems).
pub struct ReadWriteAll<'a> {
    world: &'a World,
}

impl<'a> ReadWriteAll<'a> {
}


pub struct World {
    components: BTreeMap<component::ID, ComponentMap>,
    component_by_idstr: BTreeMap<&'static str, component::ID>,
    component_lua_bindings: BTreeMap<component::ID, Box<dyn component::LuaBindings>>,
    component_queue: RefCell<Vec<(component::ID, Box<dyn component::Component>, entity::Entity)>>,
    globals: GlobalMap,
    next_id: entity::ID,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: BTreeMap::new(),
            component_by_idstr: BTreeMap::new(),
            component_lua_bindings: BTreeMap::new(),
            component_queue: RefCell::new(Vec::new()),
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
        if let Some(bindings) = c.lua_bindings() {
            // TODO(q3k): optimize this to not happen on every registration.
            self.component_by_idstr.insert(bindings.id(), cid);
            self.component_lua_bindings.insert(cid, bindings);
        }
        let map = self.components.entry(cid).or_insert_with(|| {
            if let Some(bindings) = c.lua_bindings() {
                log::info!("Registered component {}", bindings.id());
            } else {
                log::warn!("Component {:?} has no .lua_bindings() defined, will not be accessible from scripting.", cid);
            }
            ComponentMap::new()
        });
        map.insert(e.id(), c).unwrap();
    }

    pub fn enqueue_register_component_entity(
        &self,
        cid: component::ID,
        c: Box<dyn component::Component>,
        e: entity::Entity
    ) {
        self.component_queue.borrow_mut().push((cid, c, e));
    }

    pub fn queue_drain(
        &mut self,
    ) {
        for (cid, c, e) in self.component_queue.replace(Vec::new()).into_iter() {
            self.register_component_entity(cid, c, e);
        }
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

    pub fn all<'a>(&'a self) -> ReadWriteAll<'a> {
        ReadWriteAll {
            world: self,
        }
    }

    pub fn set_global<T: component::Global>(&self, r: T) {
        self.globals.set(r).unwrap();
    }

    pub fn lua_components(&self) -> &BTreeMap<component::ID, Box<dyn component::LuaBindings>> {
        &self.component_lua_bindings
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
