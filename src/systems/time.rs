use time;
use GameData;
use systems::Services;
use components::GameComponents;
use ecs::{System, Process};

pub struct Time;

impl Process for Time {
    fn process(&mut self, data: &mut GameData) {
        let old_time = data.services.running_time;
        let new_time = time::precise_time_s();
        let delta = if old_time > 0.0 {
            new_time - old_time
        } else {
            0.0
        };
        
        data.services.delta_time = delta;
        data.services.running_time = new_time;
    }
}

impl System for Time {
    type Components = GameComponents;
    type Services = Services;
}
