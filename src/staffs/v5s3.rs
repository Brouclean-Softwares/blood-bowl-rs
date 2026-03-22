use crate::positions::{Position, PositionDefinition};
use crate::rosters::Roster;
use crate::staffs::FamousCoachingStaff;
use crate::versions::Version;

const VERSION: Version = Version::V5S3;

pub(crate) fn famous_coaching_staff_list() -> Vec<FamousCoachingStaff> {
    vec![FamousCoachingStaff::JosefBugman]
}

pub(crate) fn famous_coaching_staff_maximum_for_roster(
    famous_coaching_staff: &FamousCoachingStaff,
    roster: &Roster,
) -> usize {
    match (famous_coaching_staff, roster, roster.definition(VERSION)) {
        (FamousCoachingStaff::JosefBugman, _, _) => 1,
        _ => 0,
    }
}

pub(crate) fn famous_coaching_staff_price(famous_coaching_staff: &FamousCoachingStaff) -> u32 {
    match famous_coaching_staff {
        FamousCoachingStaff::JosefBugman => 100000,
        _ => 0,
    }
}

pub(crate) fn staff_position(_famous_coaching_staff: &FamousCoachingStaff) -> Option<Position> {
    None
}

pub(crate) fn staff_position_definition(_position: &Position) -> Option<PositionDefinition> {
    None
}
