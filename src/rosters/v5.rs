use crate::errors::Error;
use crate::positions::Position;
use crate::rosters::{
    DedicatedFansInformation, Roster, RosterDefinition, SpecialRule, Staff, StaffInformation,
};
use crate::versions::Version;

const CHEERLEADER_DEFINITION: StaffInformation = StaffInformation {
    staff: Staff::Cheerleader,
    price: 10000,
    maximum: 12,
};

const ASSISTANT_COACH_DEFINITION: StaffInformation = StaffInformation {
    staff: Staff::AssistantCoach,
    price: 10000,
    maximum: 6,
};

const APOTHECARY_DEFINITION: StaffInformation = StaffInformation {
    staff: Staff::Apothecary,
    price: 50000,
    maximum: 1,
};

const REROLL_50_DEFINITION: StaffInformation = StaffInformation {
    staff: Staff::ReRoll,
    price: 50000,
    maximum: 8,
};

const REROLL_60_DEFINITION: StaffInformation = StaffInformation {
    staff: Staff::ReRoll,
    price: 60000,
    maximum: 8,
};

const DEDICATED_FANS_DEFINITION: DedicatedFansInformation = DedicatedFansInformation {
    price: 10000,
    initial: 1,
    maximum: 6,
};

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
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_60_DEFINITION,
            ],
            positions: vec![
                Position::EagleWarriorLinewoman,
                Position::PythonWarriorThrower,
                Position::PiranhaWarriorBlitzer,
                Position::JaguarWarriorBlocker,
            ],
            maximum_big_men_quantity: 0,
            special_rules: vec![SpecialRule::LustrianSuperleague],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::BlackOrc => Err(Error::RosterNotExist),

        Roster::ChaosChosen => Ok(RosterDefinition {
            version: Version::V5,
            tier: 2,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_60_DEFINITION,
            ],
            positions: vec![
                Position::BeastmanRunnerLineman,
                Position::ChosenBlocker,
                Position::ChaosTroll,
                Position::ChaosOgre,
                Position::Minotaur,
            ],
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::FavouredOf],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

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

        Roster::Orc => Ok(RosterDefinition {
            version: Version::V5,
            tier: 2,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_60_DEFINITION,
            ],
            positions: vec![
                Position::OrcLineman,
                Position::Thrower,
                Position::Blitzer,
                Position::BigUn,
                Position::Goblin,
                Position::UntrainedTroll,
            ],
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::BadlandsBrawl],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::ShamblingUndead => Err(Error::RosterNotExist),
        Roster::Skaven => Err(Error::RosterNotExist),

        Roster::Snotling => Ok(RosterDefinition {
            version: Version::V5,
            tier: 3,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_60_DEFINITION,
            ],
            positions: vec![
                Position::SnotlingLineman,
                Position::FungusFlinga,
                Position::FunHoppa,
                Position::StiltyRunna,
                Position::PumpWagon,
                Position::TrainedTroll,
            ],
            maximum_big_men_quantity: 4,
            special_rules: vec![
                SpecialRule::BriberyAndCorruption,
                SpecialRule::UnderworldChallenge,
                SpecialRule::LowCostLinemen,
            ],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::TombKings => Err(Error::RosterNotExist),
        Roster::UnderworldDenizens => Err(Error::RosterNotExist),
        Roster::Vampire => Err(Error::RosterNotExist),

        Roster::WoodElf => Ok(RosterDefinition {
            version: Version::V5,
            tier: 1,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_50_DEFINITION,
            ],
            positions: vec![
                Position::WoodElfLineman,
                Position::Thrower,
                Position::Catcher,
                Position::Wardancer,
                Position::LorenForestTreeman,
            ],
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::ElvenKingdomsLeague],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),
    }
}
