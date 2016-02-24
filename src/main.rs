#![feature(io)]

#[macro_use] extern crate ecs;
#[macro_use] extern crate glium;

extern crate cgmath;
extern crate rustc_serialize;
extern crate time;
extern crate image;

use components::GameComponents;
use glium::DrawParameters;

pub mod world;
pub mod components;
pub mod systems;

pub type GameData = ecs::DataHelper<GameComponents, systems::Services>;
pub type GameWorld = ecs::World<systems::GameSystems>;
pub type BuildData<'a> = ecs::BuildData<'a, GameComponents>;

fn main() {
    use glium::DisplayBuild;
    use world::tilemap::load_map;
    use cgmath::Point2;
    use components::*;
    
    let display = glium::glutin::WindowBuilder::new()
        .with_min_dimensions(800, 480)
        .with_dimensions(1280, 720)
        .with_title("ECS Game".into())
        .build_glium()
        .unwrap();
        
    let tileset = Sprite::load_spriteset(
        &[
            "assets/tilesets/basic/wall.png",
            "assets/tilesets/basic/breakable.png",
        ],
        &display,
    ).unwrap();
    
    let services = systems::Services {
        delta_time: 0.0,
        running_time: -1.0,
        running: true,
        tilemap_changed: true,
        tilemap: load_map("assets/levels/level1.txt"),
        tileset: tileset,
        display: display,
        frame: None,
        camera: systems::graphics::Camera::new(),
        draw_params: DrawParameters {
            blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha
                },
                constant_value: (1.0, 1.0, 1.0, 1.0)
            },
            
            ..Default::default()
        }
    };
    
    let mut world = GameWorld::with_services(services);
    
    let display = world.services.display.clone();
    world.create_entity(|e: BuildData, data: &mut GameComponents| {
        data.position.add(&e, Position { position: Point2::new(3.0, 1.0) });
        data.camera_follow.add(&e, ());
        data.sprite.add(&e, Sprite::load(
            &["assets/textures/wat.png"],
            &display,
            1.0,
        ).unwrap());
    });
    
    world.update();
    
    println!("{:?}", world.services.camera);
    
    while world.services.running {
        world.update();
    }
}
