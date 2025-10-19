use crate::errors::Error;
use crate::players::Player;
use crate::positions::Position;
use crate::translation::{LOCALES, TranslatedName, TypeName, language_from};
use fluent_templates::Loader;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
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

impl SkillCategory {
    pub fn skills_to_be_added(&self) -> Vec<Skill> {
        match self {
            SkillCategory::General => vec![
                Skill::Block,
                Skill::Dauntless,
                Skill::DirtyPlayer(1),
                Skill::Fend,
                Skill::Frenzy,
                Skill::Kick,
                Skill::Pro,
                Skill::Shadowing,
                Skill::StripBall,
                Skill::SureHands,
                Skill::Tackle,
                Skill::Wrestle,
            ],
            SkillCategory::Agility => vec![
                Skill::Catch,
                Skill::Defensive,
                Skill::DivingCatch,
                Skill::DivingTackle,
                Skill::Dodge,
                Skill::JumpUp,
                Skill::Leap,
                Skill::SideStep,
                Skill::SafePairOfHands,
                Skill::SneakyGit,
                Skill::Sprint,
                Skill::SureFeet,
            ],
            SkillCategory::Strength => vec![
                Skill::ArmBar,
                Skill::Brawler,
                Skill::BreakTackle,
                Skill::Grab,
                Skill::Guard,
                Skill::Juggernaut,
                Skill::MightyBlow(1),
                Skill::MultipleBlock,
                Skill::PileDriver,
                Skill::StandFirm,
                Skill::StrongArm,
                Skill::ThickSkull,
            ],
            SkillCategory::Pass => vec![
                Skill::Accurate,
                Skill::Cannoneer,
                Skill::CloudBurster,
                Skill::DumpOff,
                Skill::Fumblerooskie,
                Skill::HailMaryPass,
                Skill::Leader,
                Skill::NervesOfSteel,
                Skill::OnTheBall,
                Skill::Pass,
                Skill::RunningPass,
                Skill::SafePass,
            ],
            SkillCategory::Mutation => vec![
                Skill::BigHand,
                Skill::Claw,
                Skill::DisturbingPresence,
                Skill::ExtraArms,
                Skill::FoulAppearance,
                Skill::Horns,
                Skill::IronHardSkin,
                Skill::MonstrousMouth,
                Skill::PrehensileTail,
                Skill::Tentacles,
                Skill::TwoHeads,
                Skill::VeryLongLegs,
            ],
            SkillCategory::Trait => vec![],
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
    Animosity(Position),
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
            Skill::Animosity(position) => LOCALES.lookup_with_args(
                &language_from(lang_id),
                "Animosity",
                &HashMap::from([(Cow::from("position"), position.name(lang_id).into())]),
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
    pub fn is_primary_for_player(&self, player: &Player) -> Result<bool, Error> {
        Ok(player
            .position
            .definition(player.version, player.roster)?
            .primary_skill_categories
            .contains(&self.skill_category()))
    }

    pub fn is_secondary_for_player(&self, player: &Player) -> Result<bool, Error> {
        Ok(player
            .position
            .definition(player.version, player.roster)?
            .secondary_skill_categories
            .contains(&self.skill_category()))
    }

    pub fn skill_category(&self) -> SkillCategory {
        match self {
            Skill::Block
            | Skill::Dauntless
            | Skill::DirtyPlayer(_)
            | Skill::Fend
            | Skill::Frenzy
            | Skill::Kick
            | Skill::Pro
            | Skill::Shadowing
            | Skill::StripBall
            | Skill::SureHands
            | Skill::Tackle
            | Skill::Wrestle => SkillCategory::General,

            Skill::Catch
            | Skill::Defensive
            | Skill::DivingCatch
            | Skill::DivingTackle
            | Skill::Dodge
            | Skill::JumpUp
            | Skill::Leap
            | Skill::SideStep
            | Skill::SafePairOfHands
            | Skill::SneakyGit
            | Skill::Sprint
            | Skill::SureFeet => SkillCategory::Agility,

            Skill::ArmBar
            | Skill::Brawler
            | Skill::BreakTackle
            | Skill::Grab
            | Skill::Guard
            | Skill::Juggernaut
            | Skill::MightyBlow(_)
            | Skill::MultipleBlock
            | Skill::PileDriver
            | Skill::StandFirm
            | Skill::StrongArm
            | Skill::ThickSkull => SkillCategory::Strength,

            Skill::Accurate
            | Skill::Cannoneer
            | Skill::CloudBurster
            | Skill::DumpOff
            | Skill::Fumblerooskie
            | Skill::HailMaryPass
            | Skill::Leader
            | Skill::NervesOfSteel
            | Skill::OnTheBall
            | Skill::Pass
            | Skill::RunningPass
            | Skill::SafePass => SkillCategory::Pass,

            Skill::BigHand
            | Skill::Claw
            | Skill::DisturbingPresence
            | Skill::ExtraArms
            | Skill::FoulAppearance
            | Skill::Horns
            | Skill::IronHardSkin
            | Skill::MonstrousMouth
            | Skill::PrehensileTail
            | Skill::Tentacles
            | Skill::TwoHeads
            | Skill::VeryLongLegs
            | Skill::AlwaysHungry
            | Skill::Animosity(_)
            | Skill::AnimalSavagery
            | Skill::BallChain
            | Skill::BloodLust
            | Skill::Bombardier
            | Skill::BoneHead
            | Skill::BreatheFire
            | Skill::Chainsaw
            | Skill::Decay
            | Skill::Drunkard
            | Skill::HitAndRun
            | Skill::HypnoticGaze
            | Skill::KickTeamMate
            | Skill::Loner(_)
            | Skill::MyBall
            | Skill::NoHands
            | Skill::PickMeUp
            | Skill::PlagueRidden
            | Skill::PogoStick
            | Skill::ProjectileVomit
            | Skill::ReallyStupid
            | Skill::Regeneration
            | Skill::RightStuff
            | Skill::SecretWeapon
            | Skill::Stab
            | Skill::Stunty
            | Skill::Swarming
            | Skill::Swoop
            | Skill::TakeRoots
            | Skill::Timmmber
            | Skill::Trickster
            | Skill::ThrowTeamMate
            | Skill::Titchy
            | Skill::UnchannelledFury => SkillCategory::Trait,
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
