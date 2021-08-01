use crate::{
    component,
    entity,
};

pub type ID = std::any::TypeId;

pub fn index_id<I: Index>() -> ID {
    std::any::TypeId::of::<I>()
}

pub trait IndexComponent: 'static {
    type Data<'a>;
    type Typed: Sized;

    unsafe fn retype<'a>(&'a self) -> &'a Self::Typed;
    fn encode<'a>(c: &'a Box<dyn component::Component>) -> Self::Data<'a>;
}

impl<C: component::Component> IndexComponent for C {
    type Data<'a> = Option<&'a C>;
    type Typed = C;

    unsafe fn retype<'a>(&'a self) -> &'a C {
        let val = self as *const Self;
        &(*val)
    }

    fn encode<'a>(c: &'a Box<dyn component::Component>) -> Option<&'a C> {
        let v = c.as_ref() as *const (dyn component::Component) as *const C;
        unsafe {
            Some(&*v)
        }
    }
}

pub trait Index: component::Global + 'static {
    type Component: IndexComponent;

    fn id(&self) -> ID where Self: Sized {
        index_id::<Self>()
    }

    fn erase(self) -> Box<dyn IndexDyn> where Self: Sized {
        Box::new(IndexDynWrapper {
            ix: Box::new(self),
        })
    }

    fn added<'a>(&'a mut self, _eid: entity::ID, _c: <Self::Component as IndexComponent>::Data<'a>) {}
}

impl<I: Index> component::Global for I {}

pub trait IndexDyn: 'static {
    fn added(&mut self, eid: entity::ID, c: &Box<dyn component::Component>);
}

pub unsafe fn retype<'a, I: Index>(b: &'a Box<dyn IndexDyn>) -> &'a I {
    let v = b.as_ref() as *const dyn IndexDyn as *const IndexDynWrapper<I::Component>;
    let v: &'a IndexDynWrapper<I::Component> = &*v;
    let ix = v.ix.as_ref() as *const dyn Index<Component=I::Component> as *const I;
    let res: &'a I =  &*ix;
    res
}

struct IndexDynWrapper<C: IndexComponent> {
    ix: Box<dyn Index<Component=C>>,
}

impl<C: IndexComponent> IndexDyn for IndexDynWrapper<C> {
    fn added(&mut self, eid: entity::ID, c: &Box<dyn component::Component>) {
        self.ix.added(eid, C::encode(c))
    }
}

#[cfg(test)]
mod tests {
    use crate:: {
        component,
        entity,
        index,
        world,
    };

    #[derive(Debug,Clone)]
    struct Position {
        x: i32,
        y: i32,
        z: i32,
    }
    impl component::Component for Position {
        fn id(&self) -> component::ID {
            component::component_id::<Position>()
        }
        fn clone_dyn(&self) -> Box<dyn component::Component> {
            Box::new(self.clone())
        }
    }

    #[derive(Debug,Clone)]
    struct Sorted {
        by_x: Vec<(i32, entity::ID)>,
    }
    impl index::Index for Sorted {
        type Component = Position;

        fn added(&mut self, eid: entity::ID, c: Option<&Position>) {
            self.by_x.push((c.as_ref().unwrap().x, eid));
            self.by_x.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }
    impl Sorted {
        fn new() -> Self {
            Self {
                by_x: Vec::new(),
            }
        }
    }

    #[test]
    fn index() {
        let mut world = world::World::new();
        world.add_index(Sorted::new());

        let eid1 = world.new_entity().with(Position { x: 0, y: 0, z: 0 }).build();
        let eid2 = world.new_entity().with(Position { x: 10, y: 0, z: 1 }).build();
        let eid3 = world.new_entity().with(Position { x: -10, y: 0, z: 2 }).build();

        let sorted = world.index::<Sorted>().get();
        let mut iter = sorted.by_x.iter();
        assert_eq!(eid3, iter.next().unwrap().1);
        assert_eq!(eid1, iter.next().unwrap().1);
        assert_eq!(eid2, iter.next().unwrap().1);
    }
}
