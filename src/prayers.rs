use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};

pub mod v5;
pub mod v5s3;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PrayerToNuffle {
    TreacherousTrapdoor,
    FriendsWithTheRef,
    Stiletto,
    IronMan,
    KnuckleDusters,
    BadHabits,
    GreasyCleats,
    BlessingsOfNuffle,
    MolesUnderThePitch,
    PerfectPassing,
    DazzlingCatching,
    FanInteraction,
    FoulingFrenzy,
    ThrowARock,
    UnderScrutiny,
    IntensiveTraining,

    // Old
    BlessedStatueOfNuffle,
    NecessaryViolence,
}

impl TypeName for PrayerToNuffle {}
impl TranslatedName for PrayerToNuffle {}

impl PrayerToNuffle {
    pub fn roll(version: &Version) -> Self {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => Self::FriendsWithTheRef,
            Version::V5 => v5::roll_prayer(),
            Version::V5S3 => v5s3::roll_prayer(),
        }
    }

    pub fn list(version: &Version) -> Vec<Self> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => Vec::new(),
            Version::V5 => v5::prayers_list(),
            Version::V5S3 => v5s3::prayers_list(),
        }
    }
}
