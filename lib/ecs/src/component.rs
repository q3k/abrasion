pub type ID = std::any::TypeId;


pub trait Component: 'static {
}

pub fn component_id<T: Component>() -> ID {
    std::any::TypeId::of::<T>()
}

pub trait Global: 'static {
}

pub fn global_id<T: Global>() -> ID {
    std::any::TypeId::of::<T>()
}
