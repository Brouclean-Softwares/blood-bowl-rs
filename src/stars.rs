use crate::positions::{Position, PositionDefinition};
use crate::versions::Version;

pub mod v5;
pub mod v5s3;

pub fn star_position_list(version: &Version) -> Vec<Position> {
    match version {
        Version::V1 | Version::V2 | Version::V3 | Version::V4 => Vec::new(),
        Version::V5 => v5::star_position_list(),
        Version::V5S3 => v5s3::star_position_list(),
    }
}

pub fn mega_star_position_list(version: &Version) -> Vec<Position> {
    match version {
        Version::V1 | Version::V2 | Version::V3 | Version::V4 => Vec::new(),
        Version::V5 => v5::mega_star_position_list(),
        Version::V5S3 => v5s3::mega_star_position_list(),
    }
}

pub fn star_player_position_definition(
    position: &Position,
    version: &Version,
) -> Option<PositionDefinition> {
    match version {
        Version::V1 | Version::V2 | Version::V3 | Version::V4 => None,
        Version::V5 => v5::star_player_position_definition(position),
        Version::V5S3 => v5s3::star_player_position_definition(position),
    }
}
