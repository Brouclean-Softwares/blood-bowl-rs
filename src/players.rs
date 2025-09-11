use crate::errors::Error;
use crate::positions::Position;
use crate::rosters::Roster;
use crate::versions::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: Option<i32>,
    pub version: Version,
    pub position: Position,
    pub name: String,
    pub number: i32,
}

impl Player {
    pub fn available(&self) -> bool {
        true
    }

    pub fn value(&self, roster: &Roster) -> Result<u32, Error> {
        let position_price = self
            .position
            .definition(Some(self.version), *roster)
            .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?
            .cost;

        Ok(position_price)
    }

    pub fn current_value(&self, roster: &Roster) -> Result<u32, Error> {
        if self.available() {
            Ok(self.value(roster)?)
        } else {
            Ok(0)
        }
    }
}
