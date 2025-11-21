use crate::characteristics::Characteristic;
use crate::players::PlayerType;
use crate::positions::{Position, PositionDefinition};
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};
use crate::staffs::FamousCoachingStaff;
use crate::versions::Version;
use std::collections::HashMap;

const VERSION: Version = Version::V5S3;

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
        // Amazon
        //*************************************************************************************
        (Roster::Amazon, Position::EagleWarriorLinewoman) => Some(PositionDefinition {
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
        (Roster::Amazon, Position::PythonWarriorThrower) => Some(PositionDefinition {
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
        (Roster::Amazon, Position::PiranhaWarriorBlitzer) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 90000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Dodge, Skill::HitAndRun, Skill::JumpUp],
            primary_skill_categories: vec![SkillCategory::Agility, SkillCategory::General],
            secondary_skill_categories: vec![SkillCategory::Strength],
            is_big_man: false,
        }),
        (Roster::Amazon, Position::JaguarWarriorBlocker) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 110000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![Skill::Defensive, Skill::Dodge],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Strength],
            secondary_skill_categories: vec![SkillCategory::Agility],
            is_big_man: false,
        }),

        //*************************************************************************************
        // Bretonnian
        //*************************************************************************************
        (Roster::Bretonnian, Position::Squires) => Some(PositionDefinition {
            maximum_quantity: 16,
            cost: 50000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Wrestle],
            primary_skill_categories: vec![SkillCategory::General],
            secondary_skill_categories: vec![SkillCategory::Agility, SkillCategory::Strength],
            is_big_man: false,
        }),
        (Roster::Bretonnian, Position::KnightCatcher) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 85000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![Skill::Dauntless, Skill::NervesOfSteel, Skill::Catch],
            primary_skill_categories: vec![SkillCategory::Agility, SkillCategory::General],
            secondary_skill_categories: vec![SkillCategory::Strength],
            is_big_man: false,
        }),
        (Roster::Bretonnian, Position::KnightThrower) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 80000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![Skill::Dauntless, Skill::NervesOfSteel, Skill::Pass],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Pass],
            secondary_skill_categories: vec![SkillCategory::Agility, SkillCategory::Strength],
            is_big_man: false,
        }),
        (Roster::Bretonnian, Position::GrailKnight) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 95000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![Skill::Block, Skill::Dauntless, Skill::SteadyFooting],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Strength],
            secondary_skill_categories: vec![SkillCategory::Agility],
            is_big_man: false,
        }),

        //*************************************************************************************
        // Chaos Chosen
        //*************************************************************************************
        (Roster::ChaosChosen, Position::BeastmanRunnerLineman) => Some(PositionDefinition {
            maximum_quantity: 16,
            cost: 55000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![Skill::Horns, Skill::ThickSkull],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Mutation],
            secondary_skill_categories: vec![
                SkillCategory::Agility,
                SkillCategory::Devious,
                SkillCategory::Pass,
                SkillCategory::Strength,
            ],
            is_big_man: false,
        }),
        (Roster::ChaosChosen, Position::ChosenBlocker) => Some(PositionDefinition {
            maximum_quantity: 4,
            cost: 100000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![Skill::ArmBar],
            primary_skill_categories: vec![
                SkillCategory::General,
                SkillCategory::Mutation,
                SkillCategory::Strength,
            ],
            secondary_skill_categories: vec![SkillCategory::Agility, SkillCategory::Devious],
            is_big_man: false,
        }),
        (Roster::ChaosChosen, Position::ChaosTroll) => Some(PositionDefinition {
            maximum_quantity: 1,
            cost: 115000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 4),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 5),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::AlwaysHungry,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::ProjectileVomit,
                Skill::ReallyStupid,
                Skill::Regeneration,
                Skill::ThrowTeamMate,
            ],
            primary_skill_categories: vec![SkillCategory::Mutation, SkillCategory::Strength],
            secondary_skill_categories: vec![
                SkillCategory::Agility,
                SkillCategory::General,
                SkillCategory::Pass,
            ],
            is_big_man: true,
        }),
        (Roster::ChaosChosen, Position::ChaosOgre) => Some(PositionDefinition {
            maximum_quantity: 1,
            cost: 140000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::BoneHead,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::ThickSkull,
                Skill::ThrowTeamMate,
            ],
            primary_skill_categories: vec![SkillCategory::Mutation, SkillCategory::Strength],
            secondary_skill_categories: vec![SkillCategory::Agility, SkillCategory::General],
            is_big_man: true,
        }),
        (Roster::ChaosChosen, Position::Minotaur) => Some(PositionDefinition {
            maximum_quantity: 1,
            cost: 150000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Frenzy,
                Skill::Horns,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::ThickSkull,
                Skill::UnchannelledFury,
            ],
            primary_skill_categories: vec![SkillCategory::Mutation, SkillCategory::Strength],
            secondary_skill_categories: vec![SkillCategory::Agility, SkillCategory::General],
            is_big_man: true,
        }),

        //*************************************************************************************
        // Dark Elf
        //*************************************************************************************
        (Roster::DarkElf, Position::DarkElfLineman) => Some(PositionDefinition {
            maximum_quantity: 16,
            cost: 65000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::Strength, SkillCategory::Devious],
            is_big_man: false,
        }),
        (Roster::DarkElf, Position::Runner) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 80000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::DumpOff, Skill::Punt],
            primary_skill_categories: vec![
                SkillCategory::General,
                SkillCategory::Agility,
                SkillCategory::Pass,
            ],
            secondary_skill_categories: vec![SkillCategory::Strength, SkillCategory::Devious],
            is_big_man: false,
        }),
        (Roster::DarkElf, Position::Assassin) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 90000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::HitAndRun, Skill::Shadowing, Skill::Stab],
            primary_skill_categories: vec![SkillCategory::Devious, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::General, SkillCategory::Strength],
            is_big_man: false,
        }),
        (Roster::DarkElf, Position::Blitzer) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 105000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![Skill::Block],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![
                SkillCategory::Devious,
                SkillCategory::Strength,
                SkillCategory::Pass,
            ],
            is_big_man: false,
        }),
        (Roster::DarkElf, Position::WitchElf) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 110000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Dodge, Skill::Frenzy, Skill::JumpUp],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![SkillCategory::Strength, SkillCategory::Devious],
            is_big_man: false,
        }),

        //*************************************************************************************
        // Human
        //*************************************************************************************
        (Roster::Human, Position::HumanLineman) => Some(PositionDefinition {
            maximum_quantity: 16,
            cost: 50000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![],
            primary_skill_categories: vec![SkillCategory::General],
            secondary_skill_categories: vec![
                SkillCategory::Strength,
                SkillCategory::Agility,
                SkillCategory::Devious,
            ],
            is_big_man: false,
        }),
        (Roster::Human, Position::HalflingHopeful) => Some(PositionDefinition {
            maximum_quantity: 3,
            cost: 30000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 7),
            ]),
            skills: vec![Skill::Dodge, Skill::RightStuff, Skill::Stunty],
            primary_skill_categories: vec![SkillCategory::Agility],
            secondary_skill_categories: vec![
                SkillCategory::General,
                SkillCategory::Strength,
                SkillCategory::Devious,
            ],
            is_big_man: false,
        }),
        (Roster::Human, Position::Catcher) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 75000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![Skill::Catch, Skill::Dodge],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            secondary_skill_categories: vec![
                SkillCategory::Strength,
                SkillCategory::Pass,
                SkillCategory::Devious,
            ],
            is_big_man: false,
        }),
        (Roster::Human, Position::Thrower) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 75000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![Skill::Pass, Skill::SureHands],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Pass],
            secondary_skill_categories: vec![
                SkillCategory::Strength,
                SkillCategory::Agility,
                SkillCategory::Devious,
            ],
            is_big_man: false,
        }),
        (Roster::Human, Position::Blitzer) => Some(PositionDefinition {
            maximum_quantity: 2,
            cost: 85000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![Skill::Block, Skill::Tackle],
            primary_skill_categories: vec![SkillCategory::General, SkillCategory::Strength],
            secondary_skill_categories: vec![SkillCategory::Agility, SkillCategory::Devious],
            is_big_man: false,
        }),
        (Roster::Human, Position::Ogre) => Some(PositionDefinition {
            maximum_quantity: 1,
            cost: 140000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::BoneHead,
                Skill::Loner(3),
                Skill::MightyBlow,
                Skill::ThickSkull,
                Skill::ThrowTeamMate,
            ],
            primary_skill_categories: vec![SkillCategory::Strength],
            secondary_skill_categories: vec![SkillCategory::General, SkillCategory::Agility],
            is_big_man: true,
        }),

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
                Skill::MightyBlow,
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
        (_, position) => match position.player_type(&VERSION) {
            PlayerType::Star | PlayerType::MegaStar => {
                crate::stars::star_player_position_definition(position, &VERSION)
            }
            PlayerType::FamousCoachingStaff => {
                FamousCoachingStaff::position_definition(position, &VERSION)
            }
            PlayerType::FromRoster | PlayerType::Journeyman => None,
        },
    }
}
