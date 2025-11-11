use crate::errors::Error;
use crate::rosters::{Roster, RosterDefinition};

pub(crate) fn roster_list() -> Vec<Roster> {
    vec![]
}

pub(crate) fn roster_definition_from(roster: &Roster) -> Result<RosterDefinition, Error> {
    Err(Error::UnsupportedVersion)
}
