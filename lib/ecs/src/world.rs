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
use crate::index;

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

pub struct ReadIndex<'a, I: index::Index> {
    world: &'a World,
    phantom: PhantomData<&'a I>
}

impl <'a, I: index::Index> ReadIndex<'a, I> {
    pub fn get(&self) -> &'a I {
        let iid = index::index_id::<I>();
        let cid = component::component_id::<I::Component>();
        let b = self.world.indices.get(&cid).unwrap().get(&iid).unwrap();
        unsafe {
            index::retype(b)
        }
    }
}

/// ReadWriteAll gives access to all components/entities/globals within a world. Using it in a
/// system means that no other system can run in parallel, and limits performance. This should only
/// be used when absolutely necessary (eg. for scripting systems).
pub struct ReadWriteAll<'a> {
    world: &'a World,
}

impl<'a> ReadWriteAll<'a> {
    pub fn world(self) -> &'a World {
        self.world
    }
}

impl<'a> ReadWriteAll<'a> {}

impl<'a> std::ops::Deref for ReadWriteAll<'a> {
    type Target = World;
    fn deref(&self) -> &Self::Target {
        self.world
    }
}


pub struct World {
    components: BTreeMap<component::ID, ComponentMap>,
    component_by_idstr: BTreeMap<&'static str, component::ID>,
    component_lua_bindings: BTreeMap<component::ID, Box<dyn component::LuaBindings>>,
    component_queue: RefCell<Vec<(component::ID, Box<dyn component::Component>, entity::Entity)>>,
    globals: GlobalMap,
    next_id: RefCell<entity::ID>,
    indices: BTreeMap<component::ID, BTreeMap<index::ID, Box<dyn index::IndexDyn>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: BTreeMap::new(),
            component_by_idstr: BTreeMap::new(),
            component_lua_bindings: BTreeMap::new(),
            component_queue: RefCell::new(Vec::new()),
            globals: GlobalMap::new(),
            next_id: RefCell::new(1u64),
            indices: BTreeMap::new(),
        }
    }

    fn allocate_next_id(&self) -> entity::ID {
        let res = self.next_id.borrow().clone();
        let mut nid = self.next_id.borrow_mut();
        *nid = *nid + 1;
        res
    }

    pub fn new_entity(&mut self) -> entity::EntityBuilder {
        entity::EntityBuilder::new(self, self.allocate_next_id())
    }

    pub fn new_entity_lazy(&self) -> entity::LazyEntityBuilder {
        entity::LazyEntityBuilder::new(self.allocate_next_id())
    }

    pub fn register_component_lua_bindings(
        &mut self,
        bindings: Box<dyn component::LuaBindings>,
    ) {
        let idstr = bindings.idstr();
        let cid = bindings.id();
        if let Some(_) = self.component_by_idstr.get(idstr) {
            log::warn!("Ignored attempted re-registration of Lua bindings for component {} (duplicate idstr)", idstr);
            return
        }
        if let Some(_) = self.component_lua_bindings.get(&cid) {
            log::warn!("Ignored attempted re-registration of Lua bindings for component {} (duplicate ID)", idstr);
            return
        }
        self.component_by_idstr.insert(idstr, cid);
        self.component_lua_bindings.insert(cid, bindings);
    }

    pub fn lua_any_into_dyn<'a, 'b>(&'a self, ud: &'a mlua::AnyUserData) -> Option<(String, Box<dyn component::Component>)> {
        for (_, bindings) in self.component_lua_bindings.iter() {
            if let Some(b) = bindings.any_into_dyn(ud) {
                return Some((bindings.idstr().into(), b));
            }
        }
        None
    }

    pub fn register_component_entity(
        &mut self,
        cid: component::ID,
        c: Box<dyn component::Component>,
        e: entity::Entity
    ) {
        self.index_on_added(e.id(), &c);
        let map = self.components.entry(cid).or_insert(ComponentMap::new());
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

    pub fn get_component_lua_bindings(
        &self,
    ) -> Vec<(String, component::ID, &Box<dyn component::LuaBindings>)> {
        self.component_by_idstr.iter().filter_map(|(idstr, cid)| {
            let bindings = self.component_lua_bindings.get(cid)?;
            Some((idstr.to_string(), *cid, bindings))
        }).collect()
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

    pub fn component_get_dyn_cloned<'a>(
        &'a self,
        e: entity::ID,
        cid: component::ID,
    ) -> Option<Box<dyn component::Component>> {
        let map = self.components.get(&cid)?;
        match map.get_dyn(e) {
            Ok(val) => Some(val.clone_dyn()),
            Err(err) => {
                // Attempt to get from queue.
                let cq = self.component_queue.borrow();
                for (qcid, qc, qe) in cq.iter() {
                    if qe.id() == e && *qcid == cid {
                        return Some(qc.clone_dyn());
                    }
                }
                // TODO(q3k): better error handling
                panic!("get_dyn({:?}): not in queue and ecs said: {:?}", e, err);
            }
        }
    }

    pub fn component_set_dyn<'a>(
        &'a self,
        e: entity::ID,
        c: Box<dyn component::Component>
    ) {
        // TODO(q3k): error handling
        // TODO(q3k): do not insert component data if it's missing? That can happen if we attempt
        // to set_dyn before the entity/components are registered.
        match self.components.get(&c.id()) {
            None => panic!("Component {:?} not found", c),
            Some(map) => map.insert(e, c).unwrap(),
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

    pub fn add_index<I: index::Index>(&mut self, ix: I) {
        let inner = self.indices.entry(component::component_id::<I::Component>()).or_insert(BTreeMap::new());
        let ixid = index::index_id::<I>();

        use std::collections::btree_map::Entry;
        match inner.entry(ixid) {
            Entry::Occupied(_) => panic!("index already registered {:?}", ixid),
            Entry::Vacant(v) => { v.insert(ix.erase()); },
        }
    }

    pub fn index<'a, I: index::Index>(&'a self) -> ReadIndex<'a, I> {
        ReadIndex {
            world: self,
            phantom: PhantomData,
        }
    }

    fn index_on_added(&mut self, eid: entity::ID, c: &Box<dyn component::Component>) {
        let cid = c.id();
        if let Some(indices) = self.indices.get_mut(&cid) {
            for (_, ix) in indices.iter_mut() {
                ix.added(eid, c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::component;
    use crate::world;

    #[derive(Clone,Debug)]
    struct Position {
        x: u32,
        y: u32,
        z: u32,
    }
    impl component::Component for Position {
        fn id(&self) -> component::ID {
            component::component_id::<Position>()
        }
        fn clone_dyn(&self) -> Box<dyn component::Component> {
            Box::new(self.clone())
        }
    }

    #[derive(Clone,Debug)]
    struct Name(String);
    impl component::Component for Name {
        fn id(&self) -> component::ID {
            component::component_id::<Name>()
        }
        fn clone_dyn(&self) -> Box<dyn component::Component> {
            Box::new(self.clone())
        }
    }
    impl Name {
        fn new(s: &str) -> Name {
            Name(String::from(s))
        }
    }

    #[test]
    fn world() {
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
