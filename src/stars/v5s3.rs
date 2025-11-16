use crate::positions::{Position, PositionDefinition};
use crate::rosters::Roster;
use crate::versions::Version;

pub(crate) fn star_position_list() -> Vec<Position> {
    Vec::new()
}

pub(crate) fn mega_star_position_list() -> Vec<Position> {
    todo!()
}

pub(crate) fn star_player_position_definition(position: &Position) -> Option<PositionDefinition> {
    None
}

pub(crate) fn star_maximum_for_roster(position: &Position, roster: &Roster) -> usize {
    todo!()
}
