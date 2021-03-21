use std::marker::PhantomData;
use std::iter::Peekable;

use crate::{
    component,
    world::{
        ReadComponent, ReadComponentIter,
        ReadWriteComponent, ReadWriteComponentIter,
        ReadResource, ReadWriteResource,
        World,
    }
};

pub trait System<'a> {
    type SystemData: Access<'a>;

    fn run(&mut self, sd: Self::SystemData);
}

trait WorldRunner<'a> {
    fn run_world(&mut self, &'a  World);
}

impl<'a, T> WorldRunner<'a> for T
where
    T: System<'a>,
{
    fn run_world(&mut self, world: &'a  World) {
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

impl<'a, T: component::Resource> Access<'a> for ReadResource<'a, T> {
    fn fetch(world: &'a World) -> Self {
        world.resource()
    }
}

impl<'a, T: component::Resource> Access<'a> for ReadWriteResource<'a, T> {
    fn fetch(world: &'a World) -> Self {
        world.resource_mut()
    }
}

impl <'a,
    T: Access<'a>,
    U: Access<'a>,
> Access<'a> for (T, U) {
    fn fetch(world: &'a  World) -> Self {
        (
            T::fetch(world),
            U::fetch(world),
        )
    }
}

impl <'a,
    T: Access<'a>,
    U: Access<'a>,
    V: Access<'a>,
> Access<'a> for (T, U, V) {
    fn fetch(world: &'a  World) -> Self {
        (
            T::fetch(world),
            U::fetch(world),
            V::fetch(world),
        )
    }
}

impl <'a,
    T: Access<'a>,
    U: Access<'a>,
    V: Access<'a>,
    W: Access<'a>,
> Access<'a> for (T, U, V, W) {
    fn fetch(world: &'a  World) -> Self {
        (
            T::fetch(world),
            U::fetch(world),
            V::fetch(world),
            W::fetch(world),
        )
    }
}

pub struct Processor<'a> {
    world: &'a  World,
    runners: Vec<Box<dyn WorldRunner<'a>>>,
}

impl<'a> Processor<'a> {
    pub fn new(world: &'a World) -> Self {
        Self {
            world,
            runners: Vec::new(),
        }
    }
    pub fn add_system<T: System<'a> + 'static>(&mut self, system: T) {
        self.runners.push(Box::new(system));
    }

    pub fn run(&mut self) {
        for runner in &mut self.runners {
            runner.run_world(self.world);
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
        component::Resource,
        system,
        system::Join,
        world::{ReadComponent, ReadWriteComponent, ReadResource, ReadWriteResource, World},
    };

    #[derive(Clone,Debug,Default)]
    struct Delta(f32);
    impl Resource for Delta {}

    #[derive(Clone,Debug)]
    struct PhysicsStatus {
        object_count: u64,
    }
    impl Resource for PhysicsStatus {}

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
                          , ReadResource<'a, Delta>
                          , ReadWriteResource<'a, PhysicsStatus>);

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
        world.set_resource(Delta(1.0));
        world.set_resource(PhysicsStatus { object_count: 0u64 });

        let mut p = system::Processor {
            world: &world,
            runners: Vec::new(),
        };
        p.add_system(Physics);

        let positions = world.components::<Position>();
        assert_eq!(vec![3.0, 6.0],  positions.iter().map(|(_, el)| el.z).collect::<Vec<f32>>());
        p.run();
        assert_eq!(vec![4.0, 8.0],  positions.iter().map(|(_, el)| el.z).collect::<Vec<f32>>());
        world.set_resource(Delta(2.0));
        p.run();
        assert_eq!(vec![6.0, 12.0],  positions.iter().map(|(_, el)| el.z).collect::<Vec<f32>>());
        assert_eq!(2, world.resource::<PhysicsStatus>().get().object_count);
    }
}
