use crate::dices::Dice;
use crate::errors::Error;
use crate::players::Player;
use crate::skills::{Skill, SkillCategory};
use crate::translation::{LOCALES, TranslatedName, TypeName, language_from};
use crate::versions::Version;
use fluent_templates::Loader;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

pub mod v5;
pub mod v5s3;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AdvancementChoice {
    RandomPrimarySkill(SkillCategory),
    RandomSecondarySkill(SkillCategory),
    ChosenPrimarySkill,
    ChosenSecondarySkill,
    RandomCharacteristic,
}

impl TypeName for AdvancementChoice {}

impl TranslatedName for AdvancementChoice {
    fn name(&self, lang_id: &str) -> String {
        match self {
            AdvancementChoice::RandomPrimarySkill(skill_category) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "RandomPrimarySkill",
                &HashMap::from([(
                    Cow::from("skill_category"),
                    skill_category.name(lang_id).into(),
                )]),
            ),
            AdvancementChoice::RandomSecondarySkill(skill_category) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "RandomSecondarySkill",
                &HashMap::from([(
                    Cow::from("skill_category"),
                    skill_category.name(lang_id).into(),
                )]),
            ),
            _ => LOCALES.lookup(&language_from(lang_id), &*self.type_name()),
        }
    }
}

impl AdvancementChoice {
    pub fn list_could_be_available_for_player(player: &Player) -> Vec<Self> {
        let mut choices_available = vec![];

        if let Some(position_definition) = player.position_definition() {
            for skill_category in position_definition.primary_skill_categories.iter() {
                choices_available.push(Self::RandomPrimarySkill(skill_category.clone()));
            }

            for skill_category in position_definition.secondary_skill_categories.iter() {
                choices_available.push(Self::RandomSecondarySkill(skill_category.clone()));
            }

            for choice in [
                Self::ChosenPrimarySkill,
                Self::ChosenSecondarySkill,
                Self::RandomCharacteristic,
            ] {
                choices_available.push(choice);
            }
        }

        choices_available
    }

    pub fn is_buyable_for_player(&self, player: &Player) -> bool {
        self.star_player_points_cost_for_player(player) as i32 <= player.star_player_points
    }

    pub fn list_available_for_player(player: &Player) -> Result<Vec<Self>, Error> {
        let mut choices_available = vec![];

        for choice in Self::list_could_be_available_for_player(player) {
            if choice.star_player_points_cost_for_player(player) as i32 <= player.star_player_points
            {
                choices_available.push(choice);
            }
        }

        Ok(choices_available)
    }

    pub fn roll_advancements_to_choose_for_player(&self, player: &Player) -> Vec<Advancement> {
        match self {
            AdvancementChoice::RandomPrimarySkill(skill_category)
            | AdvancementChoice::RandomSecondarySkill(skill_category) => {
                let potential_skills = skill_category.skills_available_for_player(player);
                let skill_position = rand::rng().random_range(0..potential_skills.len());

                vec![Advancement::RandomSkill(potential_skills[skill_position])]
            }

            AdvancementChoice::ChosenPrimarySkill => {
                Advancement::primary_skill_advancements_available_for_player(player)
            }

            AdvancementChoice::ChosenSecondarySkill => {
                Advancement::secondary_skill_advancements_available_for_player(player)
            }

            AdvancementChoice::RandomCharacteristic => {
                let dice_result = Dice::D16.roll();

                if dice_result >= 1 && dice_result <= 7 {
                    [
                        vec![Advancement::MovementAllowance, Advancement::ArmourValue],
                        Advancement::secondary_skill_advancements_available_for_player(player),
                    ]
                    .concat()
                } else if dice_result >= 8 && dice_result <= 13 {
                    [
                        vec![
                            Advancement::MovementAllowance,
                            Advancement::PassingAbility,
                            Advancement::ArmourValue,
                        ],
                        Advancement::secondary_skill_advancements_available_for_player(player),
                    ]
                    .concat()
                } else if dice_result >= 14 {
                    [
                        vec![Advancement::Agility, Advancement::PassingAbility],
                        Advancement::secondary_skill_advancements_available_for_player(player),
                    ]
                    .concat()
                } else if dice_result >= 15 {
                    [
                        vec![Advancement::Strength, Advancement::Agility],
                        Advancement::secondary_skill_advancements_available_for_player(player),
                    ]
                    .concat()
                } else if dice_result >= 16 {
                    vec![
                        Advancement::MovementAllowance,
                        Advancement::Strength,
                        Advancement::Agility,
                        Advancement::PassingAbility,
                        Advancement::ArmourValue,
                    ]
                } else {
                    Vec::new()
                }
            }
        }
    }

    pub fn star_player_points_cost_for_player(&self, player: &Player) -> u32 {
        self.star_player_points_cost(player.advancements.len() + 1, &player.version)
    }

    pub fn star_player_points_cost(&self, advancement_number: usize, version: &Version) -> u32 {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => 0,
            Version::V5 => v5::star_player_points_cost(self, advancement_number),
            Version::V5S3 => v5s3::star_player_points_cost(self, advancement_number),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Advancement {
    ChosenSkill(Skill),
    RandomSkill(Skill),
    MovementAllowance,
    Strength,
    Agility,
    PassingAbility,
    ArmourValue,
}

impl TypeName for Advancement {}

impl TranslatedName for Advancement {
    fn name(&self, lang_id: &str) -> String {
        match self {
            Advancement::ChosenSkill(skill) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "ChosenSkill",
                &HashMap::from([(Cow::from("skill"), skill.name(lang_id).into())]),
            ),
            Advancement::RandomSkill(skill) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "RandomSkill",
                &HashMap::from([(Cow::from("skill"), skill.name(lang_id).into())]),
            ),
            _ => LOCALES.lookup(&language_from(lang_id), &*self.type_name()),
        }
    }
}

impl Advancement {
    pub const MAXIMUM: usize = 6;

    pub fn primary_skill_advancements_available_for_player(player: &Player) -> Vec<Self> {
        let mut advancements = Vec::new();

        for skill in Skill::primary_list_available_for_player(player) {
            advancements.push(Advancement::ChosenSkill(skill));
        }

        advancements
    }

    pub fn secondary_skill_advancements_available_for_player(player: &Player) -> Vec<Self> {
        let mut advancements = Vec::new();

        for skill in Skill::secondary_list_available_for_player(player) {
            advancements.push(Advancement::ChosenSkill(skill));
        }

        advancements
    }

    pub fn added_value_for_player(&self, player: &Player) -> Result<u32, Error> {
        match self {
            Advancement::ChosenSkill(skill) => {
                if skill.is_primary_for_player(player) {
                    Ok(20000)
                } else if skill.is_secondary_for_player(player) {
                    Ok(40000)
                } else {
                    Err(Error::SkillNotAvailableForPlayer)
                }
            }

            Advancement::RandomSkill(skill) => {
                if skill.is_primary_for_player(player) {
                    Ok(10000)
                } else if skill.is_secondary_for_player(player) {
                    Ok(20000)
                } else {
                    Err(Error::SkillNotAvailableForPlayer)
                }
            }

            Advancement::MovementAllowance => Ok(20000),
            Advancement::Strength => Ok(80000),
            Advancement::Agility => Ok(40000),
            Advancement::PassingAbility => Ok(20000),
            Advancement::ArmourValue => Ok(10000),
        }
    }
}
