use log;
use std::ffi::CStr;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::path;

use vulkano::descriptor::descriptor as vdd;
use vulkano::descriptor::pipeline_layout as vdp;
use vulkano::device as vd;
use vulkano::pipeline::shader as vps;

pub struct ShaderDefinition {
    pub name: String,
    pub ty: vps::GraphicsShaderType,
    pub inputs: Vec<vps::ShaderInterfaceDefEntry>,
    pub outputs: Vec<vps::ShaderInterfaceDefEntry>,
    pub uniforms: Vec<vdd::DescriptorDesc>,
    pub push_constants: Vec<vdp::PipelineLayoutDescPcRange>,
}

impl ShaderDefinition {
    pub fn load_into(self, device: Arc<vd::Device>) -> Result<LoadedShader, String> {
        fn stringify(x: std::io::Error) -> String { format!("IO error: {}", x) }

        let path = &crate::util::file::resource_path(self.name.clone());
        log::info!("Loading shader {}", path.to_str().unwrap_or("UNKNOWN"));

        let mut f = File::open(path).map_err(stringify)?;
        let mut v = vec![];
        f.read_to_end(&mut v).map_err(stringify)?;

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
