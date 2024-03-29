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

use std::ffi::CStr;
use std::io::prelude::*;
use std::sync::Arc;

use vulkano::descriptor::descriptor as vdd;
use vulkano::descriptor::pipeline_layout as vdp;
use vulkano::device as vd;
use vulkano::pipeline::shader as vps;

use engine_util::file;

pub struct ShaderDefinition {
    pub name: String,
    pub ty: vps::GraphicsShaderType,
    pub inputs: Vec<vps::ShaderInterfaceDefEntry>,
    pub outputs: Vec<vps::ShaderInterfaceDefEntry>,
    pub uniforms: Vec<vdd::DescriptorDesc>,
    pub push_constants: Vec<vdp::PipelineLayoutDescPcRange>,
}

#[derive(Debug)]
pub enum ShaderError {
    ResourceError(file::ResourceError),
    IOError(std::io::Error),
}

impl From<file::ResourceError> for ShaderError {
    fn from(v: file::ResourceError) -> Self {
        ShaderError::ResourceError(v)
    }
}

impl From<std::io::Error> for ShaderError {
    fn from(v: std::io::Error) -> Self {
        ShaderError::IOError(v)
    }
}

type Result<T> = std::result::Result<T, ShaderError>;

impl ShaderDefinition {
    pub fn load_into(self, device: Arc<vd::Device>) -> Result<LoadedShader> {
        let mut r = file::resource(self.name.clone())?;
        let mut v = vec![];
        r.read_to_end(&mut v)?;

        let module = unsafe {
            vps::ShaderModule::new(device.clone(), &v).unwrap()
        };

        Ok(LoadedShader {
            def: self,
            module: module,
        })
    }
}

pub struct LoadedShader {
    def: ShaderDefinition,
    module: Arc<vps::ShaderModule>,
}

impl LoadedShader {
    fn ios(&self) -> (ShaderInterface, ShaderInterface) {
        (
            ShaderInterface { entries: self.def.inputs.clone() },
            ShaderInterface { entries: self.def.outputs.clone() },
        )
    }

    fn layout(&self) -> RuntimeShaderLayout {
        RuntimeShaderLayout{ descs: vec![self.def.uniforms.clone()], push_constants: self.def.push_constants.clone(), }
    }

    pub fn entry_point<'a, S>(&'a self) -> vps::GraphicsEntryPoint<'a, S, ShaderInterface, ShaderInterface, RuntimeShaderLayout> {
        let (input, output) = self.ios();
        let layout = self.layout();

        unsafe {
            self.module.graphics_entry_point(
                CStr::from_bytes_with_nul_unchecked(b"main\0"),
                input,
                output,
                layout,
                self.def.ty)
        }
    }
}

#[derive (Debug, Clone)]
pub struct RuntimeShaderLayout {
    descs: Vec<Vec<vdd::DescriptorDesc>>,
    push_constants: Vec<vdp::PipelineLayoutDescPcRange>,
}

unsafe impl vdp::PipelineLayoutDesc for RuntimeShaderLayout {
    fn num_sets(&self) -> usize { self.descs.len() }
    fn num_bindings_in_set(&self, set: usize) -> Option<usize> {
        if set >= self.descs.len() {
            return None
        }
        Some(self.descs[set].len())
    }
    fn descriptor(&self, set: usize, binding: usize) -> Option<vdd::DescriptorDesc> {
        if set >= self.descs.len() {
            return None
        }
        if binding >= self.descs[set].len() {
            return None
        }
        Some(self.descs[set][binding].clone())
    }
    fn num_push_constants_ranges(&self) -> usize { self.push_constants.len() }
    fn push_constants_range(&self, num: usize) -> Option<vdp::PipelineLayoutDescPcRange> {
        if num >= self.push_constants.len() {
            return None
        }
        Some(self.push_constants[0].clone())
    }
}

pub struct ShaderInterface {
    entries: Vec<vps::ShaderInterfaceDefEntry>,
}

unsafe impl vps::ShaderInterfaceDef for ShaderInterface {
    type Iter = InterfaceIterator;

    fn elements(&self) -> Self::Iter {
        InterfaceIterator {
            entries: self.entries.clone(),
        }
    }
}

pub struct InterfaceIterator {
    entries: Vec<vps::ShaderInterfaceDefEntry>,
}

impl std::iter::Iterator for InterfaceIterator {
    type Item = vps::ShaderInterfaceDefEntry;
    fn next(&mut self) -> Option<vps::ShaderInterfaceDefEntry> {
        let cur = self.entries.first()?.clone();
        self.entries = self.entries[1..].to_vec();
        Some(cur)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.entries.len();
        (len, Some(len))
    }
}
impl std::iter::ExactSizeIterator for InterfaceIterator {}
