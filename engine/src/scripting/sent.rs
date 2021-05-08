/// sent - Scripted ENTities.
use std::collections::BTreeMap;
use std::sync::atomic;
use std::rc::Rc;
use std::cell::RefCell;

use crate::render::renderable::{Transform, Renderable};
use crate::scripting::component::{LuaComponent, UserData as ComponentUserData, AnyComponent, StorageError};

use mlua::ToLua;

static GLOBAL_SCRIPTED_ENTITY_ID: atomic::AtomicU64 = atomic::AtomicU64::new(0);

#[derive(Debug)]
pub enum ScriptedEntityError {
    ClassAlreadyRegistered,
    NoSuchClass,
    StorageError(StorageError),
    EntityRemoved,
}

impl From<ScriptedEntityError> for mlua::Error {
    fn from(e: ScriptedEntityError) -> Self {
        mlua::Error::RuntimeError(format!("ScriptedEntityError: {:?}", e))
    }
}

impl From<StorageError> for ScriptedEntityError {
    fn from(e: StorageError) -> Self {
        ScriptedEntityError::StorageError(e)
    }
}

pub type Result<T> = std::result::Result<T, ScriptedEntityError>;



#[derive(Debug, Clone)]
struct ScriptedEntityID {
    internal_id: u64,
}
impl mlua::UserData for ScriptedEntityID {}

#[derive(Debug)]
struct Class {
    name: String,
    cls: mlua::RegistryKey,
    defaults: BTreeMap<String, AnyComponent>,
}

#[derive(Debug)]
enum Status {
    QueuedForCreation(BTreeMap<String, AnyComponent>),
    Exists(ecs::EntityID),
    FailedToCreate,
}

#[derive(Debug)]
struct Entity {
    class_name: String,
    internal_id: u64,
    eid: Option<ecs::EntityID>,
    components: BTreeMap<String, AnyComponent>,
    table: mlua::RegistryKey,
}

impl Entity {
    fn new(
        class_name: String,
        table: mlua::RegistryKey,
        components: BTreeMap<String, AnyComponent>,
    ) -> Self {
        let internal_id = GLOBAL_SCRIPTED_ENTITY_ID.fetch_add(1, atomic::Ordering::SeqCst) + 1;
        Self {
            class_name,
            internal_id,
            eid: None,
            components,
            table,
        }
    }
}

#[derive(Debug)]
struct State {
    classes: BTreeMap<String, Class>,
    ecs: BTreeMap<ecs::EntityID, Entity>,
    internal_to_ecs: BTreeMap<u64, ecs::EntityID>,
    queue: Vec<Entity>,
}

impl State {
    fn get_mut(&mut self, internal: u64) -> Result<&mut Entity> {
        if let Some(eid) = self.internal_to_ecs.get(&internal) {
            return match self.ecs.get_mut(eid) {
                Some(se) => Ok(se),
                None => Err(ScriptedEntityError::EntityRemoved),
            };
        }

        for (i, el) in self.queue.iter().enumerate() {
            if el.internal_id != internal {
                continue
            }
            return Ok(&mut self.queue[i])
        }
        Err(ScriptedEntityError::EntityRemoved)
    }

    fn get(&self, internal: u64) -> Result<&Entity> {
        if let Some(eid) = self.internal_to_ecs.get(&internal) {
            return match self.ecs.get(eid) {
                Some(se) => Ok(se),
                None => Err(ScriptedEntityError::EntityRemoved),
            };
        }

        for el in self.queue.iter() {
            if el.internal_id != internal {
                continue
            }
            return Ok(&el)
        }
        Err(ScriptedEntityError::EntityRemoved)
    }
}

#[derive(Debug)]
pub struct Data {
    state: Rc<RefCell<State>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(State {
                classes: BTreeMap::new(),
                ecs: BTreeMap::new(),
                internal_to_ecs: BTreeMap::new(),
                queue: Vec::new(),
            })),
        }
    }

    fn setup_component_constructors<'a, 'world, 'lua, 'scope>(
        &'a self,
        lua: &'lua mlua::Lua,
        scope: &'a mlua::Scope<'lua, 'scope>,
        world: &'world ecs::World,
    ) -> mlua::Result<()>
    where
        'world: 'scope,
        'scope: 'a,
    {
        lua.globals().set("__component_construct", scope.create_function(move |lua, cfg: mlua::Table| -> mlua::Result<mlua::Value> {
            let idstr: String = cfg.get("idstr")?;
            let args: mlua::Table = cfg.get("args")?;
            // TODO(q3k): generalize, move to components mod?
            match idstr.as_str() {
                "Transform" => Transform::make(&lua, args)?.to_lua(lua),
                "Renderable" => Renderable::make(&lua, args)?.to_lua(lua),
                _ => Err(ScriptedEntityError::NoSuchClass.into()),
            }
        })?)?;

        {
            let state = self.state.clone();
            lua.globals().set("__sent_class_register", scope.create_function(move |lua, cfg: mlua::Table| -> mlua::Result<_> {
                let name: String = cfg.get("name")?;
                let cls: mlua::Table = cfg.get("cls")?;
                let components: mlua::Table = cfg.get("components")?;

                let defaults = components
                    .pairs::<mlua::Value, AnyComponent>()
                    .map(|pair| {
                        // TODO(q3k): don't ignore the key - use it for better type lookup?
                        let (_, any) = pair?;
                        Ok((any.idstr().into(), any))
                    })
                    .collect::<mlua::Result<Vec<(String, AnyComponent)>>>()?;
                let defaults: BTreeMap<String, AnyComponent> = defaults.into_iter().collect();
                let mut state = state.borrow_mut();

                if state.classes.contains_key(&name) {
                    return Err(ScriptedEntityError::ClassAlreadyRegistered.into());
                }
                state.classes.insert(name.clone(), Class {
                    name: name,
                    cls: lua.create_registry_value(cls)?,
                    defaults,
                });
                Ok(())
            })?)?;
        }

        {
            let state = self.state.clone();
            lua.globals().set("__sent_spawn", scope.create_function(move |lua, cfg: mlua::Table| -> mlua::Result<_> {
                let class_name: String = cfg.get("class_name")?;
                let instance: mlua::Table = cfg.get("instance")?;

                let mut state = state.borrow_mut();

                let class = state.classes.get(&class_name).ok_or(ScriptedEntityError::NoSuchClass)?;
                let components = class.defaults.iter().map(|(k, v)| {
                    let component = v.try_clone()?;
                    Ok((k.clone(), component))
                }).collect::<Result<BTreeMap<String, AnyComponent>>>()?;
                let instance = lua.create_registry_value(instance)?;

                let entity = Entity::new(class_name, instance, components);
                let internal_id = entity.internal_id;

                state.queue.push(entity);
                Ok(ScriptedEntityID {
                    internal_id,
                })
            })?)?;
        }
        Ok(())
    }

    fn setup_sent_register<'a, 'world, 'lua, 'scope>(
        &'a self,
        lua: &'lua mlua::Lua,
        scope: &'a mlua::Scope<'lua, 'scope>,
        world: &'world ecs::World,
    ) -> mlua::Result<()>
    where
        'world: 'scope,
        'scope: 'a,
    {
        let state = self.state.clone();
        lua.globals().set("__sent_register", scope.create_function(move |lua, cfg: mlua::Table| {
            let state = state.borrow_mut();

            let name: String = cfg.get("name")?;
            let cls: mlua::Table = cfg.get("cls")?;

            Ok(())
        })?)?;

        Ok(())
    }

    fn setup_sent_components_index<'a, 'world, 'lua, 'scope>(
        &'a self,
        lua: &'lua mlua::Lua,
        scope: &'a mlua::Scope<'lua, 'scope>,
        world: &'world ecs::World,
    ) -> mlua::Result<()>
    where
        'world: 'scope,
        'scope: 'a,
    {
        let state = self.state.clone();
        lua.globals().set("__sent_components_index", scope.create_function(move |lua, args: (ScriptedEntityID, String)| {
            let state = state.borrow();

            let seid = args.0;
            let component_idstr = args.1;
            let ent = state.get(seid.internal_id)?;
            let component = ent.components.get(&component_idstr)
                .ok_or(ScriptedEntityError::NoSuchClass)?;
            Ok(component.view(&lua)?)
        })?)?;
        Ok(())
    }

    pub fn setup<'a, 'world, 'lua, 'scope>(
        &'a self,
        lua: &'lua mlua::Lua,
        scope: &'a mlua::Scope<'lua, 'scope>,
        world: &'world ecs::World,
    ) -> mlua::Result<()>
    where
        'world: 'scope,
        'scope: 'a,
    {
        self.setup_component_constructors(lua, scope, world)?;
        self.setup_sent_register(lua, scope, world)?;
        self.setup_sent_components_index(lua, scope, world)?;
        Ok(())
    }

    pub fn drain_queue<'a, 'world, 'lua>(
        &'a self,
        lua: &'lua mlua::Lua,
        world: &'world ecs::World,
    ) -> Result<()>
    {
        let mut state = self.state.borrow_mut();
        if state.queue.len() > 0 {
            let queue = std::mem::replace(&mut state.queue, Vec::new());
            for mut el in queue.into_iter() {
                if !el.eid.is_none() {
                    panic!("Entity in queue with eid present: {:?}", el);
                }

                let mut entity = world.new_entity_lazy();
                for (_, component) in el.components.iter_mut() {
                    let inner = component.take();
                    entity = entity.with_dyn(inner);
                }
                let ecsid = entity.build(&world);
                for (_, component) in el.components.iter_mut() {
                    component.commit(ecsid, &world);
                }
                el.eid = Some(ecsid);
                log::debug!("Spawned scripted entity {:?}", el);
                state.internal_to_ecs.insert(el.internal_id, ecsid);
                state.ecs.insert(ecsid, el);
            }
        }
        Ok(())
    }

    pub fn run_tick<'a, 'world, 'lua>(
        &'a self,
        lua: &'lua mlua::Lua,
        world: &'world ecs::World,
    ) -> mlua::Result<()>
    {
        let state = self.state.clone();
        let tickers = {
            let state = state.borrow();
            state.ecs.iter().map(|(ecsid, entity)| {
                let object: mlua::Table = lua.registry_value(&entity.table)?;
                let f: mlua::Function = object.raw_get("__sent_tick")?;
                Ok((*ecsid, f))
            }).collect::<mlua::Result<Vec<(ecs::EntityID, mlua::Function)>>>()?
        };

        for (eid, ticker) in tickers {
            match ticker.call::<mlua::Value, mlua::Value>(mlua::Value::Nil) {
                Ok(_) => (),
                Err(e) => {
                    log::warn!("Failed to run tick() for entity {:?}: {:?}", eid, e);
                },
            }
        }

        Ok(())
    }
}
