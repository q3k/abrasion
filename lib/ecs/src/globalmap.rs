use std::cell::{Cell, UnsafeCell};
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use crate::component;
use crate::borrow;

struct GlobalMapEntry {
    global: Box<dyn component::Global>,
    borrow: Cell<borrow::Flag>,
}

pub struct GlobalMap {
    value: UnsafeCell<BTreeMap<component::ID, GlobalMapEntry>>,
    borrow: Cell<borrow::Flag>,
}

#[derive(Clone,Debug)]
pub struct AccessError(String);

impl AccessError {
    fn concurrent() -> Self {
        Self("concurrent access".to_string())
    }
}

impl GlobalMap {
    pub fn new() -> Self {
        Self {
            value: UnsafeCell::new(BTreeMap::new()),
            borrow: Cell::new(borrow::UNUSED),
        }
    }

    pub fn get<'a, T: component::Global>(&'a self) -> Result<GlobalRef<'a, T>, AccessError> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError::concurrent()),
            Some(b) => {
                let map = self.value.get();
                unsafe {
                    match (*map).get(&component::global_id::<T>()) {
                        Some(entry) => {
                            let val = &entry.global;
                            match borrow::Ref::new(&entry.borrow) {
                                None => Err(AccessError::concurrent()),
                                Some(b2) => {
                                    let val = val.as_ref();
                                    let val = & *(val as *const (dyn component::Global) as *const T);
                                    drop(b);
                                    Ok(GlobalRef { val, borrow: Some(b2) })
                                },
                            }
                        },
                        None => Err(AccessError("global absent from world".to_string())),
                    }
                }
            }
        }
    }

    pub fn get_mut<'a, T: component::Global>(&'a self) -> Result<GlobalRefMut<'a, T>, AccessError> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError::concurrent()),
            Some(b) => {
                let map = self.value.get();
                unsafe {
                    match (*map).get_mut(&component::global_id::<T>()) {
                        Some(entry) => {
                            let val = &mut entry.global;
                            match borrow::RefMut::new(&entry.borrow) {
                                None => Err(AccessError::concurrent()),
                                Some(b2) => {
                                    let val = val.as_mut();
                                    let val = &mut *(val as *mut (dyn component::Global) as *mut T);
                                    drop(b);
                                    Ok(GlobalRefMut { val, borrow: Some(b2) })
                                },
                            }
                        },
                        None => Err(AccessError("global absent from world".to_string())),
                    }
                }
            }
        }
    }

    pub fn set<'a, T: component::Global>(&'a self, r: T) -> Result<(), AccessError> {
        match borrow::RefMut::new(&self.borrow) {
            None => Err(AccessError::concurrent()),
            Some(b) => {
                let map = self.value.get();
                let rid = component::global_id::<T>();
                unsafe {
                    match (*map).get_mut(&rid) {
                        Some(entry) => {
                            match borrow::RefMut::new(&entry.borrow) {
                                None => { return Err(AccessError::concurrent()); },
                                Some(b2) => {
                                    entry.global = Box::new(r);
                                    drop(b2);
                                },
                            };
                        },
                        None => {
                            (*map).insert(rid, GlobalMapEntry {
                                global: Box::new(r),
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

pub struct GlobalRef<'a, T: component::Global> {
    val: *const T,
    borrow: Option<borrow::Ref<'a>>,
}

impl<'a, T: component::Global> Deref for GlobalRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.val)
        }
    }
}

impl <'a, T: component::Global> Drop for GlobalRef<'a, T> {
    fn drop(&mut self) {
        self.borrow = None;
    }
}

pub struct GlobalRefMut<'a, T: component::Global> {
    val: *mut T,
    borrow: Option<borrow::RefMut<'a>>,
}

impl <'a, T: component::Global> Drop for GlobalRefMut<'a, T> {
    fn drop(&mut self) {
        self.borrow = None;
    }
}

impl <'a, T: component::Global> Deref for GlobalRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.val)
        }
    }
}

impl <'a, T: component::Global> DerefMut for GlobalRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut(*self.val)
        }
    }
}
