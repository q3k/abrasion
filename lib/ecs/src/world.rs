use std::collections::BTreeMap;
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::iter::Iterator;

use crate::entity;
use crate::component;

type EntityComponent = (entity::ID, Box<dyn component::Component>);


pub struct ReadData<'a, T: component::Component> {
    underlying: &'a RefCell<Vec<EntityComponent>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: component::Component> ReadData<'a, T> {
    pub fn iter(&self) -> ReadDataIter<'a, T> {
        ReadDataIter {
            underlying: Some(Ref::map(self.underlying.borrow(), |el| el.as_slice())),
            phantom: PhantomData,
        }
    }
}

pub struct ReadDataIter<'a, T: component::Component> {
    underlying: Option<Ref<'a, [EntityComponent]>>,
    phantom: PhantomData<&'a T>,
}

impl <'a, T: component::Component> Iterator for ReadDataIter<'a, T> {
    type Item = (entity::ID, Ref<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.underlying.as_ref().unwrap().len() == 0 {
            return None;
        }
        let mut id: u64 = 0;
        let (head, tail) = Ref::map_split(self.underlying.take().unwrap(), |slice| {
            let (head, tail) = slice.split_first().unwrap();
            id = head.0;
            let ptr = head.1.as_ref();

            let el = unsafe { & *(ptr as *const (dyn component::Component) as *const T) };
            return (el, tail);
        });
        self.underlying = Some(tail);
        Some((id, head))
    }
}

pub struct ReadWriteData<'a, T: component::Component> {
    underlying: &'a RefCell<Vec<EntityComponent>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: component::Component> ReadWriteData<'a, T> {
    pub fn iter_mut(&mut self) -> ReadWriteDataIter<'a, T> {
        ReadWriteDataIter {
            underlying: Some(RefMut::map(self.underlying.borrow_mut(), |el| el.as_mut_slice())),
            phantom: PhantomData,
        }
    }
}

pub struct ReadWriteDataIter<'a, T: component::Component> {
    underlying: Option<RefMut<'a, [EntityComponent]>>,
    phantom: PhantomData<&'a T>,
}

impl <'a, T: component::Component> Iterator for ReadWriteDataIter<'a, T> {
    type Item = (entity::ID, RefMut<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.underlying.as_ref().unwrap().len() == 0 {
            return None;
        }

        let mut id: u64 = 0;
        let (head, tail) = RefMut::map_split(self.underlying.take().unwrap(), |slice| {
            let (head, tail) = slice.split_first_mut().unwrap();
            id = head.0;
            let ptr: &mut dyn component::Component = &mut (*head.1);

            let el = unsafe {
                &mut *(ptr as *mut (dyn component::Component) as *mut T)
            };
            return (el, tail);
        });
        self.underlying = Some(tail);
        Some((id, head))
    }
}

pub struct World {
    entities: BTreeMap<entity::ID, entity::Entity>,
    components_by_id: BTreeMap<component::ID, RefCell<Vec<EntityComponent>>>,
    next_id: entity::ID,

    empty: RefCell<Vec<EntityComponent>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: BTreeMap::new(),
            components_by_id: BTreeMap::new(),
            next_id: 1u64,
            empty: RefCell::new(Vec::new()),
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
        let vec = self.components_by_id.entry(cid).or_insert(RefCell::new(vec!()));
        vec.borrow_mut().push((e.id(), c));
    }

    pub fn commit(&mut self, ent: entity::Entity) {
        self.entities.insert(ent.id(), ent);
    }

    pub fn components<'a, T: component::Component>(&'a self) -> ReadData<T> {
        let underlying = match self.components_by_id.get(&component::id::<T>()) {
            None => &self.empty,
            Some(r) => r,
        };
        ReadData {
            underlying: underlying,
            phantom: PhantomData,
        }
    }

    pub fn components_mut<'a, T: component::Component>(&'a self) -> ReadWriteData<T> {
        let underlying = match self.components_by_id.get(&component::id::<T>()) {
            None => &self.empty,
            Some(r) => &r,
        };
        ReadWriteData {
            underlying: underlying,
            phantom: PhantomData,
        }
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
        //assert_eq!(named.len(), 2);
        assert_eq!(String::from("foo"), (named.next().unwrap().1).0);
        assert_eq!(String::from("foo"), (named2.next().unwrap().1).0);
        assert_eq!(String::from("bar"), (named.next().unwrap().1).0);
        assert_eq!(String::from("bar"), (named2.next().unwrap().1).0);
    }
}
