use std::collections::HashMap;
use crate::positions::Position;
use crate::rosters::{Roster, RosterDefinition, SpecialRule};
use crate::versions::Version;

pub fn from(roster: Roster) -> Option<RosterDefinition> {
    match roster {
        Roster::Amazon(Version::V5) => Some(RosterDefinition {
            tier: 1,
            staff_prices: HashMap::from([]),
            positions: vec![
                Position::EagleWarriorLinewoman(Roster::Amazon(Version::V5)),
                Position::PythonWarriorThrower(Roster::Amazon(Version::V5)),
                Position::PiranhaWarriorBlitzer(Roster::Amazon(Version::V5)),
                Position::JaguarWarriorBlocker(Roster::Amazon(Version::V5)),
            ],
            maximum_big_men_quantity: 0,
            special_rule: SpecialRule::LustrianSuperleague,
        }),
        Roster::BlackOrc(Version::V5) => None,
        Roster::ChaosChosen(Version::V5) => None,
        Roster::ChaosDwarf(Version::V5) => None,
        Roster::ChaosRenegade(Version::V5) => None,
        Roster::DarkElf(Version::V5) => None,
        Roster::Dwarf(Version::V5) => None,
        Roster::ElvenUnion(Version::V5) => None,
        Roster::Gnome(Version::V5) => None,
        Roster::Goblin(Version::V5) => None,
        Roster::Halfling(Version::V5) => None,
        Roster::HighElf(Version::V5) => None,
        Roster::Human(Version::V5) => None,
        Roster::ImperialNobility(Version::V5) => None,
        Roster::Khorne(Version::V5) => None,
        Roster::Lizardmen(Version::V5) => None,
        Roster::NecromanticHorror(Version::V5) => None,
        Roster::Norse(Version::V5) => None,
        Roster::Nurgle(Version::V5) => None,
        Roster::Ogre(Version::V5) => None,
        Roster::OldWorldAliance(Version::V5) => None,
        Roster::Orc(Version::V5) => None,
        Roster::ShamblingUndead(Version::V5) => None,
        Roster::Skaven(Version::V5) => None,
        Roster::Snotling(Version::V5) => None,
        Roster::TombKings(Version::V5) => None,
        Roster::UnderworldDenizens(Version::V5) => None,
        Roster::Vampire(Version::V5) => None,
        Roster::WoodElf(Version::V5) => None,

        _ => None,
    }
}