pub mod v5;

use crate::positions::Position;
use crate::translation::{LOCALES, language_from};
use crate::versions::Version;
use fluent_templates::Loader;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Hash)]
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

impl Roster {
    pub fn list(version: Option<Version>) -> Vec<Roster> {
        match version {
            Some(Version::V4) => vec![],
            Some(Version::V5) | None => v5::roster_list(),
        }
    }

    pub fn name(self, lang_id: &str) -> String {
        match self {
            Roster::Amazon => LOCALES.lookup(&language_from(lang_id), "Amazon"),
            Roster::BlackOrc => LOCALES.lookup(&language_from(lang_id), "BlackOrc"),
            _ => format!("{:?}", self),
        }
    }

    pub fn definition(self, version: Option<Version>) -> Option<RosterDefinition> {
        match version {
            Some(Version::V4) => None,
            Some(Version::V5) | None => v5::roster_definition_from(self),
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum SpecialRule {
    LustrianSuperleague,
}

impl SpecialRule {
    pub fn name(self, lang_id: &str) -> String {
        match self {
            SpecialRule::LustrianSuperleague => {
                LOCALES.lookup(&language_from(lang_id), "LustrianSuperleague")
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum Staff {
    Cheerleader,
    AssistantCoach,
    Apothecary,
    ReRoll,
}

impl Staff {
    pub fn name(self, lang_id: &str) -> String {
        match self {
            Staff::Cheerleader => LOCALES.lookup(&language_from(lang_id), "Cheerleader"),
            Staff::AssistantCoach => LOCALES.lookup(&language_from(lang_id), "AssistantCoach"),
            Staff::Apothecary => LOCALES.lookup(&language_from(lang_id), "Apothecary"),
            Staff::ReRoll => LOCALES.lookup(&language_from(lang_id), "ReRoll"),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RosterDefinition {
    pub version: Version,
    pub tier: u8,
    pub staff_prices: HashMap<Staff, u32>,
    pub positions: Vec<Position>,
    pub maximum_big_men_quantity: u8,
    pub special_rules: Vec<SpecialRule>,
}

impl RosterDefinition {
    pub fn special_rules_names(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.special_rules.len());

        for skill in self.special_rules.clone() {
            names.push(skill.name(lang_id));
        }

        names.join(", ")
    }
}
