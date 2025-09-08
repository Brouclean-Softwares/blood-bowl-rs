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

pub(crate) fn roster_definition_from(roster: &Roster) -> Option<RosterDefinition> {
    match roster {
        Roster::Amazon => Some(RosterDefinition {
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
                Position::EagleWarriorLinewoman(Roster::Amazon),
                Position::PythonWarriorThrower(Roster::Amazon),
                Position::PiranhaWarriorBlitzer(Roster::Amazon),
                Position::JaguarWarriorBlocker(Roster::Amazon),
            ],
            maximum_big_men_quantity: 0,
            special_rules: vec![SpecialRule::LustrianSuperleague],
            dedicated_fans_information: DedicatedFansInformation {
                price: 10000,
                initial: 1,
                maximum: 6,
            },
        }),
        Roster::BlackOrc => None,
        Roster::ChaosChosen => None,
        Roster::ChaosDwarf => None,
        Roster::ChaosRenegade => None,
        Roster::DarkElf => None,
        Roster::Dwarf => None,
        Roster::ElvenUnion => None,
        Roster::Gnome => None,
        Roster::Goblin => None,
        Roster::Halfling => None,
        Roster::HighElf => None,
        Roster::Human => None,
        Roster::ImperialNobility => None,
        Roster::Khorne => None,
        Roster::Lizardmen => None,
        Roster::NecromanticHorror => None,
        Roster::Norse => None,
        Roster::Nurgle => None,
        Roster::Ogre => None,
        Roster::OldWorldAlliance => None,
        Roster::Orc => None,
        Roster::ShamblingUndead => None,
        Roster::Skaven => None,
        Roster::Snotling => None,
        Roster::TombKings => None,
        Roster::UnderworldDenizens => None,
        Roster::Vampire => None,
        Roster::WoodElf => Some(RosterDefinition {
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
                Position::WoodElfLineman(Roster::WoodElf),
                Position::Thrower(Roster::WoodElf),
                Position::Catcher(Roster::WoodElf),
                Position::Wardancer(Roster::WoodElf),
                Position::LorenForestTreeman(Roster::WoodElf),
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
