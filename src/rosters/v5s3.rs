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

        Roster::ElvenUnion => Some(RosterDefinition {
            version: VERSION,
            tier: 2,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_50,
            ],
            positions: vec![
                Position::Lineman,
                Position::Thrower,
                Position::Catcher,
                Position::Blitzer,
            ],
            journeyman_position: Position::Lineman,
            maximum_big_men_quantity: 0,
            special_leagues: vec![SpecialLeague::ElvenKingdomsLeague],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Gnome => Some(RosterDefinition {
            version: VERSION,
            tier: 4,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_50,
            ],
            positions: vec![
                Position::GnomeLineman,
                Position::WoodlandFox,
                Position::GnomeIllusionist,
                Position::GnomeBeastmaster,
                Position::AlternForestTreeman,
            ],
            journeyman_position: Position::GnomeLineman,
            maximum_big_men_quantity: 2,
            special_leagues: vec![
                SpecialLeague::HalflingThimbleCup,
                SpecialLeague::WoodlandLeague,
            ],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Goblin => Some(RosterDefinition {
            version: VERSION,
            tier: 4,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::GoblinLineman,
                Position::Looney,
                Position::Bomma,
                Position::Ooligan,
                Position::DoomDiver,
                Position::Fanatic,
                Position::Pogoer,
                Position::TrainedTroll,
            ],
            journeyman_position: Position::GoblinLineman,
            maximum_big_men_quantity: 2,
            special_leagues: vec![
                SpecialLeague::BadlandsBrawl,
                SpecialLeague::UnderworldChallenge,
            ],
            special_rules: vec![SpecialRule::BriberyAndCorruption],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Halfling => Some(RosterDefinition {
            version: VERSION,
            tier: 4,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::HalflingHopefulLineman,
                Position::HalflingHefty,
                Position::HalflingCatcher,
                Position::AlternForestTreeman,
            ],
            journeyman_position: Position::HalflingHopefulLineman,
            maximum_big_men_quantity: 2,
            special_leagues: vec![
                SpecialLeague::HalflingThimbleCup,
                SpecialLeague::WoodlandLeague,
            ],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::HighElf => Some(RosterDefinition {
            version: VERSION,
            tier: 1,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_50,
            ],
            positions: vec![
                Position::Lineman,
                Position::Thrower,
                Position::Catcher,
                Position::Blitzer,
            ],
            journeyman_position: Position::Lineman,
            maximum_big_men_quantity: 0,
            special_leagues: vec![SpecialLeague::ElvenKingdomsLeague],
            special_rules: Vec::new(),
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

        Roster::ImperialNobility => Some(RosterDefinition {
            version: VERSION,
            tier: 2,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::ImperialRetainerLineman,
                Position::ImperialThrower,
                Position::Bodyguard,
                Position::NobleBlitzer,
                Position::Ogre,
            ],
            journeyman_position: Position::ImperialRetainerLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::OldWorldClassic],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Khorne => Some(RosterDefinition {
            version: VERSION,
            tier: 3,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::BloodbornMarauderLineman,
                Position::Khorngor,
                Position::Bloodseeker,
                Position::Bloodspawn,
            ],
            journeyman_position: Position::BloodbornMarauderLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::ChaosClash],
            special_rules: vec![SpecialRule::BrawlingBrutes, SpecialRule::FavouredOfKhorne],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Lizardmen => Some(RosterDefinition {
            version: VERSION,
            tier: 1,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_70,
            ],
            positions: vec![
                Position::SkinkRunnerLineman,
                Position::ChameleonSkink,
                Position::SaurusBlocker,
                Position::Kroxigor,
            ],
            journeyman_position: Position::SkinkRunnerLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::LustrianSuperleague],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::NecromanticHorror => Some(RosterDefinition {
            version: VERSION,
            tier: 2,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::REROLL_70,
            ],
            positions: vec![
                Position::ZombieLineman,
                Position::GhoulRunner,
                Position::Wraith,
                Position::FleshGolem,
                Position::Werewolf,
            ],
            journeyman_position: Position::ZombieLineman,
            maximum_big_men_quantity: 0,
            special_leagues: vec![SpecialLeague::SylvanianSpotlight],
            special_rules: vec![SpecialRule::MastersOfUndeath],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Norse => Some(RosterDefinition {
            version: VERSION,
            tier: 1,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
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
            special_leagues: vec![SpecialLeague::ChaosClash, SpecialLeague::OldWorldClassic],
            special_rules: vec![SpecialRule::FavouredOfKhorne],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Nurgle => Some(RosterDefinition {
            version: VERSION,
            tier: 3,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::RotterLineman,
                Position::Pestigor,
                Position::Bloater,
                Position::Rotspawn,
            ],
            journeyman_position: Position::RotterLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::ChaosClash],
            special_rules: vec![SpecialRule::BrawlingBrutes, SpecialRule::FavouredOfNurgle],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Ogre => Some(RosterDefinition {
            version: VERSION,
            tier: 4,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_70,
            ],
            positions: vec![
                Position::GnoblarLineman,
                Position::OgreBlocker,
                Position::OgreRuntPunter,
            ],
            journeyman_position: Position::GnoblarLineman,
            maximum_big_men_quantity: 6,
            special_leagues: vec![
                SpecialLeague::BadlandsBrawl,
                SpecialLeague::WorldsEdgeSuperleague,
            ],
            special_rules: vec![SpecialRule::LowCostLinemen],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::OldWorldAlliance => Some(RosterDefinition {
            version: VERSION,
            tier: 1,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_70,
            ],
            positions: vec![
                Position::OldWorldHumanLineman,
                Position::OldWorldHalflingHopefuls,
                Position::OldWorldHumanCatcher,
                Position::OldWorldDwarfBlocker,
                Position::OldWorldHumanThrower,
                Position::OldWorldDwarfRunner,
                Position::OldWorldHumanBlitzer,
                Position::OldWorldDwarfBlitzer,
                Position::OldWorldDwarfTrollSlayer,
                Position::Ogre,
                Position::AlternForestTreeman,
            ],
            journeyman_position: Position::OldWorldHumanLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::OldWorldClassic],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Orc => Some(RosterDefinition {
            version: VERSION,
            tier: 2,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_60,
            ],
            positions: vec![
                Position::OrcLineman,
                Position::Goblin,
                Position::Thrower,
                Position::Blitzer,
                Position::BigUn,
                Position::UntrainedTroll,
            ],
            journeyman_position: Position::OrcLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::BadlandsBrawl],
            special_rules: vec![SpecialRule::BrawlingBrutes, SpecialRule::TeamCaptain],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::ShamblingUndead => Some(RosterDefinition {
            version: VERSION,
            tier: 2,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::REROLL_70,
            ],
            positions: vec![
                Position::SkeletonLineman,
                Position::ZombieLineman,
                Position::GhoulRunner,
                Position::WightBlitzer,
                Position::Mummy,
            ],
            journeyman_position: Position::SkeletonLineman,
            maximum_big_men_quantity: 0,
            special_leagues: vec![SpecialLeague::SylvanianSpotlight],
            special_rules: vec![SpecialRule::MastersOfUndeath],
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Skaven => Some(RosterDefinition {
            version: VERSION,
            tier: 2,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_50,
            ],
            positions: vec![
                Position::SkavenClanratLineman,
                Position::Thrower,
                Position::GutterRunner,
                Position::Blitzer,
                Position::RatOgre,
            ],
            journeyman_position: Position::SkavenClanratLineman,
            maximum_big_men_quantity: 1,
            special_leagues: vec![SpecialLeague::UnderworldChallenge],
            special_rules: Vec::new(),
            dedicated_fans_information: DedicatedFansInformation::DEFAULT,
        }),

        Roster::Snotling => Some(RosterDefinition {
            version: VERSION,
            tier: 4,
            staff_information: vec![
                StaffInformation::CHEERLEADER,
                StaffInformation::ASSISTANT,
                StaffInformation::APOTHECARY,
                StaffInformation::REROLL_70,
            ],
            positions: vec![
                Position::SnotlingLineman,
                Position::FunHoppa,
                Position::StiltyRunna,
                Position::FungusFlinga,
                Position::PumpWagon,
                Position::TrainedTroll,
            ],
            journeyman_position: Position::SnotlingLineman,
            maximum_big_men_quantity: 4,
            special_leagues: vec![SpecialLeague::UnderworldChallenge],
            special_rules: vec![
                SpecialRule::BriberyAndCorruption,
                SpecialRule::LowCostLinemen,
                SpecialRule::Swarming,
            ],
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
