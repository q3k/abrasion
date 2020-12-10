use std::marker::PhantomData;
use std::iter::Peekable;

use crate::{
    component,
    world::{ReadData, ReadDataIter, ReadWriteData, ReadWriteDataIter, World}
};

pub trait System<'a> {
    type SystemData: DynamicSystemData<'a>;

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
    //type Component: component::Component;
    type Component;
    type Iterator: Iterator<Item = (u64, Self::Component)>;
    fn fetch(world: &'a  World) -> Self;
    fn iter(&self) -> Self::Iterator;
}

impl<'a, T: component::Component> Access<'a> for ReadData<'a, T> {
    type Component = &'a T;
    type Iterator = ReadDataIter<'a, T>;
    fn fetch(world: &'a  World) -> Self {
        world.components()
    }
    fn iter(&self) -> ReadDataIter<'a, T> {
        Self::iter(self)
    }
}
impl<'a, T: component::Component> Access<'a> for ReadWriteData<'a, T> {
    type Component = &'a mut T;
    type Iterator = ReadWriteDataIter<'a, T>;
    fn fetch(world: &'a  World) -> Self {
        world.components_mut()
    }
    fn iter(&self) -> ReadWriteDataIter<'a, T> {
        Self::iter_mut(self)
    }
}

pub trait DynamicSystemData<'a> {
    fn fetch(world: &'a  World) -> Self;
}

impl <'a, T: Access<'a>> DynamicSystemData<'a> for T {
    fn fetch(world: &'a  World) -> Self {
        T::fetch(world)
    }
}

impl <'a, T: Access<'a>, U: Access<'a>> DynamicSystemData<'a> for (T, U) {
    fn fetch(world: &'a  World) -> Self {
        (T::fetch(world), U::fetch(world))
    }
}

pub struct Processor<'a> {
    world: &'a  World,
    runners: Vec<Box<dyn WorldRunner<'a>>>,
}

impl<'a> Processor<'a> {
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

impl <'a, T: Access<'a>, U: Access<'a>> Join<'a> for (T, U) {
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
        system,
        system::Join,
        world::{ReadData, ReadWriteData, World},
    };

    #[derive(Clone,Debug)]
    struct Position {
        x: u32,
        y: u32,
        z: u32,
    }
    impl Component for Position {}

    #[derive(Clone,Debug)]
    struct Velocity {
        x: u32,
        y: u32,
        z: u32,
    }
    impl Component for Velocity {}

    struct Physics;
    impl<'a> system::System<'a> for Physics {
        type SystemData = (ReadWriteData<'a, Position>, ReadData<'a, Velocity>);

        fn run(&mut self, (pos, vel): Self::SystemData) {
            for (mut p, v) in (pos, vel).join_all() {
                p.x += v.x;
                p.y += v.y;
                p.z += v.z;
            }
        }
    }

    #[test]
    fn processor() {
        let mut world = World::new();
        world.new_entity().with(Velocity { x: 0, y: 0, z: 1 }).with(Position { x: 1, y: 2, z: 3 }).build();
        world.new_entity().with(Velocity { x: 0, y: 0, z: 2 }).with(Position { x: 4, y: 5, z: 6 }).build();

        let mut p = system::Processor {
            world: &world,
            runners: Vec::new(),
        };
        p.add_system(Physics);

        let positions = world.components::<Position>();
        assert_eq!(vec![3, 6],  positions.iter().map(|(_, el)| el.z).collect::<Vec<u32>>());
        p.run();
        assert_eq!(vec![4, 8],  positions.iter().map(|(_, el)| el.z).collect::<Vec<u32>>());
    }
}
