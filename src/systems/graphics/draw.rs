use std::mem;
use GameData;
use systems::Services;
use components::GameComponents;
use ecs::{System, Process};
use glium::Surface;

pub struct BeginDraw;

impl Process for BeginDraw {
    fn process(&mut self, data: &mut GameData) {
        let frame = data.services.display.draw();
        let dimensions = frame.get_dimensions();
        data.services.frame = Some(frame);
        let aspect = dimensions.1 as f32 / dimensions.0 as f32;
        data.services.camera.aspect_ratio = aspect;
    }
}

impl System for BeginDraw {
    type Components = GameComponents;
    type Services = Services;
}

pub struct EndDraw;

impl Process for EndDraw {
    fn process(&mut self, data: &mut GameData) {
        let frame = mem::replace(&mut data.services.frame, None).unwrap();
        frame.finish().unwrap();
    }
}

impl System for EndDraw {
    type Components = GameComponents;
    type Services = Services;
}
