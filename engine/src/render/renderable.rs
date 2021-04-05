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

use cgmath as cgm;

use ecs::Component;
use crate::render::{Light, Mesh, Material};
use crate::render::resource::{ResourceID};

pub struct Transform(pub cgm::Matrix4<f32>);

pub struct TransformBindings {}

impl ecs::ComponentLuaBindings for TransformBindings {
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
    fn id(&self) -> &'static str {
        "Transform"
    }
}

impl Component for Transform {
    fn lua_bindings(&self) -> Option<Box<dyn ecs::ComponentLuaBindings>> {
        Some(Box::new(TransformBindings{}))
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
}

pub enum Renderable {
    Light(ResourceID<Light>),
    Mesh(ResourceID<Mesh>, ResourceID<Material>),
}

impl Component for Renderable {
}
