use crate::characteristics::Characteristic;
use crate::errors::Error;
use crate::positions::{Position, PositionDefinition};
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};
use std::collections::HashMap;

pub fn positon_definition_from(
    roster: Roster,
    position: Position,
) -> Result<PositionDefinition, Error> {
    match (roster, position) {
        // Amazon
        (Roster::Amazon, Position::EagleWarriorLinewoman) => Ok(PositionDefinition {
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
        (Roster::Amazon, Position::PythonWarriorThrower) => Ok(PositionDefinition {
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
        (Roster::Amazon, Position::PiranhaWarriorBlitzer) => Ok(PositionDefinition {
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
        (Roster::Amazon, Position::JaguarWarriorBlocker) => Ok(PositionDefinition {
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

        // Wood ELf
        (Roster::WoodElf, Position::WoodElfLineman) => Ok(PositionDefinition {
            maximum_quantity: 12,
            cost: 70000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::Strength],
            is_big_man: false,
        }),
        (Roster::WoodElf, Position::Thrower) => Ok(PositionDefinition {
            maximum_quantity: 2,
            cost: 95000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 2),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Pass],
            primary_skill_categories: vec![
                SkillCategory::General,
                SkillCategory::Agility,
                SkillCategory::Pass,
            ],
            secondary_skill_categories: vec![SkillCategory::Strength],
            is_big_man: false,
        }),
        (Roster::WoodElf, Position::Catcher) => Ok(PositionDefinition {
            maximum_quantity: 4,
            cost: 90000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Catch, Skill::Dodge],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::Strength, SkillCategory::Pass],
            is_big_man: false,
        }),
        (Roster::WoodElf, Position::Wardancer) => Ok(PositionDefinition {
            maximum_quantity: 2,
            cost: 125000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Block, Skill::Dodge, Skill::Leap],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::Strength, SkillCategory::Pass],
            is_big_man: false,
        }),
        (Roster::WoodElf, Position::LorenForestTreeman) => Ok(PositionDefinition {
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
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::Strength],
            is_big_man: true,
        }),

        // Others
        _ => Err(Error::PositionNotDefined),
    }
}
