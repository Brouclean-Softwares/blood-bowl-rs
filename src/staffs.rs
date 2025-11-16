use crate::positions::{Position, PositionDefinition};
use crate::rosters::Roster;
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};

pub mod v5;
pub mod v5s3;

#[derive(sqlx::Type, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[sqlx(type_name = "varchar")]
pub enum Staff {
    Cheerleader,
    AssistantCoach,
    Apothecary,
    ReRoll,
}

impl TypeName for Staff {}
impl TranslatedName for Staff {}

#[derive(Debug, Copy, Serialize, Deserialize, Clone)]
pub struct StaffInformation {
    pub staff: Staff,
    pub price: u32,
    pub maximum: u8,
}

impl StaffInformation {
    pub const CHEERLEADER: Self = Self {
        staff: Staff::Cheerleader,
        price: 10000,
        maximum: 12,
    };

    pub const ASSISTANT: Self = Self {
        staff: Staff::AssistantCoach,
        price: 10000,
        maximum: 6,
    };

    pub const APOTHECARY: Self = Self {
        staff: Staff::Apothecary,
        price: 50000,
        maximum: 1,
    };

    pub const REROLL_50: Self = Self {
        staff: Staff::ReRoll,
        price: 50000,
        maximum: 8,
    };

    pub const REROLL_60: Self = Self {
        staff: Staff::ReRoll,
        price: 60000,
        maximum: 8,
    };

    pub const REROLL_70: Self = Self {
        staff: Staff::ReRoll,
        price: 70000,
        maximum: 8,
    };

    pub fn price(&self, team_under_creation: bool) -> u32 {
        if !team_under_creation && matches!(self.staff, Staff::ReRoll) {
            self.price * 2
        } else {
            self.price
        }
    }
}

#[derive(Debug, Copy, Serialize, Deserialize, Clone, PartialEq)]
pub enum FamousCoachingStaff {
    AyleenAndar,
    FinkDaFixer,
    GalandrilSilverwater,
    JosefBugman,
    KariColdsteel,
    KrotShockwhisker,
    MungoSpinecracker,
    PapaSkullbones,
    ProfessorFronkelheim,
    SchielundScharlitan,
}

impl TypeName for FamousCoachingStaff {}
impl TranslatedName for FamousCoachingStaff {}

impl FamousCoachingStaff {
    pub fn list(version: &Version) -> Vec<Self> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => Vec::new(),
            Version::V5 => v5::famous_coaching_staff_list(),
            Version::V5S3 => v5s3::famous_coaching_staff_list(),
        }
    }

    pub fn maximum_for_roster(&self, roster: &Roster, version: &Version) -> usize {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => 0,
            Version::V5 => v5::famous_coaching_staff_maximum_for_roster(self, roster),
            Version::V5S3 => v5s3::famous_coaching_staff_maximum_for_roster(self, roster),
        }
    }

    pub fn price(&self, version: &Version) -> u32 {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => 0,
            Version::V5 => v5::famous_coaching_staff_price(self),
            Version::V5S3 => v5s3::famous_coaching_staff_price(self),
        }
    }

    pub fn position(&self) -> Option<Position> {
        match self {
            Self::JosefBugman => Some(Position::JosefBugman),
            Self::KariColdsteel => Some(Position::KariColdsteel),

            _ => None,
        }
    }

    pub fn position_definition(
        position: &Position,
        version: &Version,
    ) -> Option<PositionDefinition> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => None,
            Version::V5 => v5::staff_position_definition(position),
            Version::V5S3 => v5s3::staff_position_definition(position),
        }
    }
}
