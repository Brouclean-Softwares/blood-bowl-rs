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
