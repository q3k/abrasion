use std::cell::{Cell, UnsafeCell};
use std::collections::BTreeMap;
use std::collections::btree_map::{
    Iter as BTMIter,
    IterMut as BTMIterMut,
};

use crate::entity;
use crate::component;


type BorrowFlag = isize;
const UNUSED: BorrowFlag = 0;

struct BorrowRef<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl<'b> BorrowRef<'b> {
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRef<'b>> {
        let b = borrow.get().wrapping_add(1);
        if b <= 0 {
            None
        } else {
            borrow.set(b);
            Some(BorrowRef { borrow })
        }
    }
}

impl<'a> Drop for BorrowRef<'a> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        self.borrow.set(borrow - 1);
    }
}

struct BorrowRefMut<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl<'b> BorrowRefMut<'b> {
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRefMut<'b>> {
        match borrow.get() {
            UNUSED => {
                borrow.set(UNUSED - 1);
                Some(BorrowRefMut { borrow })
            },
            _ => None,
        }
    }
}

impl<'a> Drop for BorrowRefMut<'a> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        self.borrow.set(borrow + 1);
    }
}

pub struct ComponentMap {
    value: UnsafeCell<BTreeMap<entity::ID, Box<dyn component::Component>>>,
    borrow: Cell<BorrowFlag>,
}

#[derive(Clone,Debug)]
pub struct AccessError(String);

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            value: UnsafeCell::new(BTreeMap::new()),
            borrow: Cell::new(UNUSED),
        }
    }

    pub fn try_iter<'a>(&'a self) -> Result<ComponentMapIter<'a>, AccessError> {
        match BorrowRef::new(&self.borrow) {
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
        match BorrowRefMut::new(&self.borrow) {
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
        match BorrowRefMut::new(&self.borrow) {
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
    borrow: BorrowRef<'a>,
}

pub struct ComponentMapIterMut<'a> {
    pub iter: BTMIterMut<'a, entity::ID, Box<dyn component::Component>>,
    #[allow(dead_code)]
    borrow: BorrowRefMut<'a>,
}
