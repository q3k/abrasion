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

use std::sync::Arc;
use std::sync::Mutex;
use std::time;

use image;
use vulkano::device as vd;
use vulkano::format as vf;
use vulkano::image as vm;

use crate::physics::color;
use crate::render::vulkan::data;
use crate::render::vulkan::material::ChannelLayoutVulkan;
use crate::util::file;

#[derive(Debug)]
pub enum Texture<T: ChannelLayoutVulkan> {
    Color(T),
    ImageRef(String),
}

impl<T: ChannelLayoutVulkan> Texture<T> {
    fn vulkan_image(&self, graphics_queue: Arc<vd::Queue>) -> Arc<vm::ImmutableImage<vf::Format>> {
        match self {
            Texture::<T>::Color(c) => c.vulkan_from_value(graphics_queue),
            Texture::<T>::ImageRef(r) => {
                let format = image::ImageFormat::from_path(r).unwrap();
                let r = file::resource(r.clone()).unwrap();
                let img = Arc::new(image::load(r, format).unwrap());
                T::vulkan_from_image(img, graphics_queue)
            },
        }
    }

    pub fn from_color(color: T) -> Self {
        Texture::<T>::Color(color)
    }

    pub fn from_image(name: String) -> Self {
        Texture::<T>::ImageRef(name)
    }
}

#[derive(Debug)]
pub struct Material {
    diffuse: Texture<color::XYZ>,
    roughness: Texture<color::LinearF32>,

    pub id: u64,
    // vulkan cache
    vulkan: Mutex<Option<data::Textures>>,
}

impl Material {
    pub fn new(
        diffuse: Texture<color::XYZ>,
        roughness: Texture<color::LinearF32>,
    ) -> Self {
        Self {
            diffuse,
            roughness,

            // TODO: use a better method
            id: time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos() as u64,
            vulkan: Mutex::new(None),
        }
    }

    pub fn vulkan_textures(
        &self,
        graphics_queue: Arc<vd::Queue>,
    ) -> data::Textures {
        let mut cache = self.vulkan.lock().unwrap();
        match &mut *cache {
            Some(data) => data.clone(),
            None => {
                let diffuse = self.diffuse.vulkan_image(graphics_queue.clone());
                let roughness = self.roughness.vulkan_image(graphics_queue.clone());
                let textures = data::Textures {
                    diffuse, roughness,
                };
                *cache = Some(textures.clone());
                textures
            },
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

