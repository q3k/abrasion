use crate::component;
use crate::world;

pub type ID = u64;

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entity(pub ID);

impl Entity {
    pub fn id(&self) -> ID {
        self.0
    }
}

pub struct EntityBuilder<'a> {
    world: &'a mut world::World,
    ent: Entity,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut world::World, id: ID) -> Self {
        Self {
            world,
            ent: Entity(id),
        }
    }

    pub fn with<T: component::Component>(self, c: T) -> Self {
        self.world.register_component_entity(component::component_id::<T>(), Box::new(c), self.ent);
        self
    }

    pub fn build(self) -> ID {
        let id = self.ent.id();
        self.world.commit(self.ent);

        id
    }
}
