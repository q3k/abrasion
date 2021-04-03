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

impl Component for Transform {}

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

impl Component for Renderable {}
