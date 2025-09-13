use crate::errors::Error;
use crate::positions::Position;
use crate::rosters::{
    DedicatedFansInformation, Roster, RosterDefinition, SpecialRule, Staff, StaffInformation,
};
use crate::versions::Version;
use std::collections::HashMap;

pub(crate) fn roster_list() -> Vec<Roster> {
    vec![
        Roster::Amazon,
        Roster::BlackOrc,
        Roster::ChaosChosen,
        Roster::ChaosDwarf,
        Roster::ChaosRenegade,
        Roster::DarkElf,
        Roster::Dwarf,
        Roster::ElvenUnion,
        Roster::Gnome,
        Roster::Goblin,
        Roster::Halfling,
        Roster::HighElf,
        Roster::Human,
        Roster::ImperialNobility,
        Roster::Khorne,
        Roster::Lizardmen,
        Roster::NecromanticHorror,
        Roster::Norse,
        Roster::Nurgle,
        Roster::Ogre,
        Roster::OldWorldAlliance,
        Roster::Orc,
        Roster::ShamblingUndead,
        Roster::Skaven,
        Roster::Snotling,
        Roster::TombKings,
        Roster::UnderworldDenizens,
        Roster::Vampire,
        Roster::WoodElf,
    ]
}

pub(crate) fn roster_definition_from(roster: &Roster) -> Result<RosterDefinition, Error> {
    match roster {
        Roster::Amazon => Ok(RosterDefinition {
            version: Version::V5,
            tier: 1,
            staff_information: HashMap::from([
                (
                    Staff::Cheerleader,
                    StaffInformation {
                        price: 10000,
                        maximum: 12,
                    },
                ),
                (
                    Staff::AssistantCoach,
                    StaffInformation {
                        price: 10000,
                        maximum: 6,
                    },
                ),
                (
                    Staff::Apothecary,
                    StaffInformation {
                        price: 50000,
                        maximum: 1,
                    },
                ),
                (
                    Staff::ReRoll,
                    StaffInformation {
                        price: 60000,
                        maximum: 8,
                    },
                ),
            ]),
            positions: vec![
                Position::EagleWarriorLinewoman,
                Position::PythonWarriorThrower,
                Position::PiranhaWarriorBlitzer,
                Position::JaguarWarriorBlocker,
            ],
            maximum_big_men_quantity: 0,
            special_rules: vec![SpecialRule::LustrianSuperleague],
            dedicated_fans_information: DedicatedFansInformation {
                price: 10000,
                initial: 1,
                maximum: 6,
            },
        }),

        Roster::BlackOrc => Err(Error::RosterNotExist),
        Roster::ChaosChosen => Err(Error::RosterNotExist),
        Roster::ChaosDwarf => Err(Error::RosterNotExist),
        Roster::ChaosRenegade => Err(Error::RosterNotExist),
        Roster::DarkElf => Err(Error::RosterNotExist),
        Roster::Dwarf => Err(Error::RosterNotExist),
        Roster::ElvenUnion => Err(Error::RosterNotExist),
        Roster::Gnome => Err(Error::RosterNotExist),
        Roster::Goblin => Err(Error::RosterNotExist),
        Roster::Halfling => Err(Error::RosterNotExist),
        Roster::HighElf => Err(Error::RosterNotExist),
        Roster::Human => Err(Error::RosterNotExist),
        Roster::ImperialNobility => Err(Error::RosterNotExist),
        Roster::Khorne => Err(Error::RosterNotExist),
        Roster::Lizardmen => Err(Error::RosterNotExist),
        Roster::NecromanticHorror => Err(Error::RosterNotExist),
        Roster::Norse => Err(Error::RosterNotExist),
        Roster::Nurgle => Err(Error::RosterNotExist),
        Roster::Ogre => Err(Error::RosterNotExist),
        Roster::OldWorldAlliance => Err(Error::RosterNotExist),
        Roster::Orc => Err(Error::RosterNotExist),
        Roster::ShamblingUndead => Err(Error::RosterNotExist),
        Roster::Skaven => Err(Error::RosterNotExist),
        Roster::Snotling => Err(Error::RosterNotExist),
        Roster::TombKings => Err(Error::RosterNotExist),
        Roster::UnderworldDenizens => Err(Error::RosterNotExist),
        Roster::Vampire => Err(Error::RosterNotExist),

        Roster::WoodElf => Ok(RosterDefinition {
            version: Version::V5,
            tier: 1,
            staff_information: HashMap::from([
                (
                    Staff::Cheerleader,
                    StaffInformation {
                        price: 10000,
                        maximum: 12,
                    },
                ),
                (
                    Staff::AssistantCoach,
                    StaffInformation {
                        price: 10000,
                        maximum: 6,
                    },
                ),
                (
                    Staff::Apothecary,
                    StaffInformation {
                        price: 50000,
                        maximum: 1,
                    },
                ),
                (
                    Staff::ReRoll,
                    StaffInformation {
                        price: 50000,
                        maximum: 8,
                    },
                ),
            ]),
            positions: vec![
                Position::WoodElfLineman,
                Position::Thrower,
                Position::Catcher,
                Position::Wardancer,
                Position::LorenForestTreeman,
            ],
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::ElvenKingdomsLeague],
            dedicated_fans_information: DedicatedFansInformation {
                price: 10000,
                initial: 1,
                maximum: 6,
            },
        }),
    }
}
