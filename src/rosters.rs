use crate::positions::Position;
use crate::staffs::{Staff, StaffInformation};
use crate::translation::{LOCALES, TranslatedName, TypeName, language_from};
use crate::versions::Version;
use fluent_templates::Loader;
use serde::{Deserialize, Serialize};

pub mod v5;
pub mod v5s3;

#[derive(sqlx::Type, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[sqlx(type_name = "varchar")]
pub enum Roster {
    Amazon,
    BlackOrc,
    ChaosChosen,
    ChaosDwarf,
    ChaosRenegade,
    DarkElf,
    Dwarf,
    ElvenUnion,
    Gnome,
    Goblin,
    Halfling,
    HighElf,
    Human,
    ImperialNobility,
    Khorne,
    Lizardmen,
    NecromanticHorror,
    Norse,
    Nurgle,
    Ogre,
    OldWorldAlliance,
    Orc,
    ShamblingUndead,
    Skaven,
    Snotling,
    TombKings,
    UnderworldDenizens,
    Vampire,
    WoodElf,
}

impl TypeName for Roster {}
impl TranslatedName for Roster {
    fn name(&self, lang_id: &str) -> String {
        LOCALES.lookup(
            &language_from(lang_id),
            &*format!("{}Roster", self.type_name()),
        )
    }
}

impl Roster {
    pub fn list(version: Version) -> Vec<Roster> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => vec![],
            Version::V5 => v5::roster_list(),
            Version::V5S3 => v5s3::roster_list(),
        }
    }

    pub fn definition(&self, version: Version) -> Option<RosterDefinition> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => None,
            Version::V5 => v5::roster_definition_from(self),
            Version::V5S3 => v5s3::roster_definition_from(self),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpecialLeague {
    ElvenKingdomsLeague,
    WoodlandLeague,
}

impl TypeName for SpecialLeague {}
impl TranslatedName for SpecialLeague {}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpecialRule {
    BadlandsBrawl,
    BriberyAndCorruption,
    ElvenKingdomsLeague,
    FavouredOf,
    FavouredOfHashut,
    FavouredOfKhorne,
    FavouredOfNurgle,
    HalflingThimbleCup,
    LowCostLinemen,
    LustrianSuperleague,
    MastersOfUndeath,
    OldWorldClassic,
    SylvanianSpotlight,
    UnderworldChallenge,
    WorldsEdgeSuperleague,
}

impl TypeName for SpecialRule {}
impl TranslatedName for SpecialRule {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DedicatedFansInformation {
    pub price: u32,
    pub initial: u8,
    pub maximum: u8,
}

impl DedicatedFansInformation {
    pub const DEFAULT: Self = Self {
        price: 10000,
        initial: 1,
        maximum: 6,
    };
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RosterDefinition {
    pub version: Version,
    pub tier: u8,
    pub staff_information: Vec<StaffInformation>,
    pub positions: Vec<Position>,
    pub journeyman_position: Position,
    pub maximum_big_men_quantity: u8,
    pub special_leagues: Vec<SpecialLeague>,
    pub special_rules: Vec<SpecialRule>,
    pub dedicated_fans_information: DedicatedFansInformation,
}

impl RosterDefinition {
    pub fn special_rules_names(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.special_rules.len());

        for skill in self.special_rules.clone() {
            names.push(skill.name(lang_id));
        }

        names.join(", ")
    }

    pub fn get_staff_information(&self, staff: &Staff) -> Option<StaffInformation> {
        for information in self.staff_information.iter() {
            if information.staff.eq(staff) {
                return Some(information.clone());
            }
        }

        None
    }

    pub fn staff_information_sorted_by_name(&self, lang_id: &str) -> Vec<StaffInformation> {
        let mut staff_sorted = self.staff_information.clone();

        staff_sorted.sort_by(|a, b| a.staff.name(lang_id).cmp(&b.staff.name(lang_id)));

        staff_sorted
    }

    pub fn contains_staff(&self, staff: &Staff) -> bool {
        for information in self.staff_information.iter() {
            if information.staff.eq(staff) {
                return true;
            }
        }

        false
    }
}
