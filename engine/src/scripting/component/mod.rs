use std::ops::Deref;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

use mlua::ToLua;

pub mod transform;
pub mod renderable;

#[derive(Debug)]
pub enum StorageError {
    ComponentRemoved,
    UnknownType,
    AccessError(ecs::componentmap::AccessError),
}

impl From<ecs::componentmap::AccessError> for StorageError {
    fn from(e: ecs::componentmap::AccessError) -> Self {
        StorageError::AccessError(e)
    }
}

impl From<StorageError> for mlua::Error {
    fn from(e: StorageError) -> Self {
        mlua::Error::RuntimeError(format!("StorageError: {:?}", e))
    }
}

pub type Result<T> = std::result::Result<T, StorageError>;

/// LuaComponent is the main trait that is to be implemented by ecs Components that wish to be
/// Lua-accessible.
pub trait LuaComponent
where
    Self: Sized,
{
    type C: ecs::Component + Sized + Clone;

    fn idstr() -> &'static str;
    fn new<'lua>(
        lua: &'lua mlua::Lua,
        args: mlua::Table<'lua>,
    ) -> mlua::Result<Self::C>;

    fn add_methods<'lua, M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<'lua, UserData<Self>>,
    { }

    fn make<'lua>(
        lua: &'lua mlua::Lua,
        args: mlua::Table<'lua>,
    ) -> mlua::Result<UserData<Self>> {
        let val = Self::new(lua, args)?;
        Ok(UserData {
            idstr: Self::idstr(),
            storage: Storage::Shared(Rc::new(RefCell::new(val))),
        })
    }
}

/// UserData are representations of a component object within a Lua UserData. They are smart
/// pointers that either store a standalone component object instance, or contain a ref to the
/// component stored under a entity/component ID pair within an ECS world.
/// 
/// The ECS World reference is erased. This is not sound, but the alternative would be to pass
/// around the World reference all the way through the Lua codebase, which is not worth the
/// trouble. The Lua logic is all contained within an ECS system that can only be triggered from a
/// single World, so this should generally be safe enough.
#[derive(Debug)]
pub struct UserData<L: LuaComponent> {
    storage: Storage<L>,
    pub idstr: &'static str,
}

impl <L: LuaComponent> mlua::UserData for UserData<L> {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        L::add_methods(methods)
    }
}

impl <L: LuaComponent> UserData<L> {
    /// try_borrow attempts to return a ref to the underlying object within the UserData. This is
    /// will either be a ref into the owned object, or into the ECS-managed object.
    /// This can fail if the ECS-managed object has been destroyed.
    pub fn try_borrow<'a>(&'a self) -> Result<StorageRef<'a, L>> {
        self.storage.try_borrow()
    }

    fn take(&mut self) -> L::C {
        self.storage.take()
    }

    fn commit(&mut self, eid: ecs::EntityID, world: &ecs::World) {
        let cid = ecs::component::component_id::<L::C>();
        match self.storage {
            Storage::Empty => {
                self.storage = Storage::ECS(ECSRef {
                    eid, cid, world,
                });
            },
            Storage::ECS(_) => panic!("cannot commit ECS component"),
            Storage::Shared(_) => panic!("cannot commit shared component"),
        }
    }

    /// view is a specialized clone of UserData, used to make copies to pass into Lua that
    /// potentially refer into the original data. When viewing UserData that is backed by Shared
    /// storage, a reference-counted shallow copy will be made. Otherwise, if the data is backed in
    /// ECS, the ECS reference will be shared with the new returned value.
    /// This is used to limit copies when presenting components to Lua code for manipulation - eg.,
    /// when scripted entities wish to update their component data in tick loops.
    fn view(&self) -> Self {
        let storage = match &self.storage {
            Storage::Empty => panic!("cannot view empty component"),
            Storage::ECS(r) => Storage::ECS(ECSRef {
                eid: r.eid,
                cid: r.cid,
                world: r.world,
            }),
            Storage::Shared(c) => Storage::Shared(c.clone()),
        };
        UserData {
            storage,
            idstr: self.idstr,
        }
    }
}

/// Storage is either an ECS Component directly owned by Storage (and shareable via reference
/// counting), or an ECSRef (which is an eid/cid reference into a leaked ECS world). See the
/// UserData for notes about safety.
/// An additional state for ease of synchronization is Empty, used by WorldContext temporarily when
/// moving from Shared to ECS.
#[derive(Debug)]
enum Storage<L: LuaComponent> {
    Shared(Rc<RefCell<L::C>>),
    ECS(ECSRef),
    Empty,
}

#[derive(Debug)]
struct ECSRef {
    eid: ecs::EntityID,
    cid: ecs::ComponentID,
    world: *const ecs::World,
}

impl <L: LuaComponent> Storage<L> {
    fn try_borrow<'a>(&'a self) -> Result<StorageRef<'a, L>> {
        match self {
            Storage::Shared(t) => Ok(StorageRef::Shared(t.borrow())),
            Storage::ECS(c) => {
                unsafe {
                    let component = (*c.world).component_get::<L::C>(c.eid)?;
                    let res: &'a L::C = & *(component.deref() as *const L::C);
                    Ok(StorageRef::ECS(res))
                }
            },
            Storage::Empty => panic!("empty"),
        }
    }
    fn take(&mut self) -> L::C {
        let res = match std::mem::replace(self, Storage::Empty) {
            Storage::Shared(t) => t.borrow().clone(),
            Storage::ECS(_) => panic!("cannot take from component stored in ECS"),
            Storage::Empty => panic!("cannot take from empty component"),
        };
        res
    }
}

pub enum StorageRef<'a, L: LuaComponent>{
    Shared(Ref<'a, L::C>),
    ECS(&'a L::C),
}

impl <'a, L: LuaComponent> StorageRef<'a, L> {
    /// clone returns a new UserData that contains a cloned data of the underlying ECS Component.
    /// The newly returned UserData always owns the new clone of the Component, and might be
    /// commited into an ECS world at a later stage.
    pub fn clone(&'a self) -> UserData<L> {
        let c: L::C = match self {
            StorageRef::Shared(r) => Clone::clone(r),
            StorageRef::ECS(r) => Clone::clone(r),
        };
        UserData {
            idstr: L::idstr(),
            storage: Storage::Shared(Rc::new(RefCell::new(c))),
        }
    }
}

impl <'a, L: LuaComponent> std::ops::Deref for StorageRef<'a, L> {
    type Target = L::C;
    fn deref(&self) -> &Self::Target {
        match self {
            StorageRef::Shared(r) => r.deref(),
            StorageRef::ECS(r) => r,
        }
    }
}

#[derive(Debug)]
pub enum AnyComponent {
    Transform(UserData<transform::Transform>),
    Renderable(UserData<renderable::Renderable>),
}

impl <'lua> mlua::FromLua<'lua> for AnyComponent {
    fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let any: mlua::AnyUserData = mlua::FromLua::from_lua(lua_value, lua)?;
        // TODO(q3k): there's no way to generalize this, right?
        if let Ok(any) = any.borrow::<UserData<transform::Transform>>() {
            return Ok(AnyComponent::Transform(any.try_borrow()?.clone()));
        }
        if let Ok(any) = any.borrow::<UserData<renderable::Renderable>>() {
            return Ok(AnyComponent::Renderable(any.try_borrow()?.clone()));
        }
        Err(StorageError::UnknownType.into())
    }
}

// TODO(q3k): this is _way_ too verbose. If there really is no better way to do this (there
// probably isn't, because this needs dependent types?), at least use a macro here.
impl AnyComponent {
    pub fn id(&self) -> ecs::ComponentID {
        match self {
            AnyComponent::Transform(_) => ecs::component::component_id::<transform::Transform>(),
            AnyComponent::Renderable(_) => ecs::component::component_id::<renderable::Renderable>(),
        }
    }
    pub fn idstr(&self) -> &'static str {
        match self {
            AnyComponent::Transform(_) => transform::Transform::idstr(),
            AnyComponent::Renderable(_) => renderable::Renderable::idstr(),
        }
    }
    pub fn try_clone(&self) -> Result<AnyComponent> {
        match self {
            AnyComponent::Transform(t) => Ok(AnyComponent::Transform(t.try_borrow()?.clone())),
            AnyComponent::Renderable(t) => Ok(AnyComponent::Renderable(t.try_borrow()?.clone())),
        }
    }
    pub fn take(&mut self) -> Box<dyn ecs::Component> {
        match self {
            AnyComponent::Transform(t) => Box::new(t.take()),
            AnyComponent::Renderable(t) => Box::new(t.take()),
        }
    }
    pub fn commit(&mut self, eid: ecs::EntityID, world: &ecs::World) {
        match self {
            AnyComponent::Transform(t) => t.commit(eid, world),
            AnyComponent::Renderable(t) => t.commit(eid, world),
        }
    }
    pub fn view<'lua>(&self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        match self {
            AnyComponent::Transform(t) => t.view().to_lua(&lua),
            AnyComponent::Renderable(t) => t.view().to_lua(&lua),
        }
    }
}
