use crate::characteristics::Characteristic;
use crate::positions::{Keyword, Position, PositionDefinition};
use crate::rosters::{Roster, SpecialLeague, SpecialRule};
use crate::skills::Skill;
use crate::versions::Version;
use std::collections::HashMap;

const VERSION: Version = Version::V5S3;

pub(crate) fn star_position_list() -> Vec<Position> {
    vec![
        Position::AkhorneTheSquirrel,
        Position::AnqiPanqi,
        Position::BarikFarblast,
        Position::BilerotVomitflesh,
        Position::BoaKonSsstriktr,
        Position::BomberDribblesnot,
        Position::CaptainKarinaVonRiesz,
        Position::CindyPiewhistle,
        Position::CountLuthorVonDrakenborg,
        Position::DeeprootStrongbranch,
        Position::DriblAndDrull,
        Position::EldrilSidewinder,
        Position::EstelleLaVeneaux,
        Position::FungusTheLoon,
        Position::GlartSmashrip,
        Position::GlorielSummerbloom,
        Position::GlotlStop,
        Position::GrakAndCrumbleberry,
        Position::GrashnakBlackhoof,
        Position::GretchenWachterTheBloodBowlWidow,
        Position::GrimIronjaw,
        Position::GrombrindalTheWhiteDwarf,
        Position::GufflePussmaw,
        Position::HelmutWulf,
        Position::IvarEriksson,
        Position::JeremiahKool,
        Position::JordellFreshbreeze,
        Position::JosefBugman,
        Position::KarlaVonKill,
        Position::KirothKrakeneye,
        Position::KreekTheVerminatorRustgouger,
        Position::LordBorakTheDespoiler,
        Position::MapleHighgrove,
        Position::MaxSpleenripper,
        Position::MightyZug,
        Position::NobblaBlackwart,
        Position::PuggyBaconbreath,
        Position::RashnakBackstabber,
        Position::RipperBolgrot,
        Position::RodneyRoachbait,
        Position::RowanaForestfoot,
        Position::RoxannaDarknail,
        Position::RumbelowSheepskin,
        Position::ScrappaSorehead,
        Position::ScylaAnfingrimm,
        Position::SkitterStabStab,
        Position::SkrorgSnowpelt,
        Position::SkrullHalfheight,
        Position::SwiftvineGlimmershard,
        Position::TheBlackGobbo,
        Position::TheSwiftTwins,
        Position::ThorssonStoutmead,
        Position::VaragGhoulChewer,
        Position::WilhelmChaney,
        Position::WillowRosebark,
        Position::WithergraspDoubledrool,
        Position::ZolcathTheZoat,
        Position::ZzhargMadeye,
    ]
}

pub(crate) fn mega_star_position_list() -> Vec<Position> {
    vec![
        Position::GriffOberwald,
        Position::HakflemSkuttlespike,
        Position::HTharkTheUnstoppable,
        Position::IvanTAnimalDeathshroud,
        Position::MorgNThorg,
    ]
}

pub(crate) fn star_player_position_definition(position: &Position) -> Option<PositionDefinition> {
    match position {
        Position::AkhorneTheSquirrel => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Squirrel],
            maximum_quantity: 1,
            cost: 80000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 1),
                (Characteristic::Agility, 2),
                (Characteristic::ArmourValue, 6),
            ]),
            skills: vec![
                Skill::Claws,
                Skill::Dauntless,
                Skill::Dodge,
                Skill::Frenzy,
                Skill::JumpUp,
                Skill::Loner(4),
                Skill::NoBall,
                Skill::SideStep,
                Skill::Stunty,
                Skill::Titchy,
                Skill::BlindRage,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::AnqiPanqi => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Lizardman],
            maximum_quantity: 1,
            cost: 190000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 5),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Grab,
                Skill::Loner(4),
                Skill::StandFirm,
                Skill::Unsteady,
                Skill::SavageBlow,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::BarikFarblast => Some(PositionDefinition {
            keywords: vec![Keyword::Thrower, Keyword::Dwarf],
            maximum_quantity: 1,
            cost: 80000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Cannoneer,
                Skill::HailMaryPass,
                Skill::Loner(4),
                Skill::Pass,
                Skill::SecretWeapon,
                Skill::SureHands,
                Skill::ThickSkull,
                Skill::BlastIt,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::BilerotVomitflesh => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Human],
            maximum_quantity: 1,
            cost: 180000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 4),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::DirtyPlayer,
                Skill::DisturbingPresence,
                Skill::FoulAppearance,
                Skill::LoneFouler,
                Skill::Loner(4),
                Skill::Regeneration,
                Skill::Unsteady,
                Skill::PutridRegurgitation,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::BoaKonSsstriktr => Some(PositionDefinition {
            keywords: vec![Keyword::Runner, Keyword::Snakeman],
            maximum_quantity: 1,
            cost: 180000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Dodge,
                Skill::Fend,
                Skill::HypnoticGaze,
                Skill::Loner(4),
                Skill::PrehensileTail,
                Skill::SafePairOfHands,
                Skill::SideStep,
                Skill::LookIntoMyEyes,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::BomberDribblesnot => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Goblin],
            maximum_quantity: 1,
            cost: 80000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Accurate,
                Skill::Bombardier,
                Skill::Dodge,
                Skill::Loner(4),
                Skill::RightStuff,
                Skill::SecretWeapon,
                Skill::Stunty,
                Skill::Kaboom,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::CaptainKarinaVonRiesz => Some(PositionDefinition {
            keywords: vec![Keyword::Runner, Keyword::Vampire],
            maximum_quantity: 1,
            cost: 230000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::BloodLust(2),
                Skill::Dodge,
                Skill::HypnoticGaze,
                Skill::JumpUp,
                Skill::Loner(4),
                Skill::Regeneration,
                Skill::TastyMorsel,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::CindyPiewhistle => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Halfling],
            maximum_quantity: 1,
            cost: 100000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 7),
            ]),
            skills: vec![
                Skill::Accurate,
                Skill::Bombardier,
                Skill::Dodge,
                Skill::Loner(4),
                Skill::SecretWeapon,
                Skill::Stunty,
                Skill::AllYouCanEat,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::CountLuthorVonDrakenborg => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Vampire],
            maximum_quantity: 1,
            cost: 300000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::Block,
                Skill::HypnoticGaze,
                Skill::Loner(4),
                Skill::Regeneration,
                Skill::SideStep,
                Skill::StarOfTheShow,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::DeeprootStrongbranch => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Treeman],
            maximum_quantity: 1,
            cost: 280000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 2),
                (Characteristic::Strength, 7),
                (Characteristic::Agility, 5),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 11),
            ]),
            skills: vec![
                Skill::Block,
                Skill::BullsEye,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::StandFirm,
                Skill::StrongArm,
                Skill::ThickSkull,
                Skill::ThrowTeamMate,
                Skill::Timmmber,
                Skill::Reliable,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::DriblAndDrull => Some(PositionDefinition {
            keywords: Vec::new(),
            maximum_quantity: 1,
            cost: 230000,
            characteristics: Default::default(),
            skills: vec![],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::Dribl => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Lizardman],
            maximum_quantity: 1,
            cost: 115000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::DirtyPlayer,
                Skill::Dodge,
                Skill::Loner(4),
                Skill::QuickFoul,
                Skill::SideStep,
                Skill::SneakyGit,
                Skill::Stunty,
                Skill::ASneakyPair,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::Drull => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Lizardman],
            maximum_quantity: 1,
            cost: 115000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Dodge,
                Skill::Loner(4),
                Skill::SideStep,
                Skill::Stab,
                Skill::Stunty,
                Skill::ASneakyPair,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::EldrilSidewinder => Some(PositionDefinition {
            keywords: vec![Keyword::Catcher, Keyword::Elf],
            maximum_quantity: 1,
            cost: 220000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Catch,
                Skill::Dodge,
                Skill::HypnoticGaze,
                Skill::Loner(4),
                Skill::NervesOfSteel,
                Skill::OnTheBall,
                Skill::MesmerizingGaze,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::EstelleLaVeneaux => Some(PositionDefinition {
            keywords: vec![Keyword::Lineman, Keyword::Human],
            maximum_quantity: 1,
            cost: 190000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::DisturbingPresence,
                Skill::Dodge,
                Skill::Guard,
                Skill::Loner(4),
                Skill::SideStep,
                Skill::BalefulHex,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::FungusTheLoon => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Goblin],
            maximum_quantity: 1,
            cost: 80000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 4),
                (Characteristic::Strength, 7),
                (Characteristic::Agility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::BallChain,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::NoBall,
                Skill::SecretWeapon,
                Skill::Stunty,
                Skill::WhirlingDervish,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::GlartSmashrip => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Skaven],
            maximum_quantity: 1,
            cost: 175000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Claws,
                Skill::Grab,
                Skill::Juggernaut,
                Skill::Loner(4),
                Skill::StandFirm,
                Skill::FrenziedRush,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::GlorielSummerbloom => Some(PositionDefinition {
            keywords: vec![Keyword::Thrower, Keyword::Elf],
            maximum_quantity: 1,
            cost: 150000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 2),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Accurate,
                Skill::Dodge,
                Skill::Loner(3),
                Skill::Pass,
                Skill::SideStep,
                Skill::SureHands,
                Skill::ShotToNothing,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::GlotlStop => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Lizardman],
            maximum_quantity: 1,
            cost: 260000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 6),
                (Characteristic::Agility, 5),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::AnimalSavagery,
                Skill::Frenzy,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::PrehensileTail,
                Skill::StandFirm,
                Skill::ThickSkull,
                Skill::PrimalSavagery,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::GrakAndCrumbleberry => Some(PositionDefinition {
            keywords: Vec::new(),
            maximum_quantity: 1,
            cost: 250000,
            characteristics: Default::default(),
            skills: vec![],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::Grak => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Ogre],
            maximum_quantity: 1,
            cost: 125000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::BoneHead,
                Skill::KickTeamMate,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::ThickSkull,
                Skill::TwoForOne,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::Crumbleberry => Some(PositionDefinition {
            keywords: vec![Keyword::Lineman, Keyword::Halfling],
            maximum_quantity: 1,
            cost: 125000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 7),
            ]),
            skills: vec![
                Skill::Dodge,
                Skill::LethalFlight,
                Skill::Loner(4),
                Skill::RightStuff,
                Skill::Stunty,
                Skill::SureHands,
                Skill::TwoForOne,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::GrashnakBlackhoof => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Minotaur],
            maximum_quantity: 1,
            cost: 240000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 6),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Frenzy,
                Skill::Horns,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::ThickSkull,
                Skill::UnchannelledFury,
                Skill::GoredByTheBull,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::GretchenWachterTheBloodBowlWidow => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Undead, Keyword::Wraith],
            maximum_quantity: 1,
            cost: 180000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::DisturbingPresence,
                Skill::Dodge,
                Skill::FoulAppearance,
                Skill::JumpUp,
                Skill::Loner(4),
                Skill::NoBall,
                Skill::Regeneration,
                Skill::Shadowing,
                Skill::SideStep,
                Skill::Incorporeal,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),
        Position::GriffOberwald => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Human],
            maximum_quantity: 1,
            cost: 300000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Dodge,
                Skill::Fend,
                Skill::Loner(3),
                Skill::Sprint,
                Skill::SureFeet,
                Skill::ConsummateProfessional,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::GrimIronjaw => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Dwarf],
            maximum_quantity: 1,
            cost: 200000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Dauntless,
                Skill::Frenzy,
                Skill::Hatred(Keyword::BigGuy),
                Skill::Loner(4),
                Skill::MultipleBlock,
                Skill::ThickSkull,
                Skill::Slayer,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::GrombrindalTheWhiteDwarf => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Dwarf],
            maximum_quantity: 1,
            cost: 170000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::Block,
                Skill::BreakTackle,
                Skill::Dauntless,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::StandFirm,
                Skill::SureFeet,
                Skill::ThickSkull,
                Skill::WisdomOfTheWhiteDwarf,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::GufflePussmaw => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Human],
            maximum_quantity: 1,
            cost: 150000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::FoulAppearance,
                Skill::Loner(4),
                Skill::MonstrousMouth,
                Skill::NervesOfSteel,
                Skill::OnTheBall,
                Skill::PlagueRidden,
                Skill::QuickBite,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::HakflemSkuttlespike => Some(PositionDefinition {
            keywords: vec![Keyword::Runner, Keyword::Skaven],
            maximum_quantity: 1,
            cost: 200000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 9),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Dodge,
                Skill::ExtraArms,
                Skill::Loner(4),
                Skill::PrehensileTail,
                Skill::TwoHeads,
                Skill::Treacherous,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::HelmutWulf => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Human],
            maximum_quantity: 1,
            cost: 140000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Chainsaw,
                Skill::Loner(4),
                Skill::NoBall,
                Skill::Pro,
                Skill::SecretWeapon,
                Skill::StandFirm,
                Skill::OldPro,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::HTharkTheUnstoppable => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Dwarf],
            maximum_quantity: 1,
            cost: 300000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 6),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::Block,
                Skill::BreakTackle,
                Skill::Defensive,
                Skill::Juggernaut,
                Skill::Loner(4),
                Skill::Sprint,
                Skill::SureFeet,
                Skill::ThickSkull,
                Skill::UnstoppableMomentum,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::IvanTAnimalDeathshroud => Some(PositionDefinition {
            keywords: vec![
                Keyword::Blitzer,
                Keyword::Human,
                Keyword::Skeleton,
                Keyword::Undead,
            ],
            maximum_quantity: 1,
            cost: 210000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::DisturbingPresence,
                Skill::Hatred(Keyword::Dwarf),
                Skill::Juggernaut,
                Skill::Loner(4),
                Skill::Regeneration,
                Skill::StripBall,
                Skill::Tackle,
                Skill::DwarvenScourge,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::IvarEriksson => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Human],
            maximum_quantity: 1,
            cost: 215000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Guard,
                Skill::Loner(3),
                Skill::Tackle,
                Skill::RaidingParty,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::JeremiahKool => Some(PositionDefinition {
            keywords: vec![Keyword::Runner, Keyword::Elf],
            maximum_quantity: 1,
            cost: 300000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 1),
                (Characteristic::PassingAbility, 2),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Dodge,
                Skill::DivingCatch,
                Skill::DumpOff,
                Skill::Loner(4),
                Skill::NervesOfSteel,
                Skill::OnTheBall,
                Skill::Pass,
                Skill::SideStep,
                Skill::TheFlashingBlade,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::JordellFreshbreeze => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Elf],
            maximum_quantity: 1,
            cost: 280000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 1),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Block,
                Skill::DivingCatch,
                Skill::Dodge,
                Skill::Leap,
                Skill::Loner(4),
                Skill::SideStep,
                Skill::SteadyFooting,
                Skill::SwiftAsTheBreeze,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::JosefBugman => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Dwarf],
            maximum_quantity: 1,
            cost: 180000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Drunkard,
                Skill::Fend,
                Skill::Loner(3),
                Skill::Tackle,
                Skill::Taunt,
                Skill::ThickSkull,
                Skill::DwarfenGrit,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::KarlaVonKill => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Human],
            maximum_quantity: 1,
            cost: 210000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Dauntless,
                Skill::Dodge,
                Skill::JumpUp,
                Skill::Loner(4),
                Skill::Indomitable,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::KirothKrakeneye => Some(PositionDefinition {
            keywords: vec![Keyword::Runner, Keyword::Elf],
            maximum_quantity: 1,
            cost: 160000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::DisturbingPresence,
                Skill::FoulAppearance,
                Skill::Loner(4),
                Skill::OnTheBall,
                Skill::Tackle,
                Skill::Tentacles,
                Skill::BlackInk,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::KreekTheVerminatorRustgouger => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Skaven, Keyword::Special],
            maximum_quantity: 1,
            cost: 180000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 7),
                (Characteristic::Agility, 4),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::BallChain,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::NoBall,
                Skill::PrehensileTail,
                Skill::SecretWeapon,
                Skill::IllBeBack,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::LordBorakTheDespoiler => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Human],
            maximum_quantity: 1,
            cost: 270000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::Block,
                Skill::DirtyPlayer,
                Skill::Leader,
                Skill::Loner(3),
                Skill::MightyBlow,
                Skill::PutTheBootIn,
                Skill::SneakyGit,
                Skill::LordOfChaos,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::MapleHighgrove => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Treeman],
            maximum_quantity: 1,
            cost: 210000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 3),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 5),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 11),
            ]),
            skills: vec![
                Skill::Brawler,
                Skill::Grab,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::StandFirm,
                Skill::Tentacles,
                Skill::ThickSkull,
                Skill::ViciousVines,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::MaxSpleenripper => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Human],
            maximum_quantity: 1,
            cost: 130000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Chainsaw,
                Skill::Loner(4),
                Skill::NoBall,
                Skill::SecretWeapon,
                Skill::MaximumCarnage,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::MightyZug => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Human],
            maximum_quantity: 1,
            cost: 220000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::CrushingBlow,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::MorgNThorg => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Ogre],
            maximum_quantity: 1,
            cost: 380000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 6),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 11),
            ]),
            skills: vec![
                Skill::Block,
                Skill::BullsEye,
                Skill::Hatred(Keyword::Undead),
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::ThickSkull,
                Skill::ThrowTeamMate,
                Skill::TheBallista,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::NobblaBlackwart => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Goblin],
            maximum_quantity: 1,
            cost: 120000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Chainsaw,
                Skill::Dodge,
                Skill::Loner(4),
                Skill::NoBall,
                Skill::SecretWeapon,
                Skill::Stunty,
                Skill::KickThemWhileTheyAreDown,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::PuggyBaconbreath => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Halfling],
            maximum_quantity: 1,
            cost: 120000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Dodge,
                Skill::Loner(3),
                Skill::NervesOfSteel,
                Skill::RightStuff,
                Skill::Stunty,
                Skill::HalflingLuck,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::RashnakBackstabber => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Goblin],
            maximum_quantity: 1,
            cost: 130000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Loner(4),
                Skill::Shadowing,
                Skill::SideStep,
                Skill::SneakyGit,
                Skill::Stab,
                Skill::ToxinConnoisseur,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::RipperBolgrot => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Troll],
            maximum_quantity: 1,
            cost: 250000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 6),
                (Characteristic::Agility, 5),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::BullsEye,
                Skill::Grab,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::Regeneration,
                Skill::ThrowTeamMate,
                Skill::ThinkingManTroll,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::RodneyRoachbait => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Gnome],
            maximum_quantity: 1,
            cost: 70000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 7),
            ]),
            skills: vec![
                Skill::Catch,
                Skill::DivingCatch,
                Skill::JumpUp,
                Skill::Loner(4),
                Skill::OnTheBall,
                Skill::SideStep,
                Skill::Stunty,
                Skill::Wrestle,
                Skill::CatchOfTheDay,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::RowanaForestfoot => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Gnome],
            maximum_quantity: 1,
            cost: 160000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Dodge,
                Skill::DumpOff,
                Skill::Guard,
                Skill::Horns,
                Skill::JumpUp,
                Skill::Leap,
                Skill::Loner(4),
                Skill::BoundingLeap,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::RoxannaDarknail => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Elf],
            maximum_quantity: 1,
            cost: 270000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 1),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Dodge,
                Skill::Frenzy,
                Skill::JumpUp,
                Skill::Juggernaut,
                Skill::Leap,
                Skill::Loner(4),
                Skill::SlashingNails,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::RumbelowSheepskin => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Halfling],
            maximum_quantity: 1,
            cost: 170000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Horns,
                Skill::Juggernaut,
                Skill::Loner(4),
                Skill::ThickSkull,
                Skill::Tackle,
                Skill::Ram,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::ScrappaSorehead => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Goblin],
            maximum_quantity: 1,
            cost: 120000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::DirtyPlayer,
                Skill::Dodge,
                Skill::Loner(4),
                Skill::PogoStick,
                Skill::RightStuff,
                Skill::Sprint,
                Skill::Stunty,
                Skill::SureFeet,
                Skill::Yoink,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::ScylaAnfingrimm => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Spawn],
            maximum_quantity: 1,
            cost: 200000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::Claws,
                Skill::Frenzy,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::PrehensileTail,
                Skill::ThickSkull,
                Skill::UnchannelledFury,
                Skill::FuryOfTheBloodGod,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::SkitterStabStab => Some(PositionDefinition {
            keywords: vec![Keyword::Runner, Keyword::Skaven],
            maximum_quantity: 1,
            cost: 170000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 9),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Dodge,
                Skill::Loner(4),
                Skill::PrehensileTail,
                Skill::Stab,
                Skill::Shadowing,
                Skill::MasterAssassin,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::SkrorgSnowpelt => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Yhetee],
            maximum_quantity: 1,
            cost: 240000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 6),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Claws,
                Skill::DisturbingPresence,
                Skill::Juggernaut,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::PumpUpTheCrowd,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::SkrullHalfheight => Some(PositionDefinition {
            keywords: vec![
                Keyword::Thrower,
                Keyword::Dwarf,
                Keyword::Undead,
                Keyword::Skeleton,
            ],
            maximum_quantity: 1,
            cost: 150000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Accurate,
                Skill::Loner(4),
                Skill::NervesOfSteel,
                Skill::Pass,
                Skill::Regeneration,
                Skill::SureHands,
                Skill::ThickSkull,
                Skill::StrongPassingGame,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::SwiftvineGlimmershard => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Spite],
            maximum_quantity: 1,
            cost: 110000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 7),
            ]),
            skills: vec![
                Skill::DisturbingPresence,
                Skill::Fend,
                Skill::Loner(4),
                Skill::SideStep,
                Skill::Stab,
                Skill::Stunty,
                Skill::FuriousOutburst,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::TheBlackGobbo => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Goblin],
            maximum_quantity: 1,
            cost: 210000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 2),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Bombardier,
                Skill::DisturbingPresence,
                Skill::Dodge,
                Skill::Loner(3),
                Skill::SideStep,
                Skill::SneakyGit,
                Skill::Stab,
                Skill::Stunty,
                Skill::SneakiestOfTheLot,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::TheSwiftTwins => Some(PositionDefinition {
            keywords: Vec::new(),
            maximum_quantity: 1,
            cost: 300000,
            characteristics: Default::default(),
            skills: vec![],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::LucienSwift => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Elf],
            maximum_quantity: 1,
            cost: 150000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::Tackle,
                Skill::WorkingInTandem,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::ValenSwift => Some(PositionDefinition {
            keywords: vec![Keyword::Thrower, Keyword::Elf],
            maximum_quantity: 1,
            cost: 150000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 7),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 2),
                (Characteristic::PassingAbility, 2),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Accurate,
                Skill::Loner(4),
                Skill::NervesOfSteel,
                Skill::Pass,
                Skill::SafePass,
                Skill::SureHands,
                Skill::WorkingInTandem,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::ThorssonStoutmead => Some(PositionDefinition {
            keywords: vec![Keyword::Lineman, Keyword::Human],
            maximum_quantity: 1,
            cost: 170000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 8),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Drunkard,
                Skill::Loner(4),
                Skill::ThickSkull,
                Skill::BeerBarrelBash,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::VaragGhoulChewer => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Orc],
            maximum_quantity: 1,
            cost: 260000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::Block,
                Skill::Hatred(Keyword::Undead),
                Skill::JumpUp,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::ThickSkull,
                Skill::Unsteady,
                Skill::KrumpAndSmash,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::WilhelmChaney => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Undead, Keyword::Werewolf],
            maximum_quantity: 1,
            cost: 220000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 8),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Catch,
                Skill::Claws,
                Skill::Frenzy,
                Skill::Loner(4),
                Skill::Regeneration,
                Skill::Wrestle,
                Skill::SavageMauling,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::WillowRosebark => Some(PositionDefinition {
            keywords: vec![Keyword::Blitzer, Keyword::Dryad],
            maximum_quantity: 1,
            cost: 160000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::Dauntless,
                Skill::Loner(4),
                Skill::SideStep,
                Skill::ThickSkull,
                Skill::WoodlandFury,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::WithergraspDoubledrool => Some(PositionDefinition {
            keywords: vec![Keyword::Blocker, Keyword::Beastman],
            maximum_quantity: 1,
            cost: 170000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 6),
                (Characteristic::Strength, 3),
                (Characteristic::Agility, 3),
                (Characteristic::PassingAbility, 4),
                (Characteristic::ArmourValue, 9),
            ]),
            skills: vec![
                Skill::FoulAppearance,
                Skill::Loner(4),
                Skill::PrehensileTail,
                Skill::Tackle,
                Skill::Tentacles,
                Skill::TwoHeads,
                Skill::Wrestle,
                Skill::WatchOut,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::ZolcathTheZoat => Some(PositionDefinition {
            keywords: vec![Keyword::BigGuy, Keyword::Zoat],
            maximum_quantity: 1,
            cost: 220000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 5),
                (Characteristic::Strength, 5),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 5),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::DisturbingPresence,
                Skill::Juggernaut,
                Skill::Loner(4),
                Skill::MightyBlow,
                Skill::PrehensileTail,
                Skill::Regeneration,
                Skill::SureFeet,
                Skill::ExcuseMeAreYouAZoat,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        Position::ZzhargMadeye => Some(PositionDefinition {
            keywords: vec![Keyword::Special, Keyword::Dwarf],
            maximum_quantity: 1,
            cost: 130000,
            characteristics: HashMap::from([
                (Characteristic::MovementAllowance, 4),
                (Characteristic::Strength, 4),
                (Characteristic::Agility, 4),
                (Characteristic::PassingAbility, 3),
                (Characteristic::ArmourValue, 10),
            ]),
            skills: vec![
                Skill::Cannoneer,
                Skill::HailMaryPass,
                Skill::Loner(4),
                Skill::NervesOfSteel,
                Skill::SecretWeapon,
                Skill::ThickSkull,
                Skill::BlastingSolvesEverything,
            ],
            primary_skill_categories: vec![],
            secondary_skill_categories: vec![],
            is_big_man: false,
        }),

        _ => None,
    }
}

pub(crate) fn star_maximum_for_roster(position: &Position, roster: &Roster) -> usize {
    let accepted = match (position, roster.definition(VERSION)) {
        (Position::AkhorneTheSquirrel, _) => true,
        (Position::AnqiPanqi, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::LustrianSuperleague),
        (Position::BarikFarblast, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::OldWorldClassic)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::WorldsEdgeSuperleague)
        }
        (Position::BilerotVomitflesh, Some(roster_definition)) => {
            roster_definition
                .special_rules
                .contains(&SpecialRule::FavouredOfNurgle)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::FavouredOf)
        }
        (Position::BoaKonSsstriktr, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::LustrianSuperleague),
        (Position::BomberDribblesnot, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::BadlandsBrawl)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::UnderworldChallenge)
        }
        (Position::CaptainKarinaVonRiesz, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::SylvanianSpotlight),
        (Position::CindyPiewhistle, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::HalflingThimbleCup),
        (Position::CountLuthorVonDrakenborg, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::SylvanianSpotlight),
        (Position::DeeprootStrongbranch, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WoodlandLeague),
        (Position::DriblAndDrull, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::LustrianSuperleague),
        (Position::Dribl, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::LustrianSuperleague),
        (Position::Drull, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::LustrianSuperleague),
        (Position::EldrilSidewinder, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ElvenKingdomsLeague),
        (Position::EstelleLaVeneaux, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::LustrianSuperleague),
        (Position::FungusTheLoon, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::BadlandsBrawl)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::UnderworldChallenge)
        }
        (Position::GlartSmashrip, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::UnderworldChallenge),
        (Position::GlorielSummerbloom, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ElvenKingdomsLeague),
        (Position::GlotlStop, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::LustrianSuperleague),
        (Position::GrakAndCrumbleberry, _) => true,
        (Position::Grak, _) => true,
        (Position::Crumbleberry, _) => true,
        (Position::GrashnakBlackhoof, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ChaosClash),
        (Position::GretchenWachterTheBloodBowlWidow, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::SylvanianSpotlight),
        (Position::GriffOberwald, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::OldWorldClassic),
        (Position::GrimIronjaw, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WorldsEdgeSuperleague),
        (Position::GrombrindalTheWhiteDwarf, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::HalflingThimbleCup)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::OldWorldClassic)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::WorldsEdgeSuperleague)
        }
        (Position::GufflePussmaw, Some(roster_definition)) => {
            roster_definition
                .special_rules
                .contains(&SpecialRule::FavouredOfNurgle)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::FavouredOf)
        }
        (Position::HakflemSkuttlespike, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::UnderworldChallenge),
        (Position::HelmutWulf, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WorldsEdgeSuperleague),
        (Position::HTharkTheUnstoppable, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::BadlandsBrawl)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::FavouredOfHashut)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::FavouredOf)
        }
        (Position::IvarEriksson, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WorldsEdgeSuperleague),
        (Position::IvanTAnimalDeathshroud, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::SylvanianSpotlight),
        (Position::JeremiahKool, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ElvenKingdomsLeague),
        (Position::JordellFreshbreeze, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::ElvenKingdomsLeague)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::WoodlandLeague)
        }
        (Position::JosefBugman, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WorldsEdgeSuperleague),
        (Position::KarlaVonKill, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::LustrianSuperleague)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::OldWorldClassic)
        }
        (Position::KirothKrakeneye, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ElvenKingdomsLeague),
        (Position::KreekTheVerminatorRustgouger, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::UnderworldChallenge),
        (Position::LordBorakTheDespoiler, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ChaosClash),
        (Position::MapleHighgrove, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WoodlandLeague),
        (Position::MaxSpleenripper, Some(roster_definition)) => {
            roster_definition
                .special_rules
                .contains(&SpecialRule::FavouredOfKhorne)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::FavouredOf)
        }
        (Position::MightyZug, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::OldWorldClassic)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::WorldsEdgeSuperleague)
        }
        (Position::MorgNThorg, Some(roster_definition)) => !roster_definition
            .special_leagues
            .contains(&SpecialLeague::SylvanianSpotlight),
        (Position::NobblaBlackwart, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::BadlandsBrawl)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::UnderworldChallenge)
        }
        (Position::PuggyBaconbreath, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::HalflingThimbleCup)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::OldWorldClassic)
        }
        (Position::RashnakBackstabber, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::BadlandsBrawl),
        (Position::RipperBolgrot, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::BadlandsBrawl)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::UnderworldChallenge)
        }
        (Position::RodneyRoachbait, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WoodlandLeague),
        (Position::RowanaForestfoot, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WoodlandLeague),
        (Position::RoxannaDarknail, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ElvenKingdomsLeague),
        (Position::RumbelowSheepskin, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::HalflingThimbleCup),
        (Position::ScrappaSorehead, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::BadlandsBrawl)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::UnderworldChallenge)
        }
        (Position::ScylaAnfingrimm, Some(roster_definition)) => {
            roster_definition
                .special_rules
                .contains(&SpecialRule::FavouredOfKhorne)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::FavouredOf)
        }
        (Position::SkitterStabStab, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::UnderworldChallenge),
        (Position::SkrorgSnowpelt, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::OldWorldClassic)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::WorldsEdgeSuperleague)
        }
        (Position::SkrullHalfheight, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::SylvanianSpotlight)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::WorldsEdgeSuperleague)
        }
        (Position::SwiftvineGlimmershard, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WoodlandLeague),
        (Position::TheBlackGobbo, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::BadlandsBrawl)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::UnderworldChallenge)
        }
        (Position::TheSwiftTwins, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ElvenKingdomsLeague),
        (Position::LucienSwift, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ElvenKingdomsLeague),
        (Position::ValenSwift, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::ElvenKingdomsLeague),
        (Position::ThorssonStoutmead, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::OldWorldClassic)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::WorldsEdgeSuperleague)
        }
        (Position::VaragGhoulChewer, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::BadlandsBrawl)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::UnderworldChallenge)
        }
        (Position::WilhelmChaney, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::SylvanianSpotlight),
        (Position::WillowRosebark, Some(roster_definition)) => roster_definition
            .special_leagues
            .contains(&SpecialLeague::WoodlandLeague),
        (Position::WithergraspDoubledrool, Some(roster_definition)) => {
            roster_definition
                .special_rules
                .contains(&SpecialRule::FavouredOfNurgle)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::FavouredOf)
        }
        (Position::ZolcathTheZoat, Some(roster_definition)) => {
            roster_definition
                .special_leagues
                .contains(&SpecialLeague::ElvenKingdomsLeague)
                || roster_definition
                    .special_leagues
                    .contains(&SpecialLeague::LustrianSuperleague)
        }
        (Position::ZzhargMadeye, Some(roster_definition)) => {
            roster_definition
                .special_rules
                .contains(&SpecialRule::FavouredOfHashut)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::FavouredOf)
        }

        _ => false,
    };

    if accepted { 1 } else { 0 }
}
