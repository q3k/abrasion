use std::cell::Ref;

pub type ID = std::any::TypeId;

pub trait Component: std::fmt::Debug + 'static {
    fn id(&self) -> ID;
    fn clone_dyn(&self) -> Box<dyn Component>;
}

pub fn component_id<T: Component>() -> ID {
    std::any::TypeId::of::<T>()
}

pub trait Global: 'static {
}

pub fn global_id<T: Global>() -> ID {
    std::any::TypeId::of::<T>()
}
