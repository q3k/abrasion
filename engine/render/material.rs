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

use engine_physics::color;

#[derive(Debug)]
pub enum Texture<T: color::Color> {
    Color(T),
    ImageRef(String),
}

#[derive(Debug)]
pub struct Material {
    pub diffuse: Texture<color::XYZ>,
    pub roughness: Texture<color::LinearF32>,
}

impl Material {
    pub fn new(
        diffuse: Texture<color::XYZ>,
        roughness: Texture<color::LinearF32>,
    ) -> Self {
        Self {
            diffuse,
            roughness,
        }
    }

}

pub struct PBRMaterialBuilder {
    pub diffuse: Texture<color::XYZ>,
    pub roughness: Texture<color::LinearF32>,
}

impl PBRMaterialBuilder {
    pub fn build(self) -> Material {
        Material::new(self.diffuse, self.roughness)
    }
}

