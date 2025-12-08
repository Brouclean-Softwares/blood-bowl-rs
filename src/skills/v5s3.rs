use crate::skills::{Skill, SkillCategory};

pub fn skills_to_be_added_for_category(skill_category: &SkillCategory) -> Vec<Skill> {
    match skill_category {
        SkillCategory::General => vec![
            Skill::Block,
            Skill::Dauntless,
            Skill::Fend,
            Skill::Frenzy,
            Skill::Kick,
            Skill::Pro,
            Skill::SteadyFooting,
            Skill::StripBall,
            Skill::SureHands,
            Skill::Tackle,
            Skill::Taunt,
            Skill::Wrestle,
        ],
        SkillCategory::Agility => vec![
            Skill::Catch,
            Skill::Defensive,
            Skill::DivingCatch,
            Skill::DivingTackle,
            Skill::Dodge,
            Skill::HitAndRun,
            Skill::JumpUp,
            Skill::Leap,
            Skill::SideStep,
            Skill::SafePairOfHands,
            Skill::Sprint,
            Skill::SureFeet,
        ],
        SkillCategory::Devious => vec![
            Skill::DirtyPlayer,
            Skill::EyeGouge,
            Skill::Fumblerooski,
            Skill::LethalFlight,
            Skill::LoneFouler,
            Skill::PileDriver,
            Skill::PutTheBootIn,
            Skill::QuickFoul,
            Skill::Saboteur,
            Skill::Shadowing,
            Skill::SneakyGit,
            Skill::ViolentInnovator,
        ],
        SkillCategory::Strength => vec![
            Skill::ArmBar,
            Skill::Brawler,
            Skill::BreakTackle,
            Skill::BullsEye,
            Skill::Grab,
            Skill::Guard,
            Skill::Juggernaut,
            Skill::MightyBlow,
            Skill::MultipleBlock,
            Skill::StandFirm,
            Skill::StrongArm,
            Skill::ThickSkull,
        ],
        SkillCategory::Pass => vec![
            Skill::Accurate,
            Skill::Cannoneer,
            Skill::CloudBurster,
            Skill::DumpOff,
            Skill::GiveAndGo,
            Skill::HailMaryPass,
            Skill::Leader,
            Skill::NervesOfSteel,
            Skill::OnTheBall,
            Skill::Pass,
            Skill::Punt,
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
        SkillCategory::Special => vec![],
    }
}

pub fn skill_category_for_skill(skill: &Skill) -> Option<SkillCategory> {
    match skill {
        Skill::Block
        | Skill::Dauntless
        | Skill::Fend
        | Skill::Frenzy
        | Skill::Kick
        | Skill::Pro
        | Skill::SteadyFooting
        | Skill::StripBall
        | Skill::SureHands
        | Skill::Tackle
        | Skill::Taunt
        | Skill::Wrestle => Some(SkillCategory::General),

        Skill::Catch
        | Skill::Defensive
        | Skill::DivingCatch
        | Skill::DivingTackle
        | Skill::Dodge
        | Skill::HitAndRun
        | Skill::JumpUp
        | Skill::Leap
        | Skill::SideStep
        | Skill::SafePairOfHands
        | Skill::Sprint
        | Skill::SureFeet => Some(SkillCategory::Agility),

        Skill::DirtyPlayer
        | Skill::EyeGouge
        | Skill::Fumblerooski
        | Skill::LethalFlight
        | Skill::LoneFouler
        | Skill::PileDriver
        | Skill::PutTheBootIn
        | Skill::QuickFoul
        | Skill::Saboteur
        | Skill::Shadowing
        | Skill::SneakyGit
        | Skill::ViolentInnovator => Some(SkillCategory::Devious),

        Skill::ArmBar
        | Skill::Brawler
        | Skill::BreakTackle
        | Skill::BullsEye
        | Skill::Grab
        | Skill::Guard
        | Skill::Juggernaut
        | Skill::MightyBlow
        | Skill::MultipleBlock
        | Skill::StandFirm
        | Skill::StrongArm
        | Skill::ThickSkull => Some(SkillCategory::Strength),

        Skill::Accurate
        | Skill::Cannoneer
        | Skill::CloudBurster
        | Skill::DumpOff
        | Skill::GiveAndGo
        | Skill::HailMaryPass
        | Skill::Leader
        | Skill::NervesOfSteel
        | Skill::OnTheBall
        | Skill::Pass
        | Skill::Punt
        | Skill::SafePass => Some(SkillCategory::Pass),

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
        | Skill::VeryLongLegs => Some(SkillCategory::Mutation),

        Skill::AlwaysHungry
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
        | Skill::Hatred(_)
        | Skill::HypnoticGaze
        | Skill::Insignificant
        | Skill::KickTeamMate
        | Skill::Loner(_)
        | Skill::MyBall
        | Skill::NoBall
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
        | Skill::Swoop
        | Skill::TakeRoots
        | Skill::Timmmber
        | Skill::Trickster
        | Skill::ThrowTeamMate
        | Skill::Titchy
        | Skill::UnchannelledFury
        | Skill::Unsteady => Some(SkillCategory::Trait),

        Skill::BlindRage
        | Skill::SavageBlow
        | Skill::BlastIt
        | Skill::PutridRegurgitation
        | Skill::LookIntoMyEyes
        | Skill::GhostlyFlames
        | Skill::TastyMorsel
        | Skill::StarOfTheShow
        | Skill::ASneakyPair
        | Skill::MesmerizingGaze
        | Skill::BalefulHex
        | Skill::BrutalBlock
        | Skill::WhirlingDervish
        | Skill::FrenziedRush
        | Skill::ShotToNothing
        | Skill::PrimalSavagery
        | Skill::TwoForOne
        | Skill::GoredByTheBull
        | Skill::Incorporeal
        | Skill::Slayer
        | Skill::WisdomOfTheWhiteDwarf
        | Skill::QuickBite
        | Skill::OldPro
        | Skill::UnstoppableMomentum
        | Skill::DwarvenScourge
        | Skill::RaidingParty
        | Skill::TheFlashingBlade
        | Skill::SwiftAsTheBreeze
        | Skill::DwarfenGrit
        | Skill::Indomitable
        | Skill::BlackInk
        | Skill::LordOfChaos
        | Skill::ViciousVines
        | Skill::MaximumCarnage
        | Skill::CrushingBlow
        | Skill::KickThemWhileTheyAreDown
        | Skill::HalflingLuck
        | Skill::ToxinConnoisseur
        | Skill::ThinkingManTroll
        | Skill::CatchOfTheDay
        | Skill::BoundingLeap
        | Skill::SlashingNails
        | Skill::Ram
        | Skill::Yoink
        | Skill::FuryOfTheBloodGod
        | Skill::MasterAssassin
        | Skill::PumpUpTheCrowd
        | Skill::StrongPassingGame
        | Skill::FuriousOutburst
        | Skill::SneakiestOfTheLot
        | Skill::WorkingInTandem
        | Skill::BeerBarrelBash
        | Skill::KrumpAndSmash
        | Skill::SavageMauling
        | Skill::WoodlandFury
        | Skill::WatchOut
        | Skill::ExcuseMeAreYouAZoat
        | Skill::BlastingSolvesEverything
        | Skill::Kaboom
        | Skill::AllYouCanEat
        | Skill::Reliable
        | Skill::ConsummateProfessional
        | Skill::Treacherous
        | Skill::IllBeBack
        | Skill::TheBallista => Some(SkillCategory::Special),

        _ => None,
    }
}

pub fn is_skill_elite(skill: &Skill) -> bool {
    match skill {
        Skill::Block | Skill::Dodge | Skill::Guard | Skill::MightyBlow => true,

        _ => false,
    }
}
