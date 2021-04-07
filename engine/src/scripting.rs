use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, atomic};

use mlua::prelude::LuaError::RuntimeError;

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
    components: Vec<Box<dyn ecs::Component>>,
}

#[derive(Debug)]
struct ScriptedEntityClassID {
    name: String,
}
impl mlua::UserData for ScriptedEntityClassID {}

static GLOBAL_SCRIPTED_ENTITY_ID: atomic::AtomicU64 = atomic::AtomicU64::new(0);

#[derive(Debug)]
enum ScriptedEntityStatus {
    QueuedForCreation,
    Exists(ecs::EntityID),
    FailedToCreate,
}

#[derive(Debug)]
struct ScriptedEntity {
    class_name: String,
    internal_id: u64,
    status: ScriptedEntityStatus,
    table: mlua::RegistryKey,
}

impl ScriptedEntity {
    fn new(class_name: String, table: mlua::RegistryKey) -> Self {
        let internal_id = GLOBAL_SCRIPTED_ENTITY_ID.fetch_add(1, atomic::Ordering::SeqCst) + 1;
        Self {
            class_name,
            internal_id,
            status: ScriptedEntityStatus::QueuedForCreation,
            table,
        }
    }
}

#[derive(Debug)]
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

    fn scope_sent<'a, 'lua, 'scope>(
        &self,
        scope: &mlua::Scope<'lua, 'scope>,
        world: &'a ecs::World,
    ) -> mlua::Result<()> 
    where
        'a: 'scope,
    {
        let globals = self.lua.globals();
        {
            let classes = self.classes.clone();
            globals.set("__sent_register", scope.create_function(move |lua, cfg: mlua::Table| -> mlua::Result<_> {
                let name: String = cfg.get("name")?;
                let cls: mlua::Table = cfg.get("cls")?;

                let components: mlua::Table = cfg.get("components")?;
                let components: Vec<Box<dyn ecs::Component>> = components
                    .pairs::<mlua::Integer, mlua::AnyUserData>()
                    .map(|pair| match pair {
                        Ok((_, v)) => match world.lua_any_into_dyn::<'a, '_>(&v) {
                            Some(b) => Ok(b),
                            None => Err(RuntimeError(
                                format!("cfg.components type error: not a component")
                            )),
                        },
                        Err(err) => Err(RuntimeError(
                            format!("cfg.components iter error: {:?}", err)
                        )),
                    })
                    .collect::<mlua::Result<Vec<Box<dyn ecs::Component>>>>()?;

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
            globals.set("__sent_new", scope.create_function(move |lua, args: mlua::AnyUserData| {
                let secid = args.borrow::<ScriptedEntityClassID>()?;
                let classes = classes.lock().unwrap();
                let sec = match classes.get(&secid.name) {
                    Some(el) => el,
                    None => return Err(RuntimeError(format!("lost secid {:?}", secid.name))),
                };
                let cls: mlua::Table = lua.registry_value(&sec.cls)?;

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

                let table = lua.create_table()?;
                let meta = lua.create_table()?;
                let dispatch = lua.create_table()?;
                let metameta = lua.create_table()?;

                table.set_metatable(Some(meta.clone()));
                meta.set("__index", dispatch.clone())?;
                dispatch.set_metatable(Some(metameta.clone()));
                metameta.set("__index", cls)?;

                let components = lua.create_table()?;
                dispatch.set("components", components.clone());
                let components_meta = lua.create_table()?;
                components.set_metatable(Some(components_meta));

                let sent = ScriptedEntity::new(secid.name.clone(), lua.create_registry_value(table.clone())?);
                table.set("__sent_id", ScriptedEntityID {
                    internal_id: sent.internal_id,
                });
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
        self.lua.scope(|scope| {
            self.global_set_components(world)?;
            self.scope_sent(scope, world)?;
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
            let mut queue = std::mem::replace(&mut instances.queue, Vec::new());

            for mut el in queue.into_iter() {
                match classes.get(&el.class_name) {
                    Some(sec) => {
                        let mut entity = sd.new_entity_lazy();
                        for component in sec.components.iter() {
                            entity = entity.with_dyn(component.clone_dyn());
                        }
                        let ecsid = entity.build(&sd);
                        el.status = ScriptedEntityStatus::Exists(ecsid);
                        log::debug!("Created sent of type {} with ECS ID {} and internal ID {}", el.class_name, ecsid, el.internal_id);
                        instances.internal_to_ecs.insert(el.internal_id, ecsid);
                        instances.ecs.insert(ecsid, el);
                    },
                    None => {
                        log::warn!("Failed to create entity with internal ID {}: no class {}", el.internal_id, el.class_name);
                        el.status = ScriptedEntityStatus::FailedToCreate;
                    },
                }
            }
        }
    }
}
