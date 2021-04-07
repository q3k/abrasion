// Copyright 2020 Sergiusz 'q3k' Bazanski <q3k@q3k.org>
//
// This file is part of Abrasion.
//
// Abrasion is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, version 3.
//
// Abrasion is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// Abrasion.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::hash;
use std::cell::Ref;

use cgmath as cgm;

use ecs::{Component, ComponentLuaBindings};
use crate::render::{Light, Mesh, Material};
use crate::render::resource::{ResourceID};

#[derive(Clone, Debug)]
pub struct Transform(pub cgm::Matrix4<f32>);

impl Component for Transform {
    fn id(&self) -> ecs::component::ID {
        ecs::component::component_id::<Transform>()
    }
    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

struct TransformBindings;

impl ComponentLuaBindings for TransformBindings {
    fn globals<'a>(&self, lua: &'a mlua::Lua) -> mlua::Table<'a> {
        let res = lua.create_table().unwrap();
        res.set("new", lua.create_function(|_, args: mlua::Variadic<mlua::Number>| {
            let args: Vec<f32> = args.iter().map(|el| *el as f32).collect();
            let t = match args.len() {
                0 => Transform::at(0., 0., 0.),
                3 => Transform::at(args[0], args[1], args[2]),
                16 => Transform(cgm::Matrix4::new(
                    // Matrix4::new takes column-wise arguments, this api takes them row-wise.
                    args[0], args[4], args[8], args[12],
                    args[1], args[5], args[9], args[13],
                    args[2], args[6], args[10], args[14],
                    args[3], args[7], args[11], args[15],
                )),
                _ => {
                    return Err(mlua::prelude::LuaError::RuntimeError("Transform.new must be called with 0, 3 ,or 16 arguments".to_string()));
                },
            };
            Ok(t)
        }).unwrap()).unwrap();
        res
    }
    fn idstr(&self) -> &'static str {
        "Transform"
    }
    fn id(&self) -> ecs::component::ID {
        ecs::component::component_id::<Transform>()
    }
    fn any_into_dyn<'a>(&self, ud: &'a mlua::AnyUserData) -> Option<Box<dyn Component>> {
        match ud.borrow::<Transform>() {
            Ok(v) => Some(Box::new(Transform::clone(&v))),
            Err(_) => None,
        }
    }
}


impl mlua::UserData for Transform {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("xyzw", |_, this, _: ()| {
            // TODO(q3k): lua wrappers for cgmath
            let xyzw = this.xyzw();
            Ok(vec![xyzw.z, xyzw.y, xyzw.z, xyzw.w])
        });
    }
}

impl Transform {
    pub fn at(x: f32, y: f32, z: f32) -> Self {
        Transform(cgm::Matrix4::from_translation(cgm::Vector3::new(x, y, z)))
    }
    pub fn xyzw(&self) -> cgm::Vector4<f32> {
        self.0 * cgm::Vector4::new(0.0, 0.0, 0.0, 1.0)
    }
    pub fn xyz(&self) -> cgm::Vector3<f32> {
        let res4 = self.xyzw();
        cgm::Vector3::new(res4.x/res4.w, res4.y/res4.w, res4.z/res4.w)
    }
    pub fn m4(&self) -> &cgm::Matrix4<f32> {
        &self.0
    }
    pub fn bindings() -> Box<dyn ComponentLuaBindings> {
        Box::new(TransformBindings)
    }
}

#[derive(Clone, Debug)]
pub enum Renderable {
    Light(ResourceID<Light>),
    Mesh(ResourceID<Mesh>, ResourceID<Material>),
}
impl mlua::UserData for Renderable {}

impl Component for Renderable {
    fn id(&self) -> ecs::component::ID {
        ecs::component::component_id::<Renderable>()
    }
    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

struct RenderableBindings;

impl ComponentLuaBindings for RenderableBindings {
    fn globals<'a>(&self, lua: &'a mlua::Lua) -> mlua::Table<'a> {
        let res = lua.create_table().unwrap();
        res.set("new_mesh", lua.create_function(|_, args: (ResourceID<Mesh>, ResourceID<Light>)| {
            Ok(1337)
        }).unwrap()).unwrap();
        res
    }
    fn idstr(&self) -> &'static str {
        "Renderable"
    }
    fn id(&self) -> ecs::component::ID {
        ecs::component::component_id::<Renderable>()
    }
    fn any_into_dyn<'a>(&self, ud: &'a mlua::AnyUserData) -> Option<Box<dyn Component>> {
        match ud.borrow::<Renderable>() {
            Ok(v) => Some(Box::new(Renderable::clone(&v))),
            Err(_) => None,
        }
    }
}
