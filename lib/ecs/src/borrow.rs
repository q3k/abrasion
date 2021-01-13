use std::cell::Cell;

pub type Flag = isize;
pub const UNUSED: Flag = 0;

pub struct Ref<'b> {
    borrow: &'b Cell<Flag>,
}

impl<'b> Ref<'b> {
    pub fn new(borrow: &'b Cell<Flag>) -> Option<Ref<'b>> {
        let b = borrow.get().wrapping_add(1);
        if b <= 0 {
            None
        } else {
            borrow.set(b);
            Some(Ref { borrow })
        }
    }
}

impl<'a> Drop for Ref<'a> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        self.borrow.set(borrow - 1);
    }
}

pub struct RefMut<'b> {
    borrow: &'b Cell<Flag>,
}

impl<'b> RefMut<'b> {
    pub fn new(borrow: &'b Cell<Flag>) -> Option<RefMut<'b>> {
        match borrow.get() {
            UNUSED => {
                borrow.set(UNUSED - 1);
                Some(RefMut { borrow })
            },
            _ => None,
        }
    }
}

impl<'a> Drop for RefMut<'a> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        self.borrow.set(borrow + 1);
    }
}

