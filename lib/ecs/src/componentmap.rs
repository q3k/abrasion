use std::cell::{Cell, UnsafeCell};
use std::collections::BTreeMap;
use std::collections::btree_map::{
    Iter as BTMIter,
    IterMut as BTMIterMut,
};
use std::ops::{Deref, DerefMut};

use crate::entity;
use crate::component;
use crate::borrow;


pub struct ComponentMap {
    value: UnsafeCell<BTreeMap<entity::ID, Box<dyn component::Component>>>,
    borrow: Cell<borrow::Flag>,
}

#[derive(Clone,Debug)]
pub enum AccessError {
    AlreadyBorrowedMutably,
    NoSuchEntity,
    NoSuchComponent,
}

pub type Result<T> = std::result::Result<T, AccessError>;

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            value: UnsafeCell::new(BTreeMap::new()),
            borrow: Cell::new(borrow::UNUSED),
        }
    }

    pub fn try_iter<'a>(&'a self) -> Result<ComponentMapIter<'a>> {
        match borrow::Ref::new(&self.borrow) {
            None => Err(AccessError::AlreadyBorrowedMutably),
            Some(b) => Ok(ComponentMapIter {
                iter: unsafe {
                    let map = &*self.value.get();
                    map.iter()
                },
                borrow: Some(b),
            }),
        }
    }
    pub fn try_iter_mut<'a>(&'a self) -> Result<ComponentMapIterMut<'a>> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError::AlreadyBorrowedMutably),
            Some(b) => Ok(ComponentMapIterMut {
                iter: unsafe {
                    let map = &mut *self.value.get();
                    map.iter_mut()
                },
                borrow: Some(b),
            }),
        }
    }

    pub fn insert(&self, e: entity::ID, c: Box<dyn component::Component>) -> Result<()> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError::AlreadyBorrowedMutably),
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

    pub fn get_dyn<'a>(&'a self, e: entity::ID) -> Result<RefDyn<'a>> {
        unsafe {
            match borrow::Ref::new(&self.borrow) {
                None => Err(AccessError::AlreadyBorrowedMutably),
                Some(b) => {
                    let map = &*self.value.get();
                    match map.get(&e) {
                        None => Err(AccessError::NoSuchEntity),
                        Some(component) => {
                            let component = component.as_ref();
                            let val = component as *const (dyn component::Component);
                            Ok(RefDyn {
                                val,
                                borrow: Some(b),
                            })
                        },
                    }
                }
            }
        }
    }

    pub unsafe fn get<'a, T: component::Component>(&'a self, e: entity::ID) -> Result<Ref<'a, T>> {
        match borrow::Ref::new(&self.borrow) {
            None => Err(AccessError::AlreadyBorrowedMutably),
            Some(b) => {
                let map = &*self.value.get();
                match map.get(&e) {
                    None => Err(AccessError::NoSuchEntity),
                    Some(component) => {
                        let component = component.as_ref();
                        let val = component as *const (dyn component::Component) as *const T;
                        Ok(Ref {
                            val,
                            borrow: Some(b),
                        })
                    },
                }
            }
        }
    }

    pub unsafe fn get_mut<'a, T: component::Component>(&'a self, e: entity::ID) -> Result<RefMut<'a, T>> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError::AlreadyBorrowedMutably),
            Some(b) => {
                let map = &mut*self.value.get();
                match map.get_mut(&e) {
                    None => Err(AccessError::NoSuchEntity),
                    Some(component) => {
                        let component = component.as_mut();
                        let val = component as *mut (dyn component::Component) as *mut T;
                        Ok(RefMut {
                            val,
                            borrow: Some(b),
                        })
                    },
                }
            }
        }
    }
}

pub struct RefDyn<'a> {
    val: *const (dyn component::Component),
    borrow: Option<borrow::Ref<'a>>,
}

impl <'a> Drop for RefDyn<'a> {
    fn drop(&mut self) {
        self.borrow = None;
    }
}

impl <'a> Deref for RefDyn<'a> {
    type Target = dyn component::Component;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.val)
        }
    }
}

pub struct Ref<'a, T: component::Component> {
    val: *const T,
    borrow: Option<borrow::Ref<'a>>,
}

impl <'a, T: component::Component> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        self.borrow = None;
    }
}

impl <'a, T: component::Component> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.val)
        }
    }
}

pub struct RefMut<'a, T: component::Component> {
    val: *mut T,
    borrow: Option<borrow::RefMut<'a>>,
}

impl <'a, T: component::Component> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        self.borrow = None;
    }
}

impl <'a, T: component::Component> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.val)
        }
    }
}

impl <'a, T: component::Component> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut(*self.val)
        }
    }
}

pub struct ComponentMapIter<'a> {
    pub iter: BTMIter<'a, entity::ID, Box<dyn component::Component>>,
    borrow: Option<borrow::Ref<'a>>,
}

impl<'a> Drop for ComponentMapIter<'a> {
    fn drop(&mut self) {
        self.borrow = None;
    }
}

pub struct ComponentMapIterMut<'a> {
    pub iter: BTMIterMut<'a, entity::ID, Box<dyn component::Component>>,
    borrow: Option<borrow::RefMut<'a>>,
}

impl<'a> Drop for ComponentMapIterMut<'a> {
    fn drop(&mut self) {
        self.borrow = None;
    }
}
