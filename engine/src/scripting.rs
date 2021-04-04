pub struct WorldContext {
    lua: mlua::Lua,
}

fn debug_str(v: &mlua::Value) -> String {
    match v {
        mlua::Value::String(s) => s.to_str().map_or(format!("{:?}", v), |s| s.to_string()),
        mlua::Value::Integer(i) => format!("{}", i),
        _ => format!("{:?}", v),
    }
}

impl WorldContext {
    pub fn new() -> Self {
        let lua = mlua::Lua::new();
        log::info!("Lua WorldContext created.");
        lua.globals().set("print", lua.create_function(|_, vals: mlua::Variadic<mlua::Value>| -> mlua::Result<()> {
            let msg: Vec<String> = vals.iter().map(|val| debug_str(val)).collect();
            log::info!("[Lua] {}", msg.join("\t"));
            Ok(())
        }).unwrap()).unwrap();

        Self {
            lua,
        }
    }

    pub fn eval(&self, val: &str) -> mlua::Result<()> {
        self.lua.load(val).exec()
    }
}
