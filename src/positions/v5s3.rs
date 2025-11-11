use crate::errors::Error;
use crate::positions::{Position, PositionDefinition};
use crate::rosters::Roster;

pub fn positon_definition_from(
    roster: Roster,
    position: Position,
) -> Result<PositionDefinition, Error> {
    Err(Error::UnsupportedVersion)
}
