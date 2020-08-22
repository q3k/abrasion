pub type ID = std::any::TypeId;


pub trait Component: 'static {
}

pub fn id<T: Component>() -> ID {
    std::any::TypeId::of::<T>()
}
