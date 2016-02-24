use self::EntityType::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityType {
    Goal = 0,
    Checkpoint,
    Player,
    Crawler,
}

impl EntityType {
    pub fn parse(id: u8) -> Result<EntityType, u8> {
        Ok(match id {
            0 => Goal,
            1 => Checkpoint,
            2 => Player,
            3 => Crawler,
            _ => return Err(id),
        })
    }
}
