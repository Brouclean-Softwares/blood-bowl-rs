use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};

pub mod v5;
pub mod v5s3;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(type_name = "varchar")]
pub enum Injury {
    Stunned,
    KO,
    BadlyHurt,
    SeriouslyHurt,
    SeriousInjury,
    HeadInjury,
    SmashedKnee,
    BrokenArm,
    DislocatedHip,
    NeckInjury,
    Dead,

    // Old
    DislocatedShoulder,
}

impl TypeName for Injury {}
impl TranslatedName for Injury {}

impl Injury {
    pub fn list(version: &Version) -> Vec<Injury> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => Vec::new(),
            Version::V5 => v5::injuries_list(),
            Version::V5S3 => v5s3::injuries_list(),
        }
    }

    pub fn injury_in_next_version_with_same_impact(
        &self,
        current_version: &Version,
    ) -> Option<Injury> {
        match current_version.next()? {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => None,
            Version::V5 => Some(v5::mapping_with_previous_version(self)),
            Version::V5S3 => Some(v5s3::mapping_with_previous_version(self)),
        }
    }

    pub fn remains_after_game(&self) -> bool {
        match self {
            Injury::Stunned => false,
            Injury::KO => false,
            Injury::BadlyHurt => false,
            Injury::SeriouslyHurt => true,
            Injury::SeriousInjury => true,
            Injury::HeadInjury => true,
            Injury::SmashedKnee => true,
            Injury::BrokenArm => true,
            Injury::DislocatedHip => true,
            Injury::NeckInjury => true,
            Injury::DislocatedShoulder => true,
            Injury::Dead => true,
        }
    }

    pub fn reduces_movement_allowance(version: &Version) -> Self {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 | Version::V5 => {
                v5::reduces_movement_allowance()
            }
            Version::V5S3 => v5s3::reduces_movement_allowance(),
        }
    }

    pub fn reduces_strength(version: &Version) -> Self {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 | Version::V5 => {
                v5::reduces_strength()
            }
            Version::V5S3 => v5s3::reduces_strength(),
        }
    }

    pub fn reduces_agility(version: &Version) -> Self {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 | Version::V5 => {
                v5::reduces_agility()
            }
            Version::V5S3 => v5s3::reduces_agility(),
        }
    }

    pub fn reduces_passing_ability(version: &Version) -> Self {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 | Version::V5 => {
                v5::reduces_passing_ability()
            }
            Version::V5S3 => v5s3::reduces_passing_ability(),
        }
    }

    pub fn reduces_armour_value(version: &Version) -> Self {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 | Version::V5 => {
                v5::reduces_armour_value()
            }
            Version::V5S3 => v5s3::reduces_armour_value(),
        }
    }
}
