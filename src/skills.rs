use crate::translation::{LOCALES, TranslatedName, TypeName, language_from};
use fluent_templates::Loader;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SkillCategory {
    Agility,
    General,
    Mutation,
    Pass,
    Strength,
    Trait,
}

impl TypeName for SkillCategory {}
impl TranslatedName for SkillCategory {}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
    Claw,
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
    Animosity,
    AnimalSavagery,
    BallChain,
    BloodLust,
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
}

impl TypeName for Skill {}

impl TranslatedName for Skill {
    fn name(&self, lang_id: &str) -> String {
        match self {
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
