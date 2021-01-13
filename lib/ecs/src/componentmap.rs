use std::cell::{Cell, UnsafeCell};
use std::collections::BTreeMap;
use std::collections::btree_map::{
    Iter as BTMIter,
    IterMut as BTMIterMut,
};

use crate::entity;
use crate::component;
use crate::borrow;


pub struct ComponentMap {
    value: UnsafeCell<BTreeMap<entity::ID, Box<dyn component::Component>>>,
    borrow: Cell<borrow::Flag>,
}

#[derive(Clone,Debug)]
pub struct AccessError(String);

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            value: UnsafeCell::new(BTreeMap::new()),
            borrow: Cell::new(borrow::UNUSED),
        }
    }

    pub fn try_iter<'a>(&'a self) -> Result<ComponentMapIter<'a>, AccessError> {
        match borrow::Ref::new(&self.borrow) {
            None => Err(AccessError("already borrowed mutably".to_string())),
            Some(b) => Ok(ComponentMapIter {
                iter: unsafe {
                    let map = &*self.value.get();
                    map.iter()
                },
                borrow: b,
            }),
        }
    }
    pub fn try_iter_mut<'a>(&'a self) -> Result<ComponentMapIterMut<'a>, AccessError> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError("already borrowed mutable".to_string())),
            Some(b) => Ok(ComponentMapIterMut {
                iter: unsafe {
                    let map = &mut *self.value.get();
                    map.iter_mut()
                },
                borrow: b,
            }),
        }
    }

    pub fn insert(&self, e: entity::ID, c: Box<dyn component::Component>) -> Result<(), AccessError> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError("already borrow mutably".to_string())),
            Some(b) => {
                unsafe {
                    let map = &mut *self.value.get();
                    map.insert(e, c);
                }
                drop(b);
                Ok(())
            }
        }
    }
}

pub struct ComponentMapIter<'a> {
    pub iter: BTMIter<'a, entity::ID, Box<dyn component::Component>>,
    #[allow(dead_code)]
    borrow: borrow::Ref<'a>,
}

pub struct ComponentMapIterMut<'a> {
    pub iter: BTMIterMut<'a, entity::ID, Box<dyn component::Component>>,
    #[allow(dead_code)]
    borrow: borrow::RefMut<'a>,
}
