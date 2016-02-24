#![feature(io)]

#[macro_use] extern crate ecs;
#[macro_use] extern crate glium;

extern crate cgmath;
extern crate rustc_serialize;
extern crate time;
extern crate image;

pub mod world;
pub mod components;
pub mod systems;

pub type GameData = ecs::DataHelper<components::GameComponents, systems::Services>;
pub type GameWorld = ecs::World<systems::GameSystems>;

fn main() {
    use glium::DisplayBuild;
    use world::tilemap::load_map;
    
    let display = glium::glutin::WindowBuilder::new()
        .with_min_dimensions(800, 480)
        .with_dimensions(1280, 720)
        .with_title("ECS Game".into())
        .build_glium()
        .unwrap();
    
    let services = systems::Services {
        delta_time: 0.0,
        running_time: -1.0,
        running: true,
        tilemap: load_map("assets/levels/level1.txt"),
        display: display,
        frame: None,
        camera: systems::graphics::Camera::new(),
    };
    
    let mut world = GameWorld::with_services(services);
    
    while world.services.running {
        world.update();
    }
}
