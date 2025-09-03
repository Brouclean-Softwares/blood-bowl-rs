use std::collections::HashMap;
use crate::characteristics::Characteristic;
use crate::positions::{Position, PositionDefinition};
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};
use crate::versions::Version;

pub fn positon_definition_from(position: Position) -> Option<PositionDefinition> {
    match position {
        Position::EagleWarriorLinewoman(Roster::Amazon(Version::V5)) => Some(PositionDefinition {
            maximum_quantity: 16,
            cost: 50000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Dodge],
            primary_skill_categories: vec![SkillCategory::General],
            secondary_skill_categories: vec![SkillCategory::Agility, SkillCategory::Strength],
            is_big_man: false,
        }),
        Position::PythonWarriorThrower(Roster::Amazon(Version::V5)) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 80000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Dodge, Skill::OnTheBall, Skill::Pass, Skill::SafePass],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Pass],
            secondary_skill_categories: vec![SkillCategory::Agility, SkillCategory::Strength],
            is_big_man: false,
        }),
        Position::PiranhaWarriorBlitzer(Roster::Amazon(Version::V5)) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 90000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Dodge, Skill::HitAndRun, Skill::JumpUp],
            primary_skill_categories: vec![SkillCategory::Agility, SkillCategory::General],
            secondary_skill_categories: vec![SkillCategory::Strength],
            is_big_man: false,
        }),
        Position::JaguarWarriorBlocker(Roster::Amazon(Version::V5)) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 110000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![Skill::Defensive, Skill::Dodge],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Strength],
            secondary_skill_categories: vec![SkillCategory::Agility],
            is_big_man: false,
        }),

        _ => None,
    }
}