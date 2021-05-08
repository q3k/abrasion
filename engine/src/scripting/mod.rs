use std::io::Read;

use crate::util;
use crate::render;
use crate::globals::Time;

use mlua::ToLua;

pub mod component;
mod sent;

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
    sent: sent::Data,
}

impl WorldContext {
    pub fn new(world: &ecs::World) -> Self {
        let lua = mlua::Lua::new().into_static();
        log::info!("Lua WorldContext created.");

        // Set up print() function to go through logger.
        lua.globals().set("print", lua.create_function(|_, vals: mlua::Variadic<mlua::Value>| -> mlua::Result<()> {
            let msg: Vec<String> = vals.iter().map(|val| debug_str(val)).collect();
            log::info!("[Lua] {}", msg.join("\t"));
            Ok(())
        }).unwrap()).unwrap();

        // Set up require() to only support //depot/relative/paths.lua.
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

        let sent = sent::Data::new();

        Self {
            lua,
            sent,
        }
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

    fn scope_time<'a, 'world, 'lua, 'scope>(
        &'a self,
        scope: &'a mlua::Scope<'lua, 'scope>,
        world: &'world ecs::World,
    ) -> mlua::Result<()>
    where
        'world: 'scope,
        'scope: 'a,
    {
        let globals = self.lua.globals();
        let time = world.global::<Time>();
        globals.set("time", time.get().instant());
        Ok(())
    }

    //fn scope_sent<'a, 'world, 'lua, 'scope>(
    //    &'a self,
    //    scope: &'a mlua::Scope<'lua, 'scope>,
    //    world: &'world ecs::World,
    //) -> mlua::Result<()> 
    //where
    //    'world: 'scope,
    //    'scope: 'a,
    //{
    //    let globals = self.lua.globals();
    //    {
    //        let classes = self.classes.clone();
    //        let instances = self.instances.clone();
    //        globals.set("__sent_components_newindex", scope.create_function(move |lua, args: (mlua::Table, mlua::Value, mlua::AnyUserData)| -> mlua::Result<mlua::Value> {
    //            let table = args.0;
    //            let key: String = match args.1 {
    //                mlua::Value::String(key) => key.to_str()?.to_string(),
    //                _ => return Ok(mlua::Value::Nil),
    //            };
    //            let value = args.2;
    //            let seid: ScriptedEntityID = table.raw_get("__sent_id")?;

    //            let classes = classes.lock().unwrap();
    //            let mut instances = instances.write().unwrap();

    //            let se = match instances.get_mut(seid.internal_id) {
    //                Some(se) => se,
    //                None => {
    //                    log::warn!("Lua code requested components for unknown entity {}", seid.internal_id);
    //                    return Err(RuntimeError(format!("unknown entity {}", seid.internal_id)));
    //                }
    //            };
    //            let ecsid = match &mut se.status {
    //                ScriptedEntityStatus::QueuedForCreation(c) => {
    //                    // Queued for creation, update in queue instead of in ECS.
    //                    let component: &Box<dyn ecs::Component> = match c.get(&key) {
    //                        Some(c) => c,
    //                        None => return Ok(mlua::Value::Nil),
    //                    };
    //                    let component_value = match component.lua_fromuserdata(&value) {
    //                        Some(cv) => cv,
    //                        None => return Err(RuntimeError(format!("unimplemented lua_fromuserdata"))),
    //                    };
    //                    c.insert(key, component_value);
    //                    return Ok(mlua::Value::Nil);
    //                },
    //                ScriptedEntityStatus::Exists(ecsid) => *ecsid,
    //                ScriptedEntityStatus::FailedToCreate => return Err(RuntimeError(format!("cannot set component on failed entity"))),
    //            };
    //            let sec = match classes.get(&se.class_name) {
    //                Some(sec) => sec,
    //                None => {
    //                    log::warn!("Lua code requested components for entity {} with unknown class {}", seid.internal_id, se.class_name);
    //                    return Err(RuntimeError(format!("unknown class {}", se.class_name)));
    //                }
    //            };
    //            let component: &Box<dyn ecs::Component> = match sec.components.get(&key) {
    //                Some(c) => c,
    //                None => return Ok(mlua::Value::Nil),
    //            };
    //            let component_value = match component.lua_fromuserdata(&value) {
    //                Some(cv) => cv,
    //                None => return Err(RuntimeError(format!("unimplemented lua_fromuserdata"))),
    //            };
    //            world.component_set_dyn(ecsid, component_value);

    //            Ok(mlua::Value::Nil)
    //        })?)?;
    //    }
    //    Ok(())
    //}

    pub fn eval_init<T>(&self, world: &ecs::World, val: T) -> mlua::Result<()>
    where
        T: Into<String>
    {
        let val: String = val.into();
        self.lua.scope(move |scope| {
            self.sent.setup(&self.lua, scope, world)?;
            self.scope_time(scope, world)?;
            self.scope_resourcemanager(scope, world)?;

            self.lua.load(&val).exec()
        })
    }
}

impl <'system, 'lua, 'scope> ecs::System<'system> for WorldContext 
where
    'lua: 'scope,
    'system: 'scope,
{
    type SystemData = ecs::ReadWriteAll<'system>;
    fn run(&mut self, sd: Self::SystemData) {
        let world: &ecs::World = &sd;
        self.lua.scope(move |scope| {
            self.sent.drain_queue(&self.lua, world)?;

            self.sent.setup(&self.lua, scope, world)?;
            self.scope_time(scope, world)?;
            self.scope_resourcemanager(scope, world)?;

            self.sent.run_tick(&self.lua, world)
        }).unwrap();
    }
}
