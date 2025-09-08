use crate::positions::Position;
use crate::versions::Version;

pub struct Player {
    pub version: Version,
    pub position: Position,
    pub name: String,
    pub number: i32,
}
