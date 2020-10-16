use crate::{
    component,
    world::{ReadData, ReadWriteData, World}
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
    type Component: component::Component;
    fn fetch(world: &'a  World) -> Self;
}

impl<'a, T: component::Component> Access<'a> for ReadData<'a, T> {
    type Component = T;
    fn fetch(world: &'a  World) -> Self {
        world.components()
    }
}
impl<'a, T: component::Component> Access<'a> for ReadWriteData<'a, T> {
    type Component = T;
    fn fetch(world: &'a  World) -> Self {
        world.components_mut()
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
    fn add_system<T: System<'a> + 'static>(&mut self, system: T) {
        self.runners.push(Box::new(system));
    }

    fn run(&mut self) {
        for runner in &mut self.runners {
            runner.run_world(self.world);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        component::Component,
        system,
        world::{ReadData, ReadWriteData, World},
    };

    struct Position {
        x: u32,
        y: u32,
        z: u32,
    }
    impl Component for Position {}

    struct Velocity {
        x: u32,
        y: u32,
        z: u32,
    }
    impl Component for Velocity {}

    struct Physics;
    impl<'a> system::System<'a> for Physics {
        type SystemData = (ReadWriteData<'a, Position>, ReadData<'a, Velocity>);

        fn run(&mut self, (mut pos, vel): Self::SystemData) {
            for ((_, mut p), (_, v)) in pos.iter_mut().zip(vel.iter()) {
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
        world.new_entity().with(Velocity { x: 0, y: 0, z: 1 }).with(Position { x: 4, y: 5, z: 6 }).build();

        let mut p = system::Processor {
            world: &world,
            runners: Vec::new(),
        };
        p.add_system(Physics);

        let positions = world.components::<Position>();
        assert_eq!(vec![3, 6],  positions.iter().map(|(_, el)| el.z).collect::<Vec<u32>>());
        p.run();
        assert_eq!(vec![4, 7],  positions.iter().map(|(_, el)| el.z).collect::<Vec<u32>>());
    }
}
