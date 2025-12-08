use crate::characteristics::Characteristic;
use crate::positions::{Position, PositionDefinition};
use crate::rosters::{Roster, SpecialRule};
use crate::skills::Skill;
use crate::staffs::FamousCoachingStaff;
use crate::versions::Version;
use std::collections::HashMap;

const VERSION: Version = Version::V5;

pub(crate) fn famous_coaching_staff_list() -> Vec<FamousCoachingStaff> {
    vec![
        FamousCoachingStaff::AyleenAndar,
        FamousCoachingStaff::FinkDaFixer,
        FamousCoachingStaff::GalandrilSilverwater,
        FamousCoachingStaff::JosefBugman,
        FamousCoachingStaff::KariColdsteel,
        FamousCoachingStaff::KrotShockwhisker,
        FamousCoachingStaff::MungoSpinecracker,
        FamousCoachingStaff::PapaSkullbones,
        FamousCoachingStaff::ProfessorFronkelheim,
        FamousCoachingStaff::SchielundScharlitan,
    ]
}

pub(crate) fn famous_coaching_staff_maximum_for_roster(
    famous_coaching_staff: &FamousCoachingStaff,
    roster: &Roster,
) -> usize {
    match (famous_coaching_staff, roster, roster.definition(VERSION)) {
        (FamousCoachingStaff::AyleenAndar, _, _) => 1,
        (FamousCoachingStaff::FinkDaFixer, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::BadlandsBrawl)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::UnderworldChallenge)
            {
                1
            } else {
                0
            }
        }
        (FamousCoachingStaff::GalandrilSilverwater, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::ElvenKingdomsLeague)
            {
                1
            } else {
                0
            }
        }
        (FamousCoachingStaff::JosefBugman, _, _) => 1,
        (FamousCoachingStaff::KariColdsteel, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::ElvenKingdomsLeague)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::LustrianSuperleague)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::OldWorldClassic)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::WorldsEdgeSuperleague)
            {
                1
            } else {
                0
            }
        }
        (FamousCoachingStaff::KrotShockwhisker, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::UnderworldChallenge)
            {
                1
            } else {
                0
            }
        }
        (FamousCoachingStaff::MungoSpinecracker, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::BadlandsBrawl)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::OldWorldClassic)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::UnderworldChallenge)
            {
                1
            } else {
                0
            }
        }
        (FamousCoachingStaff::PapaSkullbones, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::FavouredOf)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::UnderworldChallenge)
            {
                1
            } else {
                0
            }
        }
        (FamousCoachingStaff::ProfessorFronkelheim, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::SylvanianSpotlight)
            {
                1
            } else {
                0
            }
        }
        (FamousCoachingStaff::SchielundScharlitan, _, Some(_)) => 1,

        _ => 0,
    }
}

pub(crate) fn famous_coaching_staff_price(famous_coaching_staff: &FamousCoachingStaff) -> u32 {
    match famous_coaching_staff {
        FamousCoachingStaff::AyleenAndar => 100000,
        FamousCoachingStaff::FinkDaFixer => 90000,
        FamousCoachingStaff::GalandrilSilverwater => 40000,
        FamousCoachingStaff::JosefBugman => 100000,
        FamousCoachingStaff::KariColdsteel => 50000,
        FamousCoachingStaff::KrotShockwhisker => 70000,
        FamousCoachingStaff::MungoSpinecracker => 80000,
        FamousCoachingStaff::PapaSkullbones => 80000,
        FamousCoachingStaff::ProfessorFronkelheim => 130000,
        FamousCoachingStaff::SchielundScharlitan => 90000,
    }
}

pub(crate) fn staff_position_definition(position: &Position) -> Option<PositionDefinition> {
    match position {
        Position::JosefBugman => Some(PositionDefinition {
            keywords: Vec::new(),
            maximum_quantity: 1,
            cost: 0,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Loner(5),
                Skill::Tackle,
                Skill::ThickSkull,
                Skill::Wrestle,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::KariColdsteel => Some(PositionDefinition {
            keywords: Vec::new(),
            maximum_quantity: 1,
            cost: 0,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Dauntless,
                Skill::Frenzy,
                Skill::Loner(4),
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        _ => None,
    }
}
