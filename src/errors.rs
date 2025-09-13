use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    UnsupportedVersion,
    NotEnoughPlayers,
    TooMuchPlayers,
    TreasuryExceeded,
    RosterNotExist,
    NotEnoughFans,
    TooMuchFans,
    PositionNotInRoster,
    PositionNotDefined,
    PositionMaxExceeded,
    TooMuchBigMen,
    StaffNotInRoster,
    StaffExceededMaximum,
    IncorrectTreasury,
}

impl TypeName for Error {}
impl TranslatedName for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name("en"))
    }
}

impl std::error::Error for Error {}
