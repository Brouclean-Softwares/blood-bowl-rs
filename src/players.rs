use crate::positions::Position;
use crate::versions::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub version: Version,
    pub position: Position,
    pub name: String,
    pub number: i32,
}
