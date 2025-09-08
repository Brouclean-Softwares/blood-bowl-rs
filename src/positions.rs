use crate::characteristics::Characteristic;
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod v5;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Position {
    // Amazon
    EagleWarriorLinewoman(Roster),
    PythonWarriorThrower(Roster),
    PiranhaWarriorBlitzer(Roster),
    JaguarWarriorBlocker(Roster),

    // Wood Elf
    WoodElfLineman(Roster),
    Thrower(Roster),
    Catcher(Roster),
    Wardancer(Roster),
    LorenForestTreeman(Roster),
}

impl TypeName for Position {}
impl TranslatedName for Position {}

impl Position {
    pub fn definition(self, version: Option<Version>) -> Option<PositionDefinition> {
        match version {
            Some(Version::V4) => None,
            Some(Version::V5) | None => v5::positon_definition_from(self),
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
        let result = Position::Wardancer(Roster::WoodElf).name("en");
        assert_eq!(result, "Wardancer");

        let result = Position::LorenForestTreeman(Roster::WoodElf).name("fr");
        assert_eq!(result, "Loren Forest Treeman");
    }
}
