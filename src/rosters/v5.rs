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

const REROLL_70_DEFINITION: StaffInformation = StaffInformation {
    staff: Staff::ReRoll,
    price: 70000,
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

pub(crate) fn roster_definition_from(roster: &Roster) -> Option<RosterDefinition> {
    match roster {
        Roster::Amazon => Some(RosterDefinition {
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
            journeyman_position: Position::EagleWarriorLinewoman,
            maximum_big_men_quantity: 0,
            special_rules: vec![SpecialRule::LustrianSuperleague],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::BlackOrc => None,

        Roster::ChaosChosen => Some(RosterDefinition {
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
            journeyman_position: Position::BeastmanRunnerLineman,
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::FavouredOf],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::DarkElf => Some(RosterDefinition {
            version: Version::V5,
            tier: 1,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_50_DEFINITION,
            ],
            positions: vec![
                Position::DarkElfLineman,
                Position::Runner,
                Position::Blitzer,
                Position::Assassin,
                Position::WitchElf,
            ],
            journeyman_position: Position::DarkElfLineman,
            maximum_big_men_quantity: 0,
            special_rules: vec![SpecialRule::ElvenKingdomsLeague],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::Dwarf => Some(RosterDefinition {
            version: Version::V5,
            tier: 1,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_50_DEFINITION,
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
            special_rules: vec![
                SpecialRule::WorldsEdgeSuperleague,
                SpecialRule::OldWorldClassic,
            ],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::Gnome => Some(RosterDefinition {
            version: Version::V5,
            tier: 3,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_50_DEFINITION,
            ],
            positions: vec![
                Position::GnomeLineman,
                Position::GnomeBeastmaster,
                Position::GnomeIllusionist,
                Position::WoodlandFox,
                Position::AlternForestTreeman,
            ],
            journeyman_position: Position::GnomeLineman,
            maximum_big_men_quantity: 2,
            special_rules: vec![SpecialRule::HalflingThimbleCup],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::HighElf => Some(RosterDefinition {
            version: Version::V5,
            tier: 1,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_50_DEFINITION,
            ],
            positions: vec![
                Position::Lineman,
                Position::Catcher,
                Position::Thrower,
                Position::Blitzer,
            ],
            journeyman_position: Position::Lineman,
            maximum_big_men_quantity: 0,
            special_rules: vec![SpecialRule::ElvenKingdomsLeague],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::Human => Some(RosterDefinition {
            version: Version::V5,
            tier: 2,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_50_DEFINITION,
            ],
            positions: vec![
                Position::HumanLineman,
                Position::Thrower,
                Position::Catcher,
                Position::Blitzer,
                Position::HalflingHopeful,
                Position::Ogre,
            ],
            journeyman_position: Position::HumanLineman,
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::OldWorldClassic],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::Lizardmen => Some(RosterDefinition {
            version: Version::V5,
            tier: 1,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_70_DEFINITION,
            ],
            positions: vec![
                Position::SkinkRunnerLineman,
                Position::ChameleonSkink,
                Position::SaurusBlocker,
                Position::Kroxigor,
            ],
            journeyman_position: Position::SkinkRunnerLineman,
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::LustrianSuperleague],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::NecromanticHorror => Some(RosterDefinition {
            version: Version::V5,
            tier: 2,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                REROLL_70_DEFINITION,
            ],
            positions: vec![
                Position::ZombieLineman,
                Position::GhoulRunner,
                Position::Wraith,
                Position::Werewolf,
                Position::FleshGolem,
            ],
            journeyman_position: Position::ZombieLineman,
            maximum_big_men_quantity: 0,
            special_rules: vec![
                SpecialRule::SylvanianSpotlight,
                SpecialRule::MastersOfUndeath,
            ],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::Norse => Some(RosterDefinition {
            version: Version::V5,
            tier: 1,
            staff_information: vec![
                CHEERLEADER_DEFINITION,
                ASSISTANT_COACH_DEFINITION,
                APOTHECARY_DEFINITION,
                REROLL_60_DEFINITION,
            ],
            positions: vec![
                Position::NorseRaiderLineman,
                Position::BeerBoar,
                Position::NorseBerzerker,
                Position::Valkyrie,
                Position::Ulfwerener,
                Position::Yhetee,
            ],
            journeyman_position: Position::NorseRaiderLineman,
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::OldWorldClassic, SpecialRule::FavouredOf],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::Orc => Some(RosterDefinition {
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
            journeyman_position: Position::OrcLineman,
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::BadlandsBrawl],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::Snotling => Some(RosterDefinition {
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
            journeyman_position: Position::SnotlingLineman,
            maximum_big_men_quantity: 4,
            special_rules: vec![
                SpecialRule::BriberyAndCorruption,
                SpecialRule::UnderworldChallenge,
                SpecialRule::LowCostLinemen,
            ],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),

        Roster::WoodElf => Some(RosterDefinition {
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
            journeyman_position: Position::WoodElfLineman,
            maximum_big_men_quantity: 1,
            special_rules: vec![SpecialRule::ElvenKingdomsLeague],
            dedicated_fans_information: DEDICATED_FANS_DEFINITION,
        }),
        _ => None,
    }
}
