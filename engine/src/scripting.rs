use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, atomic};
use std::io::Read;

use crate::render;
use crate::util;

use mlua::prelude::LuaError::RuntimeError;
use mlua::ToLua;

fn debug_str(v: &mlua::Value) -> String {
    match v {
        mlua::Value::String(s) => s.to_str().map_or(format!("{:?}", v), |s| s.to_string()),
        mlua::Value::Integer(i) => format!("{}", i),
        _ => format!("{:?}", v),
    }
}

#[derive(Debug)]
struct ComponentID {
    id: ecs::component::ID,
    idstr: String,
}
impl mlua::UserData for ComponentID {}

#[derive(Debug)]
struct ScriptedEntityClass {
    name: String,
    cls: mlua::RegistryKey,
    components: BTreeMap<String, Box<dyn ecs::Component>>,
}

#[derive(Debug)]
struct ScriptedEntityClassID {
    name: String,
}
impl mlua::UserData for ScriptedEntityClassID {}

static GLOBAL_SCRIPTED_ENTITY_ID: atomic::AtomicU64 = atomic::AtomicU64::new(0);

#[derive(Debug)]
enum ScriptedEntityStatus {
    QueuedForCreation(BTreeMap<String, Box<dyn ecs::Component>>),
    Exists(ecs::EntityID),
    FailedToCreate,
}

#[derive(Debug)]
struct ScriptedEntity {
    class_name: String,
    internal_id: u64,
    status: ScriptedEntityStatus,
    table: mlua::RegistryKey,
    cls: mlua::RegistryKey,
}

impl ScriptedEntity {
    fn new(
        class_name: String,
        table: mlua::RegistryKey,
        cls: mlua::RegistryKey,
        components: BTreeMap<String, Box<dyn ecs::Component>>,
    ) -> Self {
        let internal_id = GLOBAL_SCRIPTED_ENTITY_ID.fetch_add(1, atomic::Ordering::SeqCst) + 1;
        Self {
            class_name,
            internal_id,
            status: ScriptedEntityStatus::QueuedForCreation(components),
            table,
            cls,
        }
    }

    fn set_metatable(
        &self,
        lua: &mlua::Lua,
        world: &ecs::World,
    ) -> mlua::Result<()>
    {
        // (meta)table tree for entity objects:
        //
        // table: { }
        //   | metatable
        //   V
        // metatable: { __index }
        //                 |
        //   .-------------'
        //   V
        // dispatch: { components.{...}, ... }
        //   | metadata
        //   V
        // metametatable: { __index }
        //                     |
        //   .-----------------'
        //   V
        // cls: { init, tick, ... }

        let table: mlua::Table = lua.registry_value(&self.table)?;
        let cls: mlua::Table = lua.registry_value(&self.cls)?;

        let meta = lua.create_table()?;
        let dispatch = lua.create_table()?;
        let metameta = lua.create_table()?;

        table.set_metatable(Some(meta.clone()));
        meta.set("__index", dispatch.clone())?;
        dispatch.set_metatable(Some(metameta.clone()));
        metameta.set("__index", cls)?;

        table.set("__sent_id", ScriptedEntityID {
            internal_id: self.internal_id,
        });

        let components = lua.create_table()?;
        dispatch.set("components", components.clone())?;

        let componentsmeta = lua.create_table()?;
        componentsmeta.set(
            "__index",
            lua.globals().get::<_, mlua::Function>("__sent_components_index")?,
        )?;
        componentsmeta.set(
            "__newindex",
            lua.globals().get::<_, mlua::Function>("__sent_components_newindex")?,
        )?;
        components.set_metatable(Some(componentsmeta));
        components.raw_set("__sent_id", ScriptedEntityID {
            internal_id: self.internal_id,
        });

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct ScriptedEntityID {
    internal_id: u64,
}
impl mlua::UserData for ScriptedEntityID {}

pub struct WorldContext {
    // TODO(q3k): this leaks memory, right?
    lua: &'static mlua::Lua,
    classes: Arc<Mutex<BTreeMap<String, ScriptedEntityClass>>>,
    instances: Arc<Mutex<InstanceData>>,
}

#[derive(Debug)]
struct InstanceData {
    ecs: BTreeMap<ecs::EntityID, ScriptedEntity>,
    internal_to_ecs: BTreeMap<u64, ecs::EntityID>,
    queue: Vec<ScriptedEntity>,
}

impl InstanceData {
    fn get_mut(&mut self, internal: u64) -> Option<&mut ScriptedEntity> {
        if let Some(eid) = self.internal_to_ecs.get(&internal) {
            if let Some(se) = self.ecs.get_mut(eid) {
                return Some(se);
            }
            panic!("Corrupt InstanceData: internal id {} found in internal_to_ecs, bot not in ecs", internal);
        }

        for (i, el) in self.queue.iter().enumerate() {
            if el.internal_id != internal {
                continue
            }
            return Some(&mut self.queue[i])
        }
        None
    }
    fn get(&self, internal: u64) -> Option<&ScriptedEntity> {
        if let Some(eid) = self.internal_to_ecs.get(&internal) {
            if let Some(se) = self.ecs.get(eid) {
                return Some(se);
            }
            panic!("Corrupt InstanceData: internal id {} found in internal_to_ecs, bot not in ecs", internal);
        }

        for el in self.queue.iter() {
            if el.internal_id != internal {
                continue
            }
            return Some(&el)
        }
        None
    }
}

impl WorldContext {
    pub fn new(world: &ecs::World) -> Self {
        let lua = mlua::Lua::new().into_static();
        log::info!("Lua WorldContext created.");
        lua.globals().set("print", lua.create_function(|_, vals: mlua::Variadic<mlua::Value>| -> mlua::Result<()> {
            let msg: Vec<String> = vals.iter().map(|val| debug_str(val)).collect();
            log::info!("[Lua] {}", msg.join("\t"));
            Ok(())
        }).unwrap()).unwrap();

        let classes = Arc::new(Mutex::new(BTreeMap::new()));
        let instances = Arc::new(Mutex::new(InstanceData {
            ecs: BTreeMap::new(),
            internal_to_ecs: BTreeMap::new(),
            queue: Vec::new(),
        }));

        lua.globals().set("sent", lua.create_table().unwrap()).unwrap();

        let loaders = lua.create_table().unwrap();
        loaders.set(1, lua.create_function(move |lua, name: String| -> mlua::Result<mlua::Value> {
            log::debug!("require({})", name);
            let path = name.clone();
            match util::file::resource(path.clone()) {
                Err(e) => Ok(format!("util::file::resource({}) failed: {:?}", path, e).to_lua(&lua)?),
                Ok(mut reader) => {
                    let mut data: Vec<u8> = Vec::new();
                    match reader.read_to_end(&mut data) {
                        Err(e) => Ok(format!("util::file::reasource read failed: {:?}", e).to_lua(&lua)?),
                        Ok(_) => lua.load(&data).set_name(&path)?.into_function()?.to_lua(&lua),
                    }
                },
            }
        }).unwrap()).unwrap();
        lua.globals().get::<_, mlua::Table>("package").unwrap().set("loaders", loaders).unwrap();

        Self {
            lua,
            classes,
            instances,
        }
    }

    fn global_set_components(&self, world: &ecs::World) -> mlua::Result<()> {
        let globals = self.lua.globals();
        let components = match globals.get("components") {
            Ok(c) => c,
            Err(_) => {
                let components = self.lua.create_table()?;
                globals.set("components", components.clone())?;
                components
            }
        };
        components.set_metatable(None);
        for (idstr, id, bindings) in world.get_component_lua_bindings() {
            let methods = bindings.globals(&self.lua);
            methods.set("__component_component_id", ComponentID {
                id,
                idstr: idstr.clone(),
            });
            components.set(idstr, methods);
        }
        Ok(())
    }

    /// Registers resourcemanager global in Lua, scoped to scope.
    // TODO(q3k): make this generic for all ECS globals.
    fn scope_resourcemanager<'a, 'world, 'lua, 'scope>(
        &'a self,
        scope: &'a mlua::Scope<'lua, 'scope>,
        world: &'world ecs::World,
    ) -> mlua::Result<()>
    where
        'world: 'scope,
        'scope: 'a,
    {
        let globals = self.lua.globals();
        let resourcemanager = self.lua.create_table()?;
        globals.set("resourcemanager", resourcemanager.clone());

        {
            let rm = world.global::<render::resource::Manager>();
            let rm = rm.get();
            resourcemanager.set("get_mesh", scope.create_function(move |lua, name: String| -> mlua::Result<mlua::Value> {
                match rm.by_label::<render::Mesh, _>(&name) {
                    None => Ok(mlua::Value::Nil),
                    Some(r) => Ok(r.to_lua(&lua)?),
                }
            })?)?;
        }
        {
            let rm = world.global::<render::resource::Manager>();
            let rm = rm.get();
            resourcemanager.set("get_material", scope.create_function(move |lua, name: String| -> mlua::Result<mlua::Value> {
                match rm.by_label::<render::Material, _>(&name) {
                    None => Ok(mlua::Value::Nil),
                    Some(r) => Ok(r.to_lua(&lua)?),
                }
            })?)?;
        }
        Ok(())
    }

    fn scope_sent<'a, 'world, 'lua, 'scope>(
        &'a self,
        scope: &'a mlua::Scope<'lua, 'scope>,
        world: &'world ecs::World,
    ) -> mlua::Result<()> 
    where
        'world: 'scope,
        'scope: 'a,
    {
        let globals = self.lua.globals();
        {
            let classes = self.classes.clone();
            globals.set("__sent_register", scope.create_function(move |lua, cfg: mlua::Table| -> mlua::Result<_> {
                let name: String = cfg.get("name")?;
                let cls: mlua::Table = cfg.get("cls")?;

                let components: mlua::Table = cfg.get("components")?;
                let components: Vec<(String, Box<dyn ecs::Component>)> = components
                    .pairs::<mlua::Integer, mlua::AnyUserData>()
                    .map(|pair| match pair {
                        Ok((_, v)) => match world.lua_any_into_dyn::<'world, '_>(&v) {
                            Some(b) => Ok(b),
                            None => Err(RuntimeError(
                                format!("cfg.components type error: not a component")
                            )),
                        },
                        Err(err) => Err(RuntimeError(
                            format!("cfg.components iter error: {:?}", err)
                        )),
                    })
                    .collect::<mlua::Result<Vec<(String, Box<dyn ecs::Component>)>>>()?;
                let components: BTreeMap<String, Box<dyn ecs::Component>> = components.into_iter().collect();

                log::info!("Registering Scripted Entity class {} at {:?}", name, cls);
                log::info!("Components: {:?}", components);

                let sec = ScriptedEntityClass {
                    name: name.clone(),
                    cls: lua.create_registry_value(cls)?,
                    components,
                };
                classes.lock().unwrap().insert(name.clone(), sec);
                Ok(ScriptedEntityClassID {
                    name: name,
                })
            })?)?;
        }
        {
            let classes = self.classes.clone();
            let instances = self.instances.clone();
            globals.set("__sent_components_index", scope.create_function(move |lua, args: (mlua::Table, mlua::Value)| -> mlua::Result<mlua::Value> {
                let table = args.0;
                let key: String = match args.1 {
                    mlua::Value::String(key) => key.to_str()?.to_string(),
                    _ => return Ok(mlua::Value::Nil),
                };
                let seid: ScriptedEntityID = table.raw_get("__sent_id")?;

                let classes = classes.lock().unwrap();
                let instances = instances.lock().unwrap();

                // Get ScriptedEntity that the user requested this component for.
                let se = match instances.get(seid.internal_id) {
                    Some(se) => se,
                    None => {
                        // This shouldn't really happen. Should we panic here? Probably not, as the
                        // Lua side could do some (accidentally) nasty things that could cause it,
                        // especially on reloads.
                        log::warn!("Lua code requested components for unknown entity {}", seid.internal_id);
                        return Err(RuntimeError(format!("unknown entity {}", seid.internal_id)));
                    }
                };

                // Now retrieve the ScriptedEntityClass, which contains information about the
                // components that this entity has.
                let sec = match classes.get(&se.class_name) {
                    Some(sec) => sec,
                    None => {
                        log::warn!("Lua code requested components for entity {} with unknown class {}", seid.internal_id, se.class_name);
                        return Err(RuntimeError(format!("unknown class {}", se.class_name)));
                    }
                };

                // And retrieve the component.
                let component: &Box<dyn ecs::Component> = match sec.components.get(&key) {
                    Some(c) => c,
                    None => return Ok(mlua::Value::Nil),
                };

                let ud: mlua::Value = match component.lua_userdata(lua) {
                    Some(ud) => ud,
                    None => return Err(RuntimeError(format!("unimplemented lua_userdata"))),
                };
                Ok(ud)
            })?)?;
        }
        {
            let classes = self.classes.clone();
            let instances = self.instances.clone();
            globals.set("__sent_components_newindex", scope.create_function(move |lua, args: (mlua::Table, mlua::Value, mlua::AnyUserData)| -> mlua::Result<mlua::Value> {
                let table = args.0;
                let key: String = match args.1 {
                    mlua::Value::String(key) => key.to_str()?.to_string(),
                    _ => return Ok(mlua::Value::Nil),
                };
                let value = args.2;
                let seid: ScriptedEntityID = table.raw_get("__sent_id")?;

                let classes = classes.lock().unwrap();
                let mut instances = instances.lock().unwrap();

                let se = match instances.get_mut(seid.internal_id) {
                    Some(se) => se,
                    None => {
                        log::warn!("Lua code requested components for unknown entity {}", seid.internal_id);
                        return Err(RuntimeError(format!("unknown entity {}", seid.internal_id)));
                    }
                };
                let ecsid = match &mut se.status {
                    ScriptedEntityStatus::QueuedForCreation(c) => {
                        // Queued for creation, update in queue instead of in ECS.
                        let component: &Box<dyn ecs::Component> = match c.get(&key) {
                            Some(c) => c,
                            None => return Ok(mlua::Value::Nil),
                        };
                        let component_value = match component.lua_fromuserdata(&value) {
                            Some(cv) => cv,
                            None => return Err(RuntimeError(format!("unimplemented lua_fromuserdata"))),
                        };
                        c.insert(key, component_value);
                        return Ok(mlua::Value::Nil);
                    },
                    ScriptedEntityStatus::Exists(ecsid) => *ecsid,
                    ScriptedEntityStatus::FailedToCreate => return Err(RuntimeError(format!("cannot set component on failed entity"))),
                };
                let sec = match classes.get(&se.class_name) {
                    Some(sec) => sec,
                    None => {
                        log::warn!("Lua code requested components for entity {} with unknown class {}", seid.internal_id, se.class_name);
                        return Err(RuntimeError(format!("unknown class {}", se.class_name)));
                    }
                };
                let component: &Box<dyn ecs::Component> = match sec.components.get(&key) {
                    Some(c) => c,
                    None => return Ok(mlua::Value::Nil),
                };
                let component_value = match component.lua_fromuserdata(&value) {
                    Some(cv) => cv,
                    None => return Err(RuntimeError(format!("unimplemented lua_fromuserdata"))),
                };
                log::info!("component_value: {:?}", component_value);
                world.component_set_dyn(ecsid, component_value);

                Ok(mlua::Value::Nil)
            })?)?;
        }
        {
            let classes = self.classes.clone();
            let instances = self.instances.clone();
            globals.set("__sent_new", scope.create_function(move |lua, args: mlua::AnyUserData| {
                let classes = classes.lock().unwrap();

                let secid = args.borrow::<ScriptedEntityClassID>()?;
                let sec = match classes.get(&secid.name) {
                    Some(el) => el,
                    None => return Err(RuntimeError(format!("lost secid {:?}", secid.name))),
                };

                let cls: mlua::Table = lua.registry_value(&sec.cls)?;
                let table = lua.create_table()?;

                let components: BTreeMap<String, Box<dyn ecs::Component>> = sec.components.iter().map(|(k, v)| {
                    (k.clone(), v.clone_dyn())
                }).collect();
                let sent = ScriptedEntity::new(
                    secid.name.clone(),
                    lua.create_registry_value(table.clone())?,
                    lua.create_registry_value(cls.clone())?,
                    components,
                );
                sent.set_metatable(lua, world)?;

                instances.lock().unwrap().queue.push(sent);

                Ok(table)
            })?)?;
        }

        Ok(())
    }

    pub fn eval_init<T>(&self, world: &ecs::World, val: T) -> mlua::Result<()>
    where
        T: Into<String>
    {
        let val: String = val.into();
        self.lua.scope(move |scope| {
            self.global_set_components(world)?;
            self.scope_sent(scope, world)?;
            self.scope_resourcemanager(scope, world)?;
            self.lua.load(&val).exec()
        })
    }
}

impl <'system> ecs::System<'system> for WorldContext {
    type SystemData = ecs::ReadWriteAll<'system>;
    fn run(&mut self, sd: Self::SystemData) {
        let mut instances = self.instances.lock().unwrap();
        let classes = self.classes.lock().unwrap();

        // Lazily create enqueued entities.
        if instances.queue.len() > 0 {
            let queue = std::mem::replace(&mut instances.queue, Vec::new());

            for mut el in queue.into_iter() {
                match el.status {
                    ScriptedEntityStatus::QueuedForCreation(components) => {
                        let mut entity = sd.new_entity_lazy();
                        for (_, component) in components.into_iter() {
                            entity = entity.with_dyn(component);
                        }
                        let ecsid = entity.build(&sd);
                        el.status = ScriptedEntityStatus::Exists(ecsid);
                        log::debug!("Created sent of type {} with ECS ID {} and internal ID {}", el.class_name, ecsid, el.internal_id);
                        instances.internal_to_ecs.insert(el.internal_id, ecsid);
                        instances.ecs.insert(ecsid, el);
                    }
                    o => panic!("ScriptedEntity in queue with unexpected status {:?}", o),
                }
            }
        }
    }
}
