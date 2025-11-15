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
    NeckInjury,
    DislocatedShoulder,
    Dead,
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
            Injury::NeckInjury => true,
            Injury::DislocatedShoulder => true,
            Injury::Dead => true,
        }
    }
}
