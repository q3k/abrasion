use log;

use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

use cgmath as cgm;
use vulkano::command_buffer as vc;
use vulkano::device as vd;
use vulkano::framebuffer as vf;

use crate::render::renderable;
use crate::render::vulkan::data;
use crate::render::vulkan::pipeline;

enum Command {
    Render(CommandRender),
    Exit,
}

struct CommandRender {
    device: Arc<vd::Device>,
    queue: Arc<vd::Queue>,
    render_pass: Arc<dyn vf::RenderPassAbstract + Send + Sync>,
    pipeline: Arc<pipeline::VulkanoPipeline>,

    matrix_v: cgm::Matrix4<f32>,
    matrix_p: cgm::Matrix4<f32>,
    data: Vec<Arc<renderable::Data>>,

    result: mpsc::Sender<Box<vc::AutoCommandBuffer>>,
}

pub struct Worker {
    handle: Option<thread::JoinHandle<()>>,
    control: mpsc::Sender<Command>,
}

impl Worker {
    pub fn new(id: u64) -> Worker {
        let (control, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            log::info!("Worker {} starting...", id);
            loop {
                let mut done = false;

                match rx.recv() {
                    Err(err) => {
                        log::error!("Worker {} cannot receive, dying: {}", id, err);
                        done = true;
                    },
                    Ok(cmd) => {
                        match cmd {
                            Command::Exit => {
                                log::info!("Worker {} exiting", id);
                                done = true;
                            }
                            Command::Render(r) => {
                                Worker::work_render(r);
                            }
                        }
                    },
                }

                if done {
                    break
                }
            }
        });

        Worker {
            handle: Some(handle),
            control
        }
    }

    fn work_render(r: CommandRender) {
        let qf = r.queue.family();

        let mut builder = vc::AutoCommandBufferBuilder::secondary_graphics_one_time_submit(
            r.device, qf, vf::Subpass::from(r.render_pass, 0).unwrap()).unwrap();

        for d in r.data {
            let ubo = data::UniformBufferObject {
                model: r.matrix_p.clone() * r.matrix_v.clone() * d.get_transform(),
            };
            let (vbuffer, ibuffer) = d.vulkan_buffers(r.queue.clone());
            builder = builder.draw_indexed(r.pipeline.clone(), &vc::DynamicState::none(),
                vec![vbuffer.clone()],
                ibuffer.clone(),
                (),
                ubo).unwrap();
        }

        let buffer = builder.build().unwrap();
        r.result.send(Box::new(buffer)).unwrap();
    }

    pub fn render(
        &self,

        device: Arc<vd::Device>,
        queue: Arc<vd::Queue>,
        render_pass: Arc<dyn vf::RenderPassAbstract + Send + Sync>,
        pipeline: Arc<pipeline::VulkanoPipeline>,

        matrix_v: cgm::Matrix4<f32>,
        matrix_p: cgm::Matrix4<f32>,
        data: Vec<Arc<renderable::Data>>,
    ) -> mpsc::Receiver<Box<vc::AutoCommandBuffer>> {
        let (result, rx) = mpsc::channel();

        let req = Command::Render(CommandRender {
            device, queue, render_pass, pipeline,
            matrix_v, matrix_p, data,

            result
        });

        self.control.send(req).unwrap();
        return rx;
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.control.send(Command::Exit).unwrap();
        self.handle.take().unwrap().join().unwrap();
    }
}
