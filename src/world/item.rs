use self::Item::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Empty,
    Coins(u32),
    ExtraLife,
}

impl Item {
    pub fn parse(value: char) -> Result<Item, char> {
        Ok(match value {
            '-' => Empty,
            'c' => Coins(1),
            'C' => Coins(3),
            '$' => Coins(10),
            '+' => ExtraLife,
            _ => return Err(value),
        })
    }
}
