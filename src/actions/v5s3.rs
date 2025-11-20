use crate::actions::Success;
use crate::rosters::{RosterDefinition, SpecialRule};

pub(crate) fn star_player_points_for_success(
    success: &Success,
    roster_definition: &RosterDefinition,
) -> u32 {
    match success {
        Success::PassingCompletion => 1,
        Success::ThrowingCompletion => 1,
        Success::Deflection => 0,
        Success::Interception => 2,
        Success::Casualty => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::BrawlingBrutes)
            {
                3
            } else {
                2
            }
        }
        Success::Touchdown => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::BrawlingBrutes)
            {
                2
            } else {
                3
            }
        }
        Success::MostValuablePlayer => 4,
        Success::StarPlayerPoint => 1,
    }
}
