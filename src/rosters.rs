use crate::positions::Position;
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod v5;

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
impl TranslatedName for Roster {}

impl Roster {
    pub fn list(version: Option<Version>) -> Vec<Roster> {
        match version {
            Some(Version::V4) => vec![],
            Some(Version::V5) | None => v5::roster_list(),
        }
    }

    pub fn definition(&self, version: Option<Version>) -> Option<RosterDefinition> {
        match version {
            Some(Version::V4) => None,
            Some(Version::V5) | None => v5::roster_definition_from(self),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SpecialRule {
    ElvenKingdomsLeague,
    LustrianSuperleague,
}

impl TypeName for SpecialRule {}
impl TranslatedName for SpecialRule {}

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
    pub price: u32,
    pub maximum: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DedicatedFansInformation {
    pub price: u32,
    pub initial: u8,
    pub maximum: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RosterDefinition {
    pub version: Version,
    pub tier: u8,
    pub staff_information: HashMap<Staff, StaffInformation>,
    pub positions: Vec<Position>,
    pub maximum_big_men_quantity: u8,
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

    pub fn staff_information_sorted_by_name(
        &self,
        lang_id: &str,
    ) -> Vec<(Staff, StaffInformation)> {
        let mut staff_sorted: Vec<(Staff, StaffInformation)> = Vec::new();

        for (staff, staff_information) in self.staff_information.clone() {
            staff_sorted.push((staff, staff_information));
        }

        staff_sorted.sort_by(|(a, _), (b, _)| a.name(lang_id).cmp(&b.name(lang_id)));

        staff_sorted
    }
}
