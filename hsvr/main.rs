use std::sync::Arc;
use cgmath as cgm;

use ecs_macros::Access;
use engine_input as input;
use engine_render as render;
use engine_render::material;
use engine_render::vulkan::data;
use engine::{globals, scripting};

use engine_physics as physics;
use engine_util as util;

struct Main {
    light1: ecs::EntityID,
    cube1: ecs::EntityID,

    cx: f32,
    cy: f32,
}

impl Main {
    pub fn new(world: &mut ecs::World, renderer: &mut render::Renderer) -> Self {
        let mut rm = render::resource::Manager::new();
        let mesh = {
            let vertices = Arc::new(vec![
                data::Vertex::new([-0.5, -0.5,  0.5], [ 0.0,  0.0,  1.0], [1.0, 0.0]),
                data::Vertex::new([ 0.5, -0.5,  0.5], [ 0.0,  0.0,  1.0], [0.0, 0.0]),
                data::Vertex::new([ 0.5,  0.5,  0.5], [ 0.0,  0.0,  1.0], [0.0, 1.0]),
                data::Vertex::new([-0.5,  0.5,  0.5], [ 0.0,  0.0,  1.0], [1.0, 1.0]),

                data::Vertex::new([ 0.5, -0.5, -0.5], [ 1.0,  0.0,  0.0], [0.0, 1.0]),
                data::Vertex::new([ 0.5,  0.5, -0.5], [ 1.0,  0.0,  0.0], [1.0, 1.0]),
                data::Vertex::new([ 0.5,  0.5,  0.5], [ 1.0,  0.0,  0.0], [1.0, 0.0]),
                data::Vertex::new([ 0.5, -0.5,  0.5], [ 1.0,  0.0,  0.0], [0.0, 0.0]),

                data::Vertex::new([-0.5, -0.5, -0.5], [-1.0,  0.0,  0.0], [1.0, 1.0]),
                data::Vertex::new([-0.5,  0.5, -0.5], [-1.0,  0.0,  0.0], [0.0, 1.0]),
                data::Vertex::new([-0.5,  0.5,  0.5], [-1.0,  0.0,  0.0], [0.0, 0.0]),
                data::Vertex::new([-0.5, -0.5,  0.5], [-1.0,  0.0,  0.0], [1.0, 0.0]),

                data::Vertex::new([-0.5, -0.5, -0.5], [ 0.0, -1.0,  0.0], [0.0, 1.0]),
                data::Vertex::new([ 0.5, -0.5, -0.5], [ 0.0, -1.0,  0.0], [1.0, 1.0]),
                data::Vertex::new([ 0.5, -0.5,  0.5], [ 0.0, -1.0,  0.0], [1.0, 0.0]),
                data::Vertex::new([-0.5, -0.5,  0.5], [ 0.0, -1.0,  0.0], [0.0, 0.0]),

                data::Vertex::new([-0.5,  0.5, -0.5], [ 0.0,  1.0,  0.0], [1.0, 1.0]),
                data::Vertex::new([ 0.5,  0.5, -0.5], [ 0.0,  1.0,  0.0], [0.0, 1.0]),
                data::Vertex::new([ 0.5,  0.5,  0.5], [ 0.0,  1.0,  0.0], [0.0, 0.0]),
                data::Vertex::new([-0.5,  0.5,  0.5], [ 0.0,  1.0,  0.0], [1.0, 0.0]),

                data::Vertex::new([-0.5, -0.5, -0.5], [ 0.0,  0.0, -1.0], [0.0, 0.0]),
                data::Vertex::new([ 0.5, -0.5, -0.5], [ 0.0,  0.0, -1.0], [1.0, 0.0]),
                data::Vertex::new([ 0.5,  0.5, -0.5], [ 0.0,  0.0, -1.0], [1.0, 1.0]),
                data::Vertex::new([-0.5,  0.5, -0.5], [ 0.0,  0.0, -1.0], [0.0, 1.0]),
            ]);
            let indices = Arc::new(vec![
                0, 1, 2, 2, 3, 0,

                4, 5, 6, 6, 7, 4,
                8, 10, 9, 10, 8, 11,

                12, 13, 14, 14, 15, 12,
                16, 18, 17, 18, 16, 19,

                20, 22, 21, 22, 20, 23,

            ]);
            rm.add(render::Mesh::new(vertices, indices), Some("cube"))
        };

        let material = rm.add(material::PBRMaterialBuilder {
            diffuse: material::Texture::from_image(String::from("//assets/test-128px.png")),
            roughness: material::Texture::from_image(String::from("//assets/test-128px-roughness.png")),
        }.build(), Some("test-128px"));

        let light = rm.add(render::Light::omni_test(), Some("omni"));

        // The Sun (Sol) is 1AU from the Earth. We ignore the diameter of the Sun and the Earth, as
        // these are negligible at this scale.
        let sun_distance: f32 = 149_597_870_700.0;
        // Solar constant: solar radiant power per square meter of earth's area [w/m^2].
        let solar_constant: f32 = 1366.0;
        // Solar luminous emittance (assuming 93 luminous efficacy) [lm/m^2].
        let sun_luminous_emittance: f32 = solar_constant * 93.0;
        // Solar luminour power (integrating over a sphere of radius == sun_distance) [lm].
        let sun_lumen: f32 = sun_luminous_emittance * (4.0 * 3.14159 * sun_distance * sun_distance);

        let sun_color = physics::color::XYZ::new(sun_lumen/3.0, sun_lumen/3.0, sun_lumen/3.0);
        let sun = rm.add(render::Light::omni_with_color(sun_color), Some("sun"));

        // In our scene, the sun at a 30 degree zenith.
        let sun_angle: f32 = (3.14159 * 2.0) / (360.0 / 30.0);
        
        let light1 = world.new_entity()
            .with(render::Transform::at(-10.0, -10.0, -5.0))
            .with(render::Renderable::Light(light))
            .build();
        let cube1 = world.new_entity()
            .with(render::Transform::at(-10.0, -10.0, -5.0))
            .with(render::Renderable::Mesh(mesh, material))
            .build();
        world.new_entity()
            .with(render::Transform::at(0.0, sun_angle.sin() * sun_distance, sun_angle.cos() * sun_distance))
            .with(render::Renderable::Light(sun))
            .build();

        world.set_global(rm);

        Self {
            light1, cube1,
            cx: 0.,
            cy: 0.,
        }
    }
}

#[derive(Access)]
struct MainData<'a> {
    scene_info: ecs::ReadWriteGlobal<'a, render::SceneInfo>,
    time: ecs::ReadGlobal<'a, globals::Time>,
    input: ecs::ReadGlobal<'a, input::Input>,
    transforms: ecs::ReadWriteComponent<'a, render::Transform>,
}

impl<'a> ecs::System <'a> for Main {
    type SystemData = MainData<'a>;
    fn run(&mut self, sd: Self::SystemData) {

        let ts: f32 = (sd.time.get().instant() / 10.0) * 3.14 * 2.0;
        let (dx, dy) = match sd.input.get().mouse_cursor() {
            Some(cursor) => (cursor.dx, cursor.dy),
            _ => (0.0, 0.0),
        };
        self.cx += (dx);
        self.cy += (dy);

        let camera = cgm::Point3::new(
            self.cx.sin() * 20.0,
            (self.cx.cos()*self.cy.cos()) * 20.0,
            self.cy.sin() * 20.0,
        );

        let view = cgm::Matrix4::look_at(
            camera.clone(),
            cgm::Point3::new(0.0, 0.0, 0.0),
            cgm::Vector3::new(0.0, 0.0, 1.0)
        );

        sd.scene_info.get().camera = camera;
        sd.scene_info.get().view = view;
        sd.scene_info.get().lock_cursor = true;

        let lx = 0.0;
        let ly = 4.0;
        let lz = -0.0 + (ts*2.0).sin() * 4.0;

        *sd.transforms.get_mut(self.light1).unwrap() = render::Transform::at(lx, ly, lz);
        let mut ctransform = render::Transform::at(lx, ly, lz);
        ctransform.0 = ctransform.0 * cgmath::Matrix4::from_scale(0.1);
        *sd.transforms.get_mut(self.cube1).unwrap() = ctransform;
    }
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    let mut world = ecs::World::new();
    world.register_component_lua_bindings(render::Transform::bindings());
    world.register_component_lua_bindings(render::Renderable::bindings());
    let mut renderer = render::Renderer::initialize(&mut world);
    let main = Main::new(&mut world, &mut renderer);

    let context = scripting::WorldContext::new(&world);

    let init = util::file::resource("//engine/lua/init.lua").unwrap().string().unwrap();
    context.eval_init(&world, init).unwrap();
    let scene = util::file::resource("//hsvr/lua/scene.lua").unwrap().string().unwrap();
    context.eval_init(&world, scene).unwrap();

    log::info!("Starting...");

    let mut p = ecs::Processor::new();
    p.add_system(main);
    p.add_system(context);
    p.add_system(renderer);

    world.set_global(globals::Time::new());
    world.set_global(input::Input::new());
    loop {
        world.queue_drain();
        world.global_mut::<globals::Time>().get().update();

        p.run(&world);
        let status = world.global::<render::Status>().get();
        if status.closed {
            log::info!("Renderer closed, exiting.");
            return;
        }
    }

}
