use GameData;
use systems::Services;
use components::GameComponents;
use ecs::{System, Process};
use glium::glutin::Event;

pub struct Input;

impl Process for Input {
    fn process(&mut self, data: &mut GameData) {
        for event in data.services.display.poll_events() {
            match event {
                Event::Closed => {
                    data.services.running = false;
                },
                // TODO: Other events
                _ => {}
            }
        }
    }
}

impl System for Input {
    type Components = GameComponents;
    type Services = Services;
}
