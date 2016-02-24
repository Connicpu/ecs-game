use GameData;
use systems::Services;
use components::GameComponents;
use ecs::{System, EntityIter};
use ecs::system::entity::EntityProcess;

pub struct Animate;

impl System for Animate {
    type Components = GameComponents;
    type Services = Services;
}

impl EntityProcess for Animate {
    fn process(&mut self, entities: EntityIter<GameComponents>, data: &mut GameData) {
        for e in entities {
            data.components.sprite[e].update(data.services.delta_time);
        }
    }
}

