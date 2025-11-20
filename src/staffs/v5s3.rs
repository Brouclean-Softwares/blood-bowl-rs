use crate::positions::{Position, PositionDefinition};
use crate::rosters::Roster;
use crate::staffs::FamousCoachingStaff;

pub(crate) fn famous_coaching_staff_list() -> Vec<FamousCoachingStaff> {
    Vec::new()
}

pub(crate) fn famous_coaching_staff_maximum_for_roster(
    _famous_coaching_staff: &FamousCoachingStaff,
    _roster: &Roster,
) -> usize {
    0
}

pub(crate) fn famous_coaching_staff_price(_famous_coaching_staff: &FamousCoachingStaff) -> u32 {
    0
}

pub(crate) fn staff_position_definition(_position: &Position) -> Option<PositionDefinition> {
    None
}
