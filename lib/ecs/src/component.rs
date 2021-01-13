pub type ID = std::any::TypeId;


pub trait Component: 'static {
}

pub fn component_id<T: Component>() -> ID {
    std::any::TypeId::of::<T>()
}

pub trait Resource: 'static {
}

pub fn resource_id<T: Resource>() -> ID {
    std::any::TypeId::of::<T>()
}
