use crate::rosters::RosterDefinition;
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};

pub mod v5;
pub mod v5s3;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Success {
    PassingCompletion,
    ThrowingCompletion,
    Deflection,
    Interception,
    Casualty,
    Touchdown,
    MostValuablePlayer,
    StarPlayerPoint,
}

impl TypeName for Success {}
impl TranslatedName for Success {}

impl Success {
    pub fn star_player_points(
        &self,
        roster_definition: &RosterDefinition,
        version: &Version,
    ) -> u32 {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => 0,
            Version::V5 => v5::star_player_points_for_success(self),
            Version::V5S3 => v5s3::star_player_points_for_success(self, roster_definition),
        }
    }
}
