pub type ID = std::any::TypeId;

pub trait LuaBindings {
    fn globals<'a>(&self, lua: &'a mlua::Lua) -> mlua::Table<'a>;
    fn id(&self) -> &'static str;
}

pub trait Component: 'static {
    fn lua_bindings(&self) -> Option<Box<dyn LuaBindings>> {
        None
    }
}

pub fn component_id<T: Component>() -> ID {
    std::any::TypeId::of::<T>()
}

pub trait Global: 'static {
}

pub fn global_id<T: Global>() -> ID {
    std::any::TypeId::of::<T>()
}
