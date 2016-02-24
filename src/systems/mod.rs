use std::sync::Arc;
use ecs;
use ecs::system::entity::EntitySystem;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{self, Frame, DrawParameters};
use world::tilemap::Tilemap;
use components::GameComponents;

pub mod gameplay;
pub mod graphics;
pub mod physics;
pub mod input;
pub mod time;

systems! {
    struct GameSystems<GameComponents, Services> {
        // Time
        time: time::Time = time::Time,
        
        // Input
        input: input::Input = input::Input,
        
        // Gameplay
        camera_follow: EntitySystem<gameplay::CameraFollow> = EntitySystem::new(
            gameplay::CameraFollow,
            aspect!(<GameComponents> all: [camera_follow, position]),
        ),
        
        // TODO: Physics
        
        // Graphics
        animate: EntitySystem<graphics::Animate> = EntitySystem::new(
            graphics::Animate,
            aspect!(<GameComponents> all: [sprite]),
        ),
        begin_draw: graphics::BeginDraw = graphics::BeginDraw,
        draw_terrain: graphics::DrawTerrain = graphics::DrawTerrain::new(),
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
    pub tilemap_changed: bool,
    pub tilemap: Tilemap,
    pub tileset: Arc<glium::texture::Texture2dArray>,
    pub display: GlutinFacade,
    pub frame: Option<Frame>,
    pub camera: graphics::Camera,
    pub draw_params: DrawParameters<'static>,
}

impl ecs::ServiceManager for Services {}

