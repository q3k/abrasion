use crate::{component, world};

pub trait System<'a> {
    type SystemData: DynamicSystemData<'a>;

    fn run(&mut self, sd: Self::SystemData);
}

trait WorldRunner<'a> {
    fn run_world(&mut self, &'a world::World);
}

impl<'a, T> WorldRunner<'a> for T
where
    T: System<'a>,
{
    fn run_world(&mut self, world: &'a world::World) {
        let data = T::SystemData::fetch(world);
        self.run(data);
    }
}

pub trait DynamicSystemData<'a> {
    fn fetch(world: &'a world::World) -> Self;
}

impl<'a, T: component::Component> DynamicSystemData<'a> for world::ReadData<'a, T> {
    fn fetch(world: &'a world::World) -> Self {
        world.components()
    }
}

impl<'a, T: component::Component> DynamicSystemData<'a> for world::ReadWriteData<'a, T> {
    fn fetch(world: &'a world::World) -> Self {
        world.components_mut()
    }
}

pub struct Processor<'a> {
    world: &'a world::World,
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
    use crate::{component, system, world};

    struct Position {
        x: u32,
        y: u32,
        z: u32,
    }
    impl component::Component for Position {}

    struct Physics;
    impl<'a> system::System<'a> for Physics {
        type SystemData = world::ReadWriteData<'a, Position>;

        fn run(&mut self, sd: Self::SystemData) {
            for mut p in sd.iter_mut() {
                p.z += 1;
            }
        }
    }

    #[test]
    fn processor() {
        let mut world = world::World::new();
        world.new_entity().with(Position { x: 1, y: 2, z: 3 }).build();
        world.new_entity().with(Position { x: 4, y: 5, z: 6 }).build();


        let mut p = system::Processor {
            world: &world,
            runners: Vec::new(),
        };
        p.add_system(Physics);

        let positions = world.components::<Position>();
        assert_eq!(vec![3, 6],  positions.iter().map(|el| el.z).collect::<Vec<u32>>());
        p.run();
        assert_eq!(vec![4, 7],  positions.iter().map(|el| el.z).collect::<Vec<u32>>());
    }
}
