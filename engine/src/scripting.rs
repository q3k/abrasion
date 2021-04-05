use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, atomic};

fn debug_str(v: &mlua::Value) -> String {
    match v {
        mlua::Value::String(s) => s.to_str().map_or(format!("{:?}", v), |s| s.to_string()),
        mlua::Value::Integer(i) => format!("{}", i),
        _ => format!("{:?}", v),
    }
}

pub struct WorldContext {
    // TODO(q3k): this leaks memory, right?
    lua: &'static mlua::Lua,
    classes: Arc<Mutex<BTreeMap<String, ScriptedEntityClass>>>,
    instances: Arc<Mutex<InstanceData>>,
}

#[derive(Debug)]
struct InstanceData {
    ecs: BTreeMap<ecs::EntityID, ScriptedEntity>,
    queue: Vec<ScriptedEntity>,
}

#[derive(Debug)]
struct ScriptedEntityClass {
    name: String,
    table: mlua::RegistryKey,
}

#[derive(Debug)]
struct ScriptedEntityClassID {
    name: String,
}
impl mlua::UserData for ScriptedEntityClassID {}

static GLOBAL_SCRIPTED_ENTITY_ID: atomic::AtomicU64 = atomic::AtomicU64::new(0);

#[derive(Debug)]
struct ScriptedEntity {
    class_name: String,
    internal_id: u64,
    ecs_id: Option<ecs::EntityID>,
    table: mlua::RegistryKey,
}

#[derive(Debug)]
struct ScriptedEntityID {
    ecs_id: Option<ecs::EntityID>,
    internal_id: u64,
}
impl mlua::UserData for ScriptedEntityID {}

impl ScriptedEntity {
    fn new(class_name: String, table: mlua::RegistryKey) -> Self {
        let internal_id = GLOBAL_SCRIPTED_ENTITY_ID.fetch_add(1, atomic::Ordering::SeqCst) + 1;
        Self {
            class_name,
            internal_id,
            ecs_id: None,
            table,
        }
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
            queue: Vec::new(),
        }));

        lua.globals().set("sent", lua.create_table().unwrap()).unwrap();

        Self {
            lua,
            classes,
            instances,
        }
    }

    fn scope_sent(
        &self,
        scope: &mlua::Scope,
    ) -> mlua::Result<()> {
        let globals = self.lua.globals();
        {
            let classes = self.classes.clone();
            globals.set("__sent_register", scope.create_function(move |lua, args: (mlua::String, mlua::Table)| -> mlua::Result<_> {
                let name = args.0.to_str()?.to_string();
                let cls = args.1;
                log::info!("Registering Scripted Entity class {} at {:?}", name, cls);

                let sec = ScriptedEntityClass {
                    name: name.clone(),
                    table: lua.create_registry_value(cls)?,
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
                    None => return Err(mlua::prelude::LuaError::RuntimeError(format!("lost secid {:?}", secid.name))),
                };
                let cls: mlua::Table = lua.registry_value(&sec.table)?;

                let table = lua.create_table()?;
                let meta = lua.create_table()?;
                meta.set("__index", cls)?;
                table.set_metatable(Some(meta));

                let sent = ScriptedEntity::new(secid.name.clone(), lua.create_registry_value(table.clone())?);
                table.set("__sent_id", ScriptedEntityID {
                    ecs_id: sent.ecs_id,
                    internal_id: sent.internal_id,
                });
                instances.lock().unwrap().queue.push(sent);

                Ok(table)
            })?)?;
        }

        Ok(())
    }

    pub fn eval_init<T>(&self, val: T) -> mlua::Result<()>
    where
        T: Into<String>
    {
        let val: String = val.into();
        self.lua.scope(|scope| {
            self.scope_sent(scope)?;
            self.lua.load(&val).exec()
        })
    }
}

impl <'system> ecs::System<'system> for WorldContext {
    type SystemData = ecs::ReadWriteAll<'system>;
    fn run(&mut self, sd: Self::SystemData) {
    }
}
