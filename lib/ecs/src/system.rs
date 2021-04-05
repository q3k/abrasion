use std::marker::PhantomData;
use std::iter::Peekable;

use crate::{
    component,
    world::{
        ReadComponent, ReadComponentIter,
        ReadWriteComponent, ReadWriteComponentIter,
        ReadGlobal, ReadWriteGlobal,
        ReadWriteAll,
        World,
    }
};

pub trait System<'a> {
    type SystemData: Access<'a>;

    fn run(&mut self, sd: Self::SystemData);
}

trait WorldRunner {
    fn run_world(&mut self, w: &World);
}

impl<T> WorldRunner for T
where
    T: for<'a> System<'a>
{
    fn run_world(&mut self, world: &World) {
        let data = T::SystemData::fetch(world);
        self.run(data);
    }
}

pub trait Access<'a> {
    fn fetch(world: &'a World) -> Self;
}

pub trait AccessComponent<'a> : Access<'a> {
    type Component;
    type Iterator: Iterator<Item = (u64, Self::Component)>;
    fn iter(&self) -> Self::Iterator;
}

impl<'a, T: component::Component> Access<'a> for ReadComponent<'a, T> {
    fn fetch(world: &'a  World) -> Self {
        world.components()
    }
}

impl<'a, T: component::Component> AccessComponent<'a> for ReadComponent<'a, T> {
    type Component = &'a T;
    type Iterator = ReadComponentIter<'a, T>;
    fn iter(&self) -> ReadComponentIter<'a, T> {
        Self::iter(self)
    }
}

impl<'a, T: component::Component> Access<'a> for ReadWriteComponent<'a, T> {
    fn fetch(world: &'a  World) -> Self {
        world.components_mut()
    }
}

impl<'a, T: component::Component> AccessComponent<'a> for ReadWriteComponent<'a, T> {
    type Component = &'a mut T;
    type Iterator = ReadWriteComponentIter<'a, T>;
    fn iter(&self) -> ReadWriteComponentIter<'a, T> {
        Self::iter_mut(self)
    }
}

impl<'a, T: component::Global> Access<'a> for ReadGlobal<'a, T> {
    fn fetch(world: &'a World) -> Self {
        world.global()
    }
}

impl<'a, T: component::Global> Access<'a> for ReadWriteGlobal<'a, T> {
    fn fetch(world: &'a World) -> Self {
        world.global_mut()
    }
}

impl<'a> Access<'a> for ReadWriteAll<'a> {
    fn fetch(world: &'a World) -> Self {
        world.all()
    }
}

macro_rules! impl_access_tuple {
    ( $($ty:ident),* ) => {
        impl <'a, $($ty),*> Access<'a> for ( $($ty,)* )
            where $( $ty : Access<'a> ),*
        {
            fn fetch(world: &'a  World) -> Self {
                ( $( $ty::fetch(world), )* )
            }
        }
    }
}

impl_access_tuple!(A);
impl_access_tuple!(A, B);
impl_access_tuple!(A, B, C);
impl_access_tuple!(A, B, C, D);
impl_access_tuple!(A, B, C, D, E);
impl_access_tuple!(A, B, C, D, E, F);
impl_access_tuple!(A, B, C, D, E, F, G);
impl_access_tuple!(A, B, C, D, E, F, G, H);
impl_access_tuple!(A, B, C, D, E, F, G, H, I);
impl_access_tuple!(A, B, C, D, E, F, G, H, I, J);

pub struct Processor {
    runners: Vec<Box<dyn WorldRunner>>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            runners: Vec::new(),
        }
    }
    pub fn add_system<T>(&mut self, system: T)
    where
        T: for<'c> System<'c> + 'static
    {
        self.runners.push(Box::new(system));
    }

    pub fn run(&mut self, world: &World) {
        for runner in &mut self.runners {
            runner.run_world(world);
        }
    }
}

pub trait Join<'a> {
    type Iterators;
    type Result;
    fn join_all(&'a self) -> JoinIter<'a, Self> where Self: Sized;
    fn next(iters: &mut Self::Iterators) -> Option<Self::Result>;
}

impl <'a, T: AccessComponent<'a>, U: AccessComponent<'a>> Join<'a> for (T, U) {
    type Iterators = (Peekable<T::Iterator>, Peekable<U::Iterator>);
    type Result = (T::Component, U::Component);
    fn join_all(&'a self) -> JoinIter<'a, Self> {
        return JoinIter{
            iterators: (self.0.iter().peekable(), self.1.iter().peekable()),
            phantom: PhantomData,
        }
    }
    fn next(iters: &mut Self::Iterators) -> Option<Self::Result> {
        loop {
            let k1 = { let (k, _) = iters.0.peek()?; *k };
            let k2 = { let (k, _) = iters.1.peek()?; *k };
            if k1 == k2 {
                let (_, v1) = iters.0.next().unwrap();
                let (_, v2) = iters.1.next().unwrap();
                return Some((v1, v2));
            }
            if k1 < k2 {
                iters.0.next().unwrap();
            }
            if k2 > k1 {
                iters.1.next().unwrap();
            }
        }
    }
}

pub struct JoinIter<'a, J: Join<'a>> {
    iterators: J::Iterators,
    phantom: PhantomData<&'a J::Result>,
}

impl <'a, J: Join<'a>> Iterator for JoinIter<'a, J> {
    type Item = J::Result;
    fn next(&mut self) -> Option<Self::Item> {
        J::next(&mut self.iterators)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        component::Component,
        component::Global,
        system,
        system::Join,
        world::{
            ReadComponent, ReadWriteComponent,
            ReadGlobal, ReadWriteGlobal,
            ReadWriteAll,
            World,
        },
    };

    #[derive(Clone,Debug,Default)]
    struct Delta(f32);
    impl Global for Delta {}

    #[derive(Clone,Debug)]
    struct PhysicsStatus {
        object_count: u64,
    }
    impl Global for PhysicsStatus {}

    #[derive(Clone,Debug)]
    struct Position {
        x: f32,
        y: f32,
        z: f32,
    }
    impl Component for Position {}

    #[derive(Clone,Debug)]
    struct Velocity {
        x: f32,
        y: f32,
        z: f32,
    }
    impl Component for Velocity {}

    struct Physics;
    impl<'a> system::System<'a> for Physics {
        type SystemData = ( ReadWriteComponent<'a, Position>
                          , ReadComponent<'a, Velocity>
                          , ReadGlobal<'a, Delta>
                          , ReadWriteGlobal<'a, PhysicsStatus>);

        fn run(&mut self, (pos, vel, delta, status): Self::SystemData) {
            let d = delta.get();
            let mut count = 0u64;
            for (mut p, v) in (pos, vel).join_all() {
                p.x += v.x * d.0;
                p.y += v.y * d.0;
                p.z += v.z * d.0;
                count += 1;
            }
            status.get().object_count = count;
        }
    }

    #[test]
    fn processor() {
        let mut world = World::new();
        world.new_entity().with(Velocity { x: 0.0, y: 0.0, z: 1.0 }).with(Position { x: 1.0, y: 2.0, z: 3.0 }).build();
        world.new_entity().with(Velocity { x: 0.0, y: 0.0, z: 2.0 }).with(Position { x: 4.0, y: 5.0, z: 6.0 }).build();
        world.set_global(Delta(1.0));
        world.set_global(PhysicsStatus { object_count: 0u64 });

        let mut p = system::Processor {
            world: &world,
            runners: Vec::new(),
        };
        p.add_system(Physics);

        let positions = world.components::<Position>();
        assert_eq!(vec![3.0, 6.0],  positions.iter().map(|(_, el)| el.z).collect::<Vec<f32>>());
        p.run();
        assert_eq!(vec![4.0, 8.0],  positions.iter().map(|(_, el)| el.z).collect::<Vec<f32>>());
        world.set_global(Delta(2.0));
        p.run();
        assert_eq!(vec![6.0, 12.0],  positions.iter().map(|(_, el)| el.z).collect::<Vec<f32>>());
        assert_eq!(2, world.global::<PhysicsStatus>().get().object_count);
    }
}
