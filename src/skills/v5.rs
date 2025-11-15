use crate::skills::{Skill, SkillCategory};

pub fn skills_to_be_added_for_category(skill_category: &SkillCategory) -> Vec<Skill> {
    match skill_category {
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
            Skill::Claws,
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

pub fn skill_category_for_skill(skill: &Skill) -> SkillCategory {
    match skill {
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
        | Skill::Claws
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
        | Skill::BloodLust(_)
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
