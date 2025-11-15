use crate::positions::PositionDefinition;
use crate::rosters::{Roster, RosterDefinition, SpecialRule};
use crate::staffs::FamousCoachingStaff;

pub(crate) fn staff_player_default_definition() -> PositionDefinition {
    PositionDefinition {
        maximum_quantity: 1,
        cost: 0,
        characteristics: Default::default(),
        skills: Vec::new(),
        primary_skill_categories: Vec::new(),
        secondary_skill_categories: Vec::new(),
        is_big_man: false,
    }
}

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

pub(crate) fn famous_coaching_staff_maximum_for_team(
    famous_coaching_staff: &FamousCoachingStaff,
    roster: &Roster,
    roster_definition: &Option<RosterDefinition>,
) -> usize {
    match (famous_coaching_staff, roster, roster_definition) {
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
