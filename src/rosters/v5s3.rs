use crate::positions::Position;
use crate::rosters::{DedicatedFansInformation, Roster, RosterDefinition, SpecialLeague};
use crate::staffs::StaffInformation;
use crate::versions::Version;

pub(crate) fn roster_list() -> Vec<Roster> {
    super::v5::roster_list()
}

pub(crate) fn mapping_with_previous_version(roster_in_previous_version: &Roster) -> Option<Roster> {
    match roster_in_previous_version {
        roster_in_previous_version => Some(roster_in_previous_version.clone()),
    }
}

pub(crate) fn roster_definition_from(roster: &Roster) -> Option<RosterDefinition> {
    match roster {
        Roster::WoodElf => Some(RosterDefinition {
            version: Version::V5S3,
            tier: 1,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_50,
            ],
            positions: vec![
                Position::WoodElfLineman,
                Position::Thrower,
                Position::Catcher,
                Position::Wardancer,
                Position::LorenForestTreeman,
            ],
            journeyman_position: Position::WoodElfLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![
                SpecialLeague::ElvenKingdomsLeague,
                SpecialLeague::WoodlandLeague,
            ],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        _ => None,
    }
}
