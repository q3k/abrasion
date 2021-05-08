pub use crate::render::renderable::Renderable;
use crate::render::resource::ResourceID;
use crate::render::{Mesh, Material};
use crate::scripting::component::{UserData, LuaComponent};

impl LuaComponent for Renderable {
    type C = Renderable;

    fn idstr() -> &'static str {
        return "Renderable"
    }

    fn new<'lua>(
        lua: &'lua mlua::Lua,
        args: mlua::Table<'lua>,
    ) -> mlua::Result<Self::C> {
        let mesh: ResourceID<Mesh> = args.get(1)?;
        let material: ResourceID<Material> = args.get(2)?;
        Ok(Renderable::Mesh(mesh, material))
    }
}
