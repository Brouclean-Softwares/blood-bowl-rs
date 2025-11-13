use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

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
