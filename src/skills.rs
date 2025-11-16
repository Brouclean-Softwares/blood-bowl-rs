use crate::players::Player;
use crate::positions::Position;
use crate::translation::{LOCALES, TranslatedName, TypeName, language_from};
use crate::versions::Version;
use fluent_templates::Loader;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

pub mod v5;
pub mod v5s3;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillCategory {
    Agility,
    General,
    Mutation,
    Pass,
    Strength,
    Trait,
    Special,
}

impl TypeName for SkillCategory {}
impl TranslatedName for SkillCategory {}

impl SkillCategory {
    pub fn skills_available_for_player(&self, player: &Player) -> Vec<Skill> {
        let mut skills = vec![];

        for skill in self.skills_to_be_added(&player.version) {
            if !player.skills().contains(&skill) {
                skills.push(skill)
            }
        }

        skills
    }

    pub fn skills_to_be_added(&self, version: &Version) -> Vec<Skill> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => Vec::new(),
            Version::V5 => v5::skills_to_be_added_for_category(self),
            Version::V5S3 => v5s3::skills_to_be_added_for_category(self),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Skill {
    // General
    Block,
    Dauntless,
    DirtyPlayer(u8),
    Fend,
    Frenzy,
    Kick,
    Pro,
    Shadowing,
    StripBall,
    SureHands,
    Tackle,
    Wrestle,

    // Agility
    Catch,
    Defensive,
    DivingCatch,
    DivingTackle,
    Dodge,
    JumpUp,
    Leap,
    SideStep,
    SafePairOfHands,
    SneakyGit,
    Sprint,
    SureFeet,

    // Strength
    ArmBar,
    Brawler,
    BreakTackle,
    Grab,
    Guard,
    Juggernaut,
    MightyBlow(u8),
    MultipleBlock,
    PileDriver,
    StandFirm,
    StrongArm,
    ThickSkull,

    // Passing
    Accurate,
    Cannoneer,
    CloudBurster,
    DumpOff,
    Fumblerooskie,
    HailMaryPass,
    Leader,
    NervesOfSteel,
    OnTheBall,
    Pass,
    RunningPass,
    SafePass,

    // Mutation
    BigHand,
    Claws,
    DisturbingPresence,
    ExtraArms,
    FoulAppearance,
    Horns,
    IronHardSkin,
    MonstrousMouth,
    PrehensileTail,
    Tentacles,
    TwoHeads,
    VeryLongLegs,

    // Trait
    AlwaysHungry,
    Animosity(Position),
    AnimalSavagery,
    BallChain,
    BloodLust(u8),
    Bombardier,
    BoneHead,
    BreatheFire,
    Chainsaw,
    Decay,
    Drunkard,
    HitAndRun,
    HypnoticGaze,
    KickTeamMate,
    Loner(u8),
    MyBall,
    NoHands,
    PickMeUp,
    PlagueRidden,
    PogoStick,
    ProjectileVomit,
    ReallyStupid,
    Regeneration,
    RightStuff,
    SecretWeapon,
    Stab,
    Stunty,
    Swarming,
    Swoop,
    TakeRoots,
    Timmmber,
    Trickster,
    ThrowTeamMate,
    Titchy,
    UnchannelledFury,

    // Special
    BlindRage,
    SavageBlow,
    BlastIt,
    PutridRegurgitation,
    LookIntoMyEyes,
    GhostlyFlames,
    TastyMorsel,
    StarOfTheShow,
    ASneakyPair,
    MesmerizingGaze,
    BalefulHex,
    BrutalBlock,
    WhirlingDervish,
    FrenziedRush,
    ShotToNothing,
    PrimalSavagery,
    TwoForOne,
    GoredByTheBull,
    Incorporeal,
    Slayer,
    WisdomOfTheWhiteDwarf,
    QuickBite,
    OldPro,
    UnstoppableMomentum,
    DwarvenScourge,
    RaidingParty,
    TheFlashingBlade,
    SwiftAsTheBreeze,
    Indomitable,
    BlackInk,
    LordOfChaos,
    ViciousVines,
    MaximumCarnage,
    CrushingBlow,
    KickThemWhileTheyAreDown,
    HalflingLuck,
    ToxinConnoisseur,
    ThinkingManTroll,
    CatchOfTheDay,
    BoundingLeap,
    BurstOfSpeed,
    Ram,
    Yoink,
    FuryOfTheBloodGod,
    MasterAssassin,
    PumpUpTheCrowd,
    StrongPassingGame,
    FuriousOutburst,
    SneakiestOfTheLot,
    BeerBarrelBash,
    SavageMauling,
    WatchOut,
    ExcuseMeAreYouAZoat,
    ThenIStartedBlastin,
    Kaboom,
    AllYouCanEat,
    Reliable,
    ConsummateProfessional,
    Treacherous,
    IllBeBack,
    TheBallista,
}

impl TypeName for Skill {}

impl TranslatedName for Skill {
    fn name(&self, lang_id: &str) -> String {
        match self {
            Skill::Animosity(position) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "Animosity",
                &HashMap::from([(Cow::from("position"), position.name(lang_id).into())]),
            ),
            Skill::BloodLust(value) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "BloodLust",
                &HashMap::from([(Cow::from("value"), value.into())]),
            ),
            Skill::DirtyPlayer(value) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "DirtyPlayer",
                &HashMap::from([(Cow::from("value"), value.into())]),
            ),
            Skill::MightyBlow(value) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "MightyBlow",
                &HashMap::from([(Cow::from("value"), value.into())]),
            ),
            Skill::Loner(value) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "Loner",
                &HashMap::from([(Cow::from("value"), value.into())]),
            ),
            _ => LOCALES.lookup(&language_from(lang_id), &*self.type_name()),
        }
    }
}

impl Skill {
    pub fn is_primary_for_player(&self, player: &Player) -> bool {
        if let Some(position_definition) = player.position_definition() {
            position_definition
                .primary_skill_categories
                .contains(&self.skill_category(&player.version))
        } else {
            false
        }
    }

    pub fn is_secondary_for_player(&self, player: &Player) -> bool {
        if let Some(position_definition) = player.position_definition() {
            position_definition
                .secondary_skill_categories
                .contains(&self.skill_category(&player.version))
        } else {
            false
        }
    }

    pub fn primary_list_available_for_player(player: &Player) -> Vec<Self> {
        let mut skills_available = vec![];

        if let Some(position_definition) = player.position_definition() {
            for skill_category in position_definition.primary_skill_categories.iter() {
                skills_available.push(skill_category.skills_available_for_player(player));
            }
        }

        skills_available.concat()
    }

    pub fn secondary_list_available_for_player(player: &Player) -> Vec<Self> {
        let mut skills_available = vec![];

        if let Some(position_definition) = player.position_definition() {
            for skill_category in position_definition.secondary_skill_categories.iter() {
                skills_available.push(skill_category.skills_available_for_player(player));
            }
        }

        skills_available.concat()
    }

    pub fn skill_category(&self, version: &Version) -> SkillCategory {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => SkillCategory::General,
            Version::V5 => v5::skill_category_for_skill(self),
            Version::V5S3 => v5s3::skill_category_for_skill(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_category_names() {
        let result = SkillCategory::Pass.name("en");
        assert_eq!(result, "Pass");

        let result = SkillCategory::Pass.name("fr");
        assert_eq!(result, "Passe");

        let result = SkillCategory::Strength.first_letter("en");
        assert_eq!(result, "S");

        let result = SkillCategory::Strength.first_letter("fr");
        assert_eq!(result, "F");
    }

    #[test]
    fn skill_names() {
        let result = Skill::Dauntless.name("en");
        assert_eq!(result, "Dauntless");

        let result = Skill::Dauntless.name("fr");
        assert_eq!(result, "Intrépide");
    }
}
