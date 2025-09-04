pub mod v5;

use crate::characteristics::Characteristic;
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};
use crate::translation::{LOCALES, language_from};
use crate::versions::Version;
use fluent_templates::Loader;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum Position {
    EagleWarriorLinewoman(Roster),
    PythonWarriorThrower(Roster),
    PiranhaWarriorBlitzer(Roster),
    JaguarWarriorBlocker(Roster),
}

impl Position {
    pub fn name(self, lang_id: &str) -> String {
        match self {
            Position::EagleWarriorLinewoman(Roster::Amazon) => {
                LOCALES.lookup(&language_from(lang_id), "EagleWarriorLinewoman")
            }
            Position::PythonWarriorThrower(Roster::Amazon) => {
                LOCALES.lookup(&language_from(lang_id), "PythonWarriorThrower")
            }
            Position::PiranhaWarriorBlitzer(Roster::Amazon) => {
                LOCALES.lookup(&language_from(lang_id), "PiranhaWarriorBlitzer")
            }
            Position::JaguarWarriorBlocker(Roster::Amazon) => {
                LOCALES.lookup(&language_from(lang_id), "JaguarWarriorBlocker")
            }
            _ => format!("{:?}", self),
        }
    }

    pub fn definition(self, version: Option<Version>) -> Option<PositionDefinition> {
        match version {
            Some(Version::V4) => None,
            Some(Version::V5) | None => v5::positon_definition_from(self),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionDefinition {
    pub maximum_quantity: u8,
    pub cost: u32,
    pub characteristics: HashMap<Characteristic, u8>,
    pub skills: Vec<Skill>,
    pub primary_skill_categories: Vec<SkillCategory>,
    pub secondary_skill_categories: Vec<SkillCategory>,
    pub is_big_man: bool,
}

impl PositionDefinition {
    pub fn movement_allowance(&self) -> &u8 {
        self.characteristics
            .get(&Characteristic::MovementAllowance)
            .unwrap()
    }

    pub fn strength(&self) -> &u8 {
        self.characteristics.get(&Characteristic::Strength).unwrap()
    }

    pub fn agility(&self) -> &u8 {
        self.characteristics.get(&Characteristic::Agility).unwrap()
    }

    pub fn armour_value(&self) -> &u8 {
        self.characteristics
            .get(&Characteristic::ArmourValue)
            .unwrap()
    }

    pub fn passing_ability(&self) -> Option<&u8> {
        self.characteristics.get(&Characteristic::PassingAbility)
    }

    pub fn primary_skill_categories_short_names(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.primary_skill_categories.len());

        for skill_category in self.primary_skill_categories.clone() {
            names.push(skill_category.short_name(lang_id));
        }

        names.join("")
    }

    pub fn secondary_skill_categories_short_names(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.secondary_skill_categories.len());

        for skill_category in self.secondary_skill_categories.clone() {
            names.push(skill_category.short_name(lang_id));
        }

        names.join("")
    }

    pub fn skills_names(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.skills.len());

        for skill in self.skills.clone() {
            names.push(skill.name(lang_id));
        }

        names.join(", ")
    }
}
