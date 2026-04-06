#[derive(Debug)]
pub enum EntityType {
    Unknown = -1,
    Item = 71,
    Player = 155
}

impl From<i32> for EntityType {
    fn from(id: i32) -> Self {
        match id {
            71 => EntityType::Item,
            155 => EntityType::Player,
            _ => EntityType::Unknown,
        }
    }
}