use std::cell::{Cell, UnsafeCell};
use std::collections::BTreeMap;
use std::ops::Deref;

use crate::component;
use crate::borrow;

struct ResourceMapEntry {
    resource: Box<dyn component::Resource>,
    borrow: Cell<borrow::Flag>,
}

pub struct ResourceMap {
    value: UnsafeCell<BTreeMap<component::ID, ResourceMapEntry>>,
    borrow: Cell<borrow::Flag>,
}

#[derive(Clone,Debug)]
pub struct AccessError(String);

impl AccessError {
    fn concurrent() -> Self {
        Self("concurrent access".to_string())
    }
}

impl ResourceMap {
    pub fn new() -> Self {
        Self {
            value: UnsafeCell::new(BTreeMap::new()),
            borrow: Cell::new(borrow::UNUSED),
        }
    }

    pub fn get<'a, T: component::Resource>(&'a self) -> Result<ResourceRef<'a, T>, AccessError> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError::concurrent()),
            Some(b) => {
                let map = self.value.get();
                unsafe {
                    match (*map).get(&component::resource_id::<T>()) {
                        Some(entry) => {
                            let val = &entry.resource;
                            match borrow::Ref::new(&entry.borrow) {
                                None => Err(AccessError::concurrent()),
                                Some(b2) => {
                                    let val = val.as_ref();
                                    let val = unsafe { & *(val as *const (dyn component::Resource) as *const T) };
                                    drop(b);
                                    Ok(ResourceRef { val, borrow: b2 })
                                },
                            }
                        },
                        None => Err(AccessError("resource absent from world".to_string())),
                    }
                }
            }
        }
    }

    pub fn set<'a, T: component::Resource>(&'a self, r: T) -> Result<(), AccessError> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError::concurrent()),
            Some(b) => {
                let map = self.value.get();
                let rid = component::resource_id::<T>();
                unsafe {
                    match (*map).get_mut(&rid) {
                        Some(entry) => {
                            match borrow::RefMut::new(&entry.borrow) {
                                None => { return Err(AccessError::concurrent()); },
                                Some(b2) => {
                                    entry.resource = Box::new(r);
                                    drop(b2);
                                },
                            };
                        },
                        None => {
                            (*map).insert(rid, ResourceMapEntry {
                                resource: Box::new(r),
                                borrow: Cell::new(borrow::UNUSED),
                            });
                        },
                    };
                    drop(b);
                    Ok(())
                }
            }
        }
    }
}

pub struct ResourceRef<'a, T: component::Resource> {
    val: *const T,
    borrow: borrow::Ref<'a>,
}

impl<'a, T: component::Resource> Deref for ResourceRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.val)
        }
    }
}
