use log;
use std::ffi::CStr;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

use runfiles::Runfiles;
use vulkano::descriptor::descriptor as vdd;
use vulkano::descriptor::pipeline_layout as vdp;
use vulkano::device as vd;
use vulkano::pipeline::shader as vps;

pub struct ShaderDefinition {
    pub name: String,
    pub ty: vps::GraphicsShaderType,
    pub inputs: Vec<vps::ShaderInterfaceDefEntry>,
    pub outputs: Vec<vps::ShaderInterfaceDefEntry>,
}

impl ShaderDefinition {
    pub fn load_into(self, device: Arc<vd::Device>) -> Result<LoadedShader, String> {
        fn stringify(x: std::io::Error) -> String { format!("IO error: {}", x) }

        let r = Runfiles::create().map_err(stringify)?;
        let path = r.rlocation(format!("abrasion/engine/shaders/{}", self.name));

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
        let sets = vec![
            vec![
                vdd::DescriptorDesc {
                    ty: vdd::DescriptorDescTy::Buffer(vdd::DescriptorBufferDesc {
                        dynamic: Some(false),
                        storage: false,
                    }),
                    array_count: 1,
                    readonly: true,
                    stages: vdd::ShaderStages {
                        vertex: true,
                        ..vdd::ShaderStages::none()
                    },
                },
            ],
        ];
        match self.def.ty {
            vps::GraphicsShaderType::Vertex => RuntimeShaderLayout::vertex(sets.clone()),
            vps::GraphicsShaderType::Fragment => RuntimeShaderLayout::fragment(sets.clone()),
            _ => panic!("unknown shader type")
        }
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
    sets: Vec<Vec<vdd::DescriptorDesc>>,

    vertex: bool,
    fragment: bool,
}

impl RuntimeShaderLayout {
    fn vertex(sets: Vec<Vec<vdd::DescriptorDesc>>) -> RuntimeShaderLayout {
        RuntimeShaderLayout {
            sets,
            vertex: true,
            fragment: false,
        }
    }
    fn fragment(sets: Vec<Vec<vdd::DescriptorDesc>>) -> RuntimeShaderLayout {
        RuntimeShaderLayout {
            sets,
            vertex: false,
            fragment: true,
        }
    }
}

unsafe impl vdp::PipelineLayoutDesc for RuntimeShaderLayout {
    fn num_sets(&self) -> usize { self.sets.len() }
    fn num_bindings_in_set(&self, set: usize) -> Option<usize> {
        if set >= self.sets.len() {
            return None
        }
        Some(self.sets[set].len())
    }
    fn descriptor(&self, set: usize, binding: usize) -> Option<vdd::DescriptorDesc> {
        if set >= self.sets.len() {
            return None
        }
        if binding >= self.sets[set].len() {
            return None
        }
        Some(self.sets[set][binding].clone())
    }
    fn num_push_constants_ranges(&self) -> usize { 0 }
    fn push_constants_range(&self, _num: usize) -> Option<vdp::PipelineLayoutDescPcRange> { None }
    //fn num_push_constants_ranges(&self) -> usize { 1 }
    //fn push_constants_range(&self, num: usize) -> Option<vdp::PipelineLayoutDescPcRange> {
    //    match num {
    //        0 => Some(vdp::PipelineLayoutDescPcRange {
    //            offset: 0,
    //            size: 64usize,
    //            stages: vdd::ShaderStages {
    //                vertex: true,
    //                ..vdd::ShaderStages::none()
    //            },
    //        }),
    //        _ => None,
    //    }
    //}
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
