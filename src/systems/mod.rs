use ecs;
use ecs::system::entity::EntitySystem;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{Frame, DrawParameters};
use world::tilemap::Tilemap;
use components::GameComponents;

pub mod graphics;
pub mod physics;
pub mod input;
pub mod time;

systems! {
    struct GameSystems<GameComponents, Services> {
        time: time::Time = time::Time,
        
        input: input::Input = input::Input,
        // TODO: Physics
        
        animate: EntitySystem<graphics::Animate> = EntitySystem::new(
            graphics::Animate,
            aspect!(<GameComponents> all: [sprite]),
        ),
        begin_draw: graphics::BeginDraw = graphics::BeginDraw,
        draw_sprites: EntitySystem<graphics::DrawSprites> = EntitySystem::new(
            graphics::DrawSprites::new(),
            aspect!(<GameComponents> all: [sprite, position]),
        ),
        end_draw: graphics::EndDraw = graphics::EndDraw,
    }
}

pub struct Services {
    pub delta_time: f64,
    pub running_time: f64,
    pub running: bool,
    pub tilemap: Tilemap,
    pub display: GlutinFacade,
    pub frame: Option<Frame>,
    pub camera: graphics::Camera,
    pub draw_params: DrawParameters<'static>,
}

impl ecs::ServiceManager for Services {}

