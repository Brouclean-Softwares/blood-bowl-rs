pub mod v5;

use crate::positions::Position;
use crate::translation::{LOCALES, language_from};
use crate::versions::Version;
use fluent_templates::Loader;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    pub fn list(version: Version) -> Vec<Roster> {
        match version {
            Version::V4 => vec![],
            Version::V5 => v5::roster_list(),
        }
    }

    pub fn name(self, lang_id: &str) -> String {
        match self {
            Roster::Amazon => LOCALES.lookup(&language_from(lang_id), "Amazon"),
            _ => format!("{:?}", self),
        }
    }

    pub fn definition(self, version: Version) -> Option<RosterDefinition> {
        match version {
            Version::V4 => None,
            Version::V5 => v5::roster_definition_from(self),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SpecialRule {
    LustrianSuperleague,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Staff {
    Cheerleader,
    AssistantCoach,
    Apothecary,
    ReRoll,
}

#[derive(Debug, Clone)]
pub struct RosterDefinition {
    pub tier: u8,
    pub staff_prices: HashMap<Staff, u32>,
    pub positions: Vec<Position>,
    pub maximum_big_men_quantity: u8,
    pub special_rule: SpecialRule,
}
