use crate::characteristics::Characteristic;
use crate::players::PlayerType;
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};
use crate::staffs::FamousCoachingStaff;
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod v5;
pub mod v5s3;

#[derive(sqlx::Type, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[sqlx(type_name = "varchar")]
pub enum Position {
    // Keyword
    All,
    BigGuy,
    Dwarfs,
    Halflings,
    Humans,
    Troll,
    Undead,

    // Common
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

    // Bretonnian
    Squires,
    KnightCatcher,
    KnightThrower,
    GrailKnight,

    // Black Orc
    GoblinBruiserLineman,
    BlackOrc,
    TrainedTroll,

    // Chaos Chosen
    BeastmanRunnerLineman,
    ChosenBlocker,
    ChaosTroll,
    ChaosOgre,
    Minotaur,

    // Chaos Dwarf
    HobgoblinLineman,
    HobgoblinSneakyStabba,
    ChaosDwarfBlocker,
    ChaosDwarfFlamesmith,
    BullCentaurBlitzer,

    // Chaos Renegade
    RenegadeHumanLineman,
    RenegadeHumanThrower,
    RenegadeGoblin,
    RenegadeOrc,
    RenegadeSkaven,
    RenegadeDarkElf,
    RenegadeTroll,
    RenegadeOgre,
    RenegadeMinotaur,
    RenegadeRatOgre,

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

    // Goblin
    GoblinLineman,
    Bomma,
    Looney,
    Fanatic,
    Pogoer,
    Ooligan,
    DoomDiver,

    // Halfling
    HalflingHopefulLineman,
    HalflingHefty,
    HalflingCatcher,

    // Human
    HumanLineman,
    HalflingHopeful,
    Ogre,

    // Imperial Nobility
    ImperialRetainerLineman,
    ImperialThrower,
    NobleBlitzer,
    Bodyguard,

    // Khorne
    BloodbornMarauderLineman,
    Khorngor,
    Bloodseeker,
    Bloodspawn,

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

    // Nurgle
    RotterLineman,
    Pestigor,
    Bloater,
    Rotspawn,

    // Ogre
    GnoblarLineman,
    OgreRuntPunter,
    OgreBlocker,

    // Old World Alliance
    OldWorldHumanLineman,
    OldWorldHumanThrower,
    OldWorldHumanCatcher,
    OldWorldDwarfBlitzer,
    OldWorldDwarfBlocker,
    OldWorldDwarfRunner,
    OldWorldHumanBlitzer,
    OldWorldDwarfTrollSlayer,
    OldWorldHalflingHopefuls,

    // Orc
    OrcLineman,
    BigUn,
    Goblin,
    UntrainedTroll,

    // Shambling Undead
    SkeletonLineman,
    WightBlitzer,
    Mummy,

    // Skaven
    SkavenClanratLineman,
    GutterRunner,
    RatOgre,

    // Snotling
    SnotlingLineman,
    FungusFlinga,
    FunHoppa,
    StiltyRunna,
    PumpWagon,

    // Tomb Kings
    AnointedThrower,
    AnointedBlitzer,
    TombGuardian,

    // Underworld Denizens
    UnderworldGoblinLineman,
    UnderworldSnotling,
    SkavenClanrat,
    SkavenThrower,
    SkavenBlitzer,
    UnderworldTroll,
    MutantRatOgre,

    // Vampire
    ThrallLineman,
    VampireRunner,
    VampireBlitzer,
    VampireThrower,
    Vargheist,

    // Wood Elf
    WoodElfLineman,
    Wardancer,
    LorenForestTreeman,

    // Others
    Journeyman,

    // Staff
    JosefBugman,
    KariColdsteel,

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
    Dribl,
    Drull,
    EldrilSidewinder,
    EstelleLaVeneaux,
    FrankNStein,
    FungusTheLoon,
    GlartSmashrip,
    GlorielSummerbloom,
    GlotlStop,
    GrakAndCrumbleberry,
    Grak,
    Crumbleberry,
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
    LucienSwift,
    ValenSwift,
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
    pub fn positions_for_players(&self) -> Vec<Self> {
        match self {
            Position::DriblAndDrull => vec![Position::Dribl, Position::Drull],
            Position::GrakAndCrumbleberry => vec![Position::Grak, Position::Crumbleberry],
            Position::TheSwiftTwins => vec![Position::LucienSwift, Position::ValenSwift],
            position => vec![position.clone()],
        }
    }

    pub fn definition(&self, version: Version, roster: Roster) -> Option<PositionDefinition> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => None,
            Version::V5 => v5::positon_definition_from(&roster, self),
            Version::V5S3 => v5s3::positon_definition_from(&roster, self),
        }
    }

    pub fn player_type(&self, version: &Version) -> PlayerType {
        if crate::stars::star_position_list(version).contains(self) {
            return PlayerType::Star;
        }

        if crate::stars::mega_star_position_list(version).contains(self) {
            return PlayerType::MegaStar;
        }

        if FamousCoachingStaff::list(version)
            .iter()
            .filter(|&famous_coaching_staff| famous_coaching_staff.position().eq(&Some(*self)))
            .count()
            > 0
        {
            return PlayerType::FamousCoachingStaff;
        }

        if matches!(self, Position::Journeyman) {
            return PlayerType::Journeyman;
        }

        PlayerType::FromRoster
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
    pub fn characteristic_value(&self, characteristic: Characteristic) -> Option<u8> {
        self.characteristics
            .get(&characteristic)
            .map(|value| value.clone())
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

        let result = Position::Wardancer.name("fr");
        assert_eq!(result, "Danseur de Guerre");
    }
}
