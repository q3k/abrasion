use log;
use std::borrow::Cow;
use std::ffi::CStr;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

use runfiles::Runfiles;
use vulkano::descriptor::descriptor as vdd;
use vulkano::descriptor::pipeline_layout as vdp;
use vulkano::device as vd;
use vulkano::format::Format;
use vulkano::framebuffer as vf;
use vulkano::pipeline as vp;
use vulkano::pipeline::shader as vps;

pub fn pipeline_forward(
    device: Arc<vd::Device>,
    swap_chain_extent: [u32; 2],
    render_pass: Arc<dyn vf::RenderPassAbstract + Send + Sync>,
) -> Arc<dyn vp::GraphicsPipelineAbstract + Send + Sync> {
    let vertex = ShaderDefinition {
        name: "forward_vert.spv".to_string(),
        ty: vps::GraphicsShaderType::Vertex,
        inputs: vec![
            vps::ShaderInterfaceDefEntry { location: 0..1, format: Format::R32G32B32Sfloat, name: Some(Cow::Borrowed("pos")) },
            vps::ShaderInterfaceDefEntry { location: 1..2, format: Format::R32G32B32Sfloat, name: Some(Cow::Borrowed("color")) },
        ],
        outputs: vec![
            vps::ShaderInterfaceDefEntry { location: 0..1, format: Format::R32G32B32Sfloat, name: Some(Cow::Borrowed("fragColor")) }
        ],
    }.load_into(device.clone()).expect("could not load vertex shader");

    let fragment = ShaderDefinition {
        name: "forward_frag.spv".to_string(),
        ty: vps::GraphicsShaderType::Fragment,
        inputs: vec![
            vps::ShaderInterfaceDefEntry { location: 0..1, format: Format::R32G32B32Sfloat, name: Some(Cow::Borrowed("fragColor")) }
        ],
        outputs: vec![
            vps::ShaderInterfaceDefEntry { location: 0..1, format: Format::R32G32B32A32Sfloat, name: Some(Cow::Borrowed("outColor")) }
        ],
    }.load_into(device.clone()).expect("could not load fragment shader");

    let dimensions = [swap_chain_extent[0] as f32, swap_chain_extent[1] as f32];
    let viewport = vp::viewport::Viewport {
        origin: [0.0, 0.0],
        dimensions,
        depth_range: 0.0 .. 1.0,
    };

    Arc::new(vp::GraphicsPipeline::start()
             .vertex_input_single_buffer::<super::data::Vertex>()
             .vertex_shader(vertex.entry_point(), ())
             .triangle_list()
             .primitive_restart(false)
             .viewports(vec![viewport])
             .fragment_shader(fragment.entry_point(), ())
             .depth_clamp(false)
             .polygon_mode_fill()
             .line_width(1.0)
             .cull_mode_back()
             .front_face_clockwise()
             .blend_pass_through()
             .render_pass(vf::Subpass::from(render_pass.clone(), 0).unwrap())
             .build(device.clone())
             .unwrap()
    )
}

struct ShaderDefinition {
    name: String,
    ty: vps::GraphicsShaderType,
    inputs: Vec<vps::ShaderInterfaceDefEntry>,
    outputs: Vec<vps::ShaderInterfaceDefEntry>,
}

impl ShaderDefinition {
    fn load_into(self, device: Arc<vd::Device>) -> Result<LoadedShader, String> {
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

struct LoadedShader {
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

    fn layout(&self) -> ShaderLayout {
        match self.def.ty {
            vps::GraphicsShaderType::Vertex => ShaderLayout(vdd::ShaderStages{ vertex: true, ..vdd::ShaderStages::none() }),
            vps::GraphicsShaderType::Fragment => ShaderLayout(vdd::ShaderStages{ fragment: true, ..vdd::ShaderStages::none() }),
            _ => panic!("unknown shader type")
        }
    }

    pub fn entry_point<'a, S>(&'a self) -> vps::GraphicsEntryPoint<'a, S, ShaderInterface, ShaderInterface, ShaderLayout> {
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
struct ShaderLayout(vdd::ShaderStages);

unsafe impl vdp::PipelineLayoutDesc for ShaderLayout {
    fn num_sets(&self) -> usize { 0 }
    fn num_bindings_in_set(&self, _set: usize) -> Option<usize> { None }
    fn descriptor(&self, _set: usize, _binding: usize) -> Option<vdd::DescriptorDesc> { None }
    fn num_push_constants_ranges(&self) -> usize { 0 }
    fn push_constants_range(&self, _num: usize) -> Option<vdp::PipelineLayoutDescPcRange> { None }
}

struct ShaderInterface {
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
