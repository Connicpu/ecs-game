use GameData;
use systems::Services;
use components::GameComponents;
use ecs::{System, EntityIter};
use ecs::system::entity::EntityProcess;
use cgmath::Point;

pub struct CameraFollow;

impl EntityProcess for CameraFollow {
    fn process(&mut self, entities: EntityIter<GameComponents>, data: &mut GameData) {
        for e in entities {
            let pos = data.components.position[e].position.to_vec();
            let center = (data.services.camera.center + pos) / 2.0;
            data.services.camera.center = center;
        }
    }
}

impl System for CameraFollow {
    type Components = GameComponents;
    type Services = Services;
}
