use crate::characteristics::Characteristic;
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod v5;
pub mod v5s3;

#[derive(sqlx::Type, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[sqlx(type_name = "varchar")]
pub enum Position {
    // Common
    All,
    Lineman,
    Thrower,
    Catcher,
    Blitzer,
    Runner,

    // Amazon
    EagleWarriorLinewoman,
    PythonWarriorThrower,
    PiranhaWarriorBlitzer,
    JaguarWarriorBlocker,

    // Chaos Chosen
    BeastmanRunnerLineman,
    ChosenBlocker,
    ChaosTroll,
    ChaosOgre,
    Minotaur,

    // DarkElf
    DarkElfLineman,
    Assassin,
    WitchElf,

    // Dwarf
    DwarfBlockerLineman,
    TrollSlayer,
    Deathroller,

    // Gnome
    GnomeLineman,
    GnomeBeastmaster,
    GnomeIllusionist,
    WoodlandFox,
    AlternForestTreeman,

    // Human
    HumanLineman,
    HalflingHopeful,
    Ogre,

    // Lizardmen
    SkinkRunnerLineman,
    ChameleonSkink,
    SaurusBlocker,
    Kroxigor,

    // NecromanticHorror
    ZombieLineman,
    GhoulRunner,
    Wraith,
    Werewolf,
    FleshGolem,

    // Norse
    NorseRaiderLineman,
    BeerBoar,
    NorseBerzerker,
    Valkyrie,
    Ulfwerener,
    Yhetee,

    // Orc
    OrcLineman,
    BigUn,
    Goblin,
    UntrainedTroll,

    // Snotling
    SnotlingLineman,
    FungusFlinga,
    FunHoppa,
    StiltyRunna,
    PumpWagon,
    TrainedTroll,

    // Wood Elf
    WoodElfLineman,
    Wardancer,
    LorenForestTreeman,

    // Others
    Journeyman,

    // Star players
    AkhorneTheSquirrel,
    AnqiPanqi,
    BarikFarblast,
    BilerotVomitflesh,
    BoaKonSsstriktr,
    BomberDribblesnot,
    BryceTheSliceCambuel,
    CaptainKarinaVonRiesz,
    CindyPiewhistle,
    CountLuthorVonDrakenborg,
    DeeprootStrongbranch,
    DriblAndDrull,
    EldrilSidewinder,
    EstelleLaVeneaux,
    FrankNStein,
    FungusTheLoon,
    GlartSmashrip,
    GlorielSummerbloom,
    GlotlStop,
    GrakAndCrumbleberry,
    GrashnakBlackhoof,
    GretchenWachterTheBloodBowlWidow,
    GriffOberwald,
    GrimIronjaw,
    GrombrindalTheWhiteDwarf,
    GufflePussmaw,
    HakflemSkuttlespike,
    HelmutWulf,
    HTharkTheUnstoppable,
    IvanTAnimalDeathshroud,
    IvarEriksson,
    JeremiahKool,
    JordellFreshbreeze,
    KarlaVonKill,
    KirothKrakeneye,
    KreekTheVerminatorRustgouger,
    LordBorakTheDespoiler,
    MapleHighgrove,
    MaxSpleenripper,
    MightyZug,
    MorgNThorg,
    NobblaBlackwart,
    PuggyBaconbreath,
    RashnakBackstabber,
    RipperBolgrot,
    RodneyRoachbait,
    RowanaForestfoot,
    RoxannaDarknail,
    RumbelowSheepskin,
    ScrappaSorehead,
    ScylaAnfingrimm,
    SkitterStabStab,
    SkrorgSnowpelt,
    SkrullHalfheight,
    SwiftvineGlimmershard,
    TheBlackGobbo,
    TheSwiftTwins,
    ThorssonStoutmead,
    VaragGhoulChewer,
    WilhelmChaney,
    WillowRosebark,
    WithergraspDoubledrool,
    ZolcathTheZoat,
    ZzhargMadeye,
}

impl TypeName for Position {}
impl TranslatedName for Position {}

impl Position {
    pub fn definition(self, version: Version, roster: Roster) -> Option<PositionDefinition> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => None,
            Version::V5 => v5::positon_definition_from(roster, self),
            Version::V5S3 => v5s3::positon_definition_from(roster, self),
        }
    }

    pub fn is_journeyman(self) -> bool {
        matches!(self, Self::Journeyman)
    }

    pub fn is_star(self) -> bool {
        matches!(
            self,
            Self::AkhorneTheSquirrel
                | Self::AnqiPanqi
                | Self::BarikFarblast
                | Self::BilerotVomitflesh
                | Self::BoaKonSsstriktr
                | Self::BomberDribblesnot
                | Self::BryceTheSliceCambuel
                | Self::CaptainKarinaVonRiesz
                | Self::CindyPiewhistle
                | Self::CountLuthorVonDrakenborg
                | Self::DeeprootStrongbranch
                | Self::DriblAndDrull
                | Self::EldrilSidewinder
                | Self::EstelleLaVeneaux
                | Self::FrankNStein
                | Self::FungusTheLoon
                | Self::GlartSmashrip
                | Self::GlorielSummerbloom
                | Self::GlotlStop
                | Self::GrakAndCrumbleberry
                | Self::GrashnakBlackhoof
                | Self::GretchenWachterTheBloodBowlWidow
                | Self::GriffOberwald
                | Self::GrimIronjaw
                | Self::GrombrindalTheWhiteDwarf
                | Self::GufflePussmaw
                | Self::HakflemSkuttlespike
                | Self::HelmutWulf
                | Self::HTharkTheUnstoppable
                | Self::IvanTAnimalDeathshroud
                | Self::IvarEriksson
                | Self::JeremiahKool
                | Self::JordellFreshbreeze
                | Self::KarlaVonKill
                | Self::KirothKrakeneye
                | Self::KreekTheVerminatorRustgouger
                | Self::LordBorakTheDespoiler
                | Self::MapleHighgrove
                | Self::MaxSpleenripper
                | Self::MightyZug
                | Self::MorgNThorg
                | Self::NobblaBlackwart
                | Self::PuggyBaconbreath
                | Self::RashnakBackstabber
                | Self::RipperBolgrot
                | Self::RodneyRoachbait
                | Self::RowanaForestfoot
                | Self::RoxannaDarknail
                | Self::RumbelowSheepskin
                | Self::ScrappaSorehead
                | Self::ScylaAnfingrimm
                | Self::SkitterStabStab
                | Self::SkrorgSnowpelt
                | Self::SkrullHalfheight
                | Self::SwiftvineGlimmershard
                | Self::TheBlackGobbo
                | Self::TheSwiftTwins
                | Self::ThorssonStoutmead
                | Self::VaragGhoulChewer
                | Self::WilhelmChaney
                | Self::WillowRosebark
                | Self::WithergraspDoubledrool
                | Self::ZolcathTheZoat
                | Self::ZzhargMadeye
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionDefinition {
    pub maximum_quantity: u8,
    pub cost: u32,
    pub characteristics: HashMap<Characteristic, u8>,
    pub skills: Vec<Skill>,
    pub primary_skill_categories: Vec<SkillCategory>,
    pub secondary_skill_categories: Vec<SkillCategory>,
    pub is_big_man: bool,
}

impl PositionDefinition {
    pub fn movement_allowance(&self) -> u8 {
        self.characteristics
            .get(&Characteristic::MovementAllowance)
            .unwrap()
            .clone()
    }

    pub fn strength(&self) -> u8 {
        self.characteristics
            .get(&Characteristic::Strength)
            .unwrap()
            .clone()
    }

    pub fn agility(&self) -> u8 {
        self.characteristics
            .get(&Characteristic::Agility)
            .unwrap()
            .clone()
    }

    pub fn armour_value(&self) -> u8 {
        self.characteristics
            .get(&Characteristic::ArmourValue)
            .unwrap()
            .clone()
    }

    pub fn passing_ability(&self) -> Option<u8> {
        self.characteristics
            .get(&Characteristic::PassingAbility)
            .and_then(|value| Some(*value))
    }

    pub fn primary_skill_categories_first_letter(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.primary_skill_categories.len());

        for skill_category in self.primary_skill_categories.clone() {
            names.push(skill_category.first_letter(lang_id));
        }

        names.join("")
    }

    pub fn secondary_skill_categories_first_letter(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.secondary_skill_categories.len());

        for skill_category in self.secondary_skill_categories.clone() {
            names.push(skill_category.first_letter(lang_id));
        }

        names.join("")
    }

    pub fn skills_names(&self, lang_id: &str) -> String {
        let mut names: Vec<String> = Vec::with_capacity(self.skills.len());

        for skill in self.skills.clone() {
            names.push(skill.name(lang_id));
        }

        names.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        let result = Position::Wardancer.name("en");
        assert_eq!(result, "Wardancer");

        let result = Position::LorenForestTreeman.name("fr");
        assert_eq!(result, "Loren Forest Treeman");
    }
}
