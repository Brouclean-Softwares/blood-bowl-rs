use crate::characteristics::Characteristic;
use crate::positions::{Position, PositionDefinition};
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};
use std::collections::HashMap;

pub(crate) fn mapping_with_previous_version(
    roster_in_previous_version: &Roster,
    position_in_previous_version: &Position,
) -> Option<Position> {
    match (roster_in_previous_version, position_in_previous_version) {
        (_, position_in_previous_version) => Some(position_in_previous_version.clone()),
    }
}

pub fn positon_definition_from(roster: &Roster, position: &Position) -> Option<PositionDefinition> {
    match (roster, position) {
        //*************************************************************************************
        // Wood ELf
        //*************************************************************************************
        (Roster::WoodElf, Position::WoodElfLineman) => Some(PositionDefinition {
            maximum_quantity: 16,
            cost: 65000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::Strength],
            is_big_man: false,
        }),
        (Roster::WoodElf, Position::Thrower) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 85000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 2),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Pass, Skill::SafePairOfHands],
            primary_skill_categories: vec![
                SkillCategory::General,
                SkillCategory::Agility,
                SkillCategory::Pass,
            ],
            secondary_skill_categories: vec![SkillCategory::Strength],
            is_big_man: false,
        }),
        (Roster::WoodElf, Position::Catcher) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 90000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Catch, Skill::Dodge, Skill::Sprint],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::Strength, SkillCategory::Pass],
            is_big_man: false,
        }),
        (Roster::WoodElf, Position::Wardancer) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 130000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Block, Skill::Dodge, Skill::Leap],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::Strength, SkillCategory::Pass],
            is_big_man: false,
        }),
        (Roster::WoodElf, Position::LorenForestTreeman) => Some(PositionDefinition {
            maximum_quantity: 1,
            cost: 120000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 2),
                (Characteristic::Strength, 6),
                (Characteristic::Agility, 5),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 11),
            ]),
            skills: vec![
                Skill::Loner(4),
                Skill::MightyBlow(1),
                Skill::StandFirm,
                Skill::StrongArm,
                Skill::TakeRoots,
                Skill::ThickSkull,
                Skill::ThrowTeamMate,
            ],
            primary_skill_categories: vec![SkillCategory::Strength],
            secondary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            is_big_man: true,
        }),

        // Others
        _ => None,
    }
}
