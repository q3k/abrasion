pub use crate::render::renderable::Transform;
use crate::scripting::component::{UserData, LuaComponent};

impl LuaComponent for Transform {
    type C = Transform;

    fn idstr() -> &'static str {
        return "Transform"
    }

    fn new<'lua>(
        lua: &'lua mlua::Lua,
        args: mlua::Table<'lua>,
    ) -> mlua::Result<Self::C> {
        let args: Vec<f32> = args.pairs::<mlua::Value, mlua::Number>().map(|pair| -> mlua::Result<f32> {
            let (k, v) = pair?;
            Ok(v as f32)
        }).collect::<mlua::Result<_>>()?;
        match args.len() {
            0 => Ok(Transform::at(0., 0., 0.)),
            3 => Ok(Transform::at(args[0], args[1], args[2])),
            16 => Ok(Transform(cgmath::Matrix4::new(
                // Matrix4::new takes column-wise arguments, this api takes them row-wise.
                args[0], args[4], args[8], args[12],
                args[1], args[5], args[9], args[13],
                args[2], args[6], args[10], args[14],
                args[3], args[7], args[11], args[15],
            ))),
            _ => {
                Err(mlua::Error::RuntimeError("Transform must be called with 0, 3, or 16 arguments".to_string()))
            },
        }
    }

    fn add_methods<'lua, M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<'lua, UserData<Self>>,
    {
        methods.add_meta_method(mlua::MetaMethod::ToString, move |lua, transform, _: mlua::Value| -> mlua::Result<String> {
            let storage = transform.storage.try_borrow()?;
            let m4 = storage.m4();
            Ok(format!(
                "Transform([{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}])",
                m4.x.x, m4.y.x, m4.z.x, m4.w.x,
                m4.x.y, m4.y.y, m4.z.y, m4.w.y,
                m4.x.z, m4.y.z, m4.z.z, m4.w.z,
                m4.x.w, m4.y.w, m4.z.w, m4.w.w,
            ))
        });
        methods.add_method("xyzw", move |lua, transform, _: mlua::Value| -> mlua::Result<f32> {
            //let storage = transform.storage.try_borrow()?;
            //let v = storage.xyzw();
            //Ok(vec![v.x, v.y, v.z, v.w])
            Ok(2137.0)
        });
    }
}
