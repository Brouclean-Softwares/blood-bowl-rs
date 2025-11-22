use crate::positions::Position;
use crate::rosters::{
    DedicatedFansInformation, Roster, RosterDefinition, SpecialLeague, SpecialRule,
};
use crate::staffs::StaffInformation;
use crate::versions::Version;

const VERSION: Version = Version::V5S3;

pub(crate) fn roster_list() -> Vec<Roster> {
    vec![
        Roster::Amazon,
        Roster::BlackOrc,
        Roster::Bretonnian,
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

pub(crate) fn mapping_with_previous_version(roster_in_previous_version: &Roster) -> Option<Roster> {
    match roster_in_previous_version {
        roster_in_previous_version => Some(roster_in_previous_version.clone()),
    }
}

pub(crate) fn roster_definition_from(roster: &Roster) -> Option<RosterDefinition> {
    if !roster_list().contains(roster) {
        return None;
    }

    match roster {
        Roster::Amazon => Some(RosterDefinition {
            version: VERSION,
            tier: 1,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::EagleWarriorLinewoman,
                Position::PythonWarriorThrower,
                Position::PiranhaWarriorBlitzer,
                Position::JaguarWarriorBlocker,
            ],
            journeyman_position: Position::EagleWarriorLinewoman,
            maximum_big_men_quantity: 0,
            special_leagues: vec![SpecialLeague::LustrianSuperleague],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::BlackOrc => Some(RosterDefinition {
            version: VERSION,
            tier: 3,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::GoblinBruiserLineman,
                Position::BlackOrc,
                Position::TrainedTroll,
            ],
            journeyman_position: Position::GoblinBruiserLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::BadlandsBrawl],
            special_rules: vec![
                SpecialRule::BrawlingBrutes,
                SpecialRule::BriberyAndCorruption,
            ],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Bretonnian => Some(RosterDefinition {
            version: VERSION,
            tier: 2,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::Squires,
                Position::KnightCatcher,
                Position::KnightThrower,
                Position::GrailKnight,
            ],
            journeyman_position: Position::Squires,
            maximum_big_men_quantity: 0,
            special_leagues: vec![SpecialLeague::OldWorldClassic],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::ChaosChosen => Some(RosterDefinition {
            version: VERSION,
            tier: 3,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_50,
            ],
            positions: vec![
                Position::BeastmanRunnerLineman,
                Position::ChosenBlocker,
                Position::ChaosTroll,
                Position::ChaosOgre,
                Position::Minotaur,
            ],
            journeyman_position: Position::BeastmanRunnerLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::ChaosClash],
            special_rules: vec![SpecialRule::FavouredOf],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::ChaosDwarf => Some(RosterDefinition {
            version: VERSION,
            tier: 1,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_70,
            ],
            positions: vec![
                Position::HobgoblinLineman,
                Position::HobgoblinSneakyStabba,
                Position::ChaosDwarfBlocker,
                Position::ChaosDwarfFlamesmith,
                Position::BullCentaurBlitzer,
                Position::Minotaur,
            ],
            journeyman_position: Position::HobgoblinLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::BadlandsBrawl, SpecialLeague::ChaosClash],
            special_rules: vec![SpecialRule::FavouredOfHashut],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::ChaosRenegade => Some(RosterDefinition {
            version: VERSION,
            tier: 3,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_70,
            ],
            positions: vec![
                Position::RenegadeHumanLineman,
                Position::RenegadeGoblin,
                Position::RenegadeOrc,
                Position::RenegadeSkaven,
                Position::RenegadeDarkElf,
                Position::RenegadeHumanThrower,
                Position::RenegadeTroll,
                Position::RenegadeOgre,
                Position::RenegadeMinotaur,
                Position::RenegadeRatOgre,
            ],
            journeyman_position: Position::RenegadeHumanLineman,
            maximum_big_men_quantity: 3,
            special_leagues: vec![SpecialLeague::ChaosClash],
            special_rules: vec![SpecialRule::FavouredOf],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::DarkElf => Some(RosterDefinition {
            version: VERSION,
            tier: 1,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_50,
            ],
            positions: vec![
                Position::DarkElfLineman,
                Position::Runner,
                Position::Assassin,
                Position::Blitzer,
                Position::WitchElf,
            ],
            journeyman_position: Position::DarkElfLineman,
            maximum_big_men_quantity: 0,
            special_leagues: vec![SpecialLeague::ElvenKingdomsLeague],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Dwarf => Some(RosterDefinition {
            version: VERSION,
            tier: 1,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::DwarfBlockerLineman,
                Position::Runner,
                Position::Blitzer,
                Position::TrollSlayer,
                Position::Deathroller,
            ],
            journeyman_position: Position::DwarfBlockerLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::WorldsEdgeSuperleague],
            special_rules: vec![
                SpecialRule::BrawlingBrutes,
                SpecialRule::BriberyAndCorruption,
            ],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Human => Some(RosterDefinition {
            version: VERSION,
            tier: 2,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_50,
            ],
            positions: vec![
                Position::HumanLineman,
                Position::HalflingHopeful,
                Position::Catcher,
                Position::Thrower,
                Position::Blitzer,
                Position::Ogre,
            ],
            journeyman_position: Position::HumanLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::OldWorldClassic],
            special_rules: vec![SpecialRule::TeamCaptain],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::WoodElf => Some(RosterDefinition {
            version: VERSION,
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
