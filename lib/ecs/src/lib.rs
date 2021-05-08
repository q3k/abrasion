pub mod borrow;
pub mod component;
pub mod componentmap;
pub mod entity;
pub mod globalmap;
pub mod system;
pub mod world;

pub use entity::ID as EntityID;
pub use component::ID as ComponentID;
pub use component::{Component, Global};
pub use world::{
    World,
    ReadComponent, ReadWriteComponent,
    ReadGlobal, ReadWriteGlobal,
    ReadWriteAll,
};
pub use system::{
    System, Join, Processor,
};
