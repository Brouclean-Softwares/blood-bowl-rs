use crate::characteristics::Characteristic;
use crate::errors::Error;
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod v5;

#[derive(sqlx::Type, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[sqlx(type_name = "varchar")]
pub enum Position {
    Journeyman,

    // Amazon
    EagleWarriorLinewoman,
    PythonWarriorThrower,
    PiranhaWarriorBlitzer,
    JaguarWarriorBlocker,

    // Wood Elf
    WoodElfLineman,
    Thrower,
    Catcher,
    Wardancer,
    LorenForestTreeman,
}

impl TypeName for Position {}
impl TranslatedName for Position {}

impl Position {
    pub fn definition(self, version: Version, roster: Roster) -> Result<PositionDefinition, Error> {
        match version {
            Version::V4 => Err(Error::PositionNotDefined),
            Version::V5 => v5::positon_definition_from(roster, self),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn movement_allowance(&self) -> u8 {
        self.characteristics
            .get(&Characteristic::MovementAllowance)
            .unwrap()
            .clone()
    }

    pub fn strength(&self) -> u8 {
        self.characteristics
            .get(&Characteristic::Strength)
            .unwrap()
            .clone()
    }

    pub fn agility(&self) -> u8 {
        self.characteristics
            .get(&Characteristic::Agility)
            .unwrap()
            .clone()
    }

    pub fn armour_value(&self) -> u8 {
        self.characteristics
            .get(&Characteristic::ArmourValue)
            .unwrap()
            .clone()
    }

    pub fn passing_ability(&self) -> Option<u8> {
        self.characteristics
            .get(&Characteristic::PassingAbility)
            .and_then(|value| Some(*value))
    }

    pub fn primary_skill_categories_first_letter(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.primary_skill_categories.len());

        for skill_category in self.primary_skill_categories.clone() {
            names.push(skill_category.first_letter(lang_id));
        }

        names.join("")
    }

    pub fn secondary_skill_categories_first_letter(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.secondary_skill_categories.len());

        for skill_category in self.secondary_skill_categories.clone() {
            names.push(skill_category.first_letter(lang_id));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        let result = Position::Wardancer.name("en");
        assert_eq!(result, "Wardancer");

        let result = Position::LorenForestTreeman.name("fr");
        assert_eq!(result, "Loren Forest Treeman");
    }
}
