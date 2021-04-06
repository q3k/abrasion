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
        self.ent.id()
    }
}

pub struct LazyEntityBuilder {
    components: Vec<(component::ID, Box<dyn component::Component>)>,
    ent: Entity,
}

impl LazyEntityBuilder {
    pub fn new(id: ID) -> Self {
        Self {
            components: Vec::new(),
            ent: Entity(id),
        }
    }

    pub fn with<T: component::Component>(mut self, c: T) -> Self {
        self.components.push((component::component_id::<T>(), Box::new(c)));
        self
    }

    pub fn build(self, w: &world::World) -> ID {
        for (cid, c) in self.components.into_iter() {
            w.enqueue_register_component_entity(cid, c, self.ent);
        }
        self.ent.id()
    }
}
