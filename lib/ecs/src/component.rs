use std::cell::Ref;

pub type ID = std::any::TypeId;

pub trait LuaBindings {
    fn globals<'a>(&self, lua: &'a mlua::Lua) -> mlua::Table<'a>;
    fn idstr(&self) -> &'static str;
    fn id(&self) -> ID;
    fn any_into_dyn<'a>(&self, ud: &'a mlua::AnyUserData) -> Option<Box<dyn Component>>;
}

pub trait Component: std::fmt::Debug + 'static {
    fn id(&self) -> ID;
    fn clone_dyn(&self) -> Box<dyn Component>;
    fn lua_userdata<'access, 'lua>(&'access self, lua: &'lua mlua::Lua) -> Option<mlua::Value<'lua>> {
        None
    }
    fn lua_fromuserdata<'a>(&self, ud: &'a mlua::AnyUserData) -> Option<Box<dyn Component>> {
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
