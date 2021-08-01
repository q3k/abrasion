#![feature(generic_associated_types)]

pub mod borrow;
pub mod component;
pub mod componentmap;
pub mod entity;
pub mod index;
pub mod globalmap;
pub mod system;
pub mod world;

pub use entity::ID as EntityID;
pub use component::{Component, Global};
pub use component::LuaBindings as ComponentLuaBindings;
pub use world::{
    World,
    ReadComponent, ReadWriteComponent,
    ReadGlobal, ReadWriteGlobal,
    ReadWriteAll,
};
pub use system::{
    System, Join, Processor,
};
