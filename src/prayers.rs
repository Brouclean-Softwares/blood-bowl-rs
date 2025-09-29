use crate::dices::Dice;
use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PrayerToNuffle {
    TreacherousTrapdoor,
    FriendsWithTheRef,
    Stiletto,
    IronMan,
    KnuckleDusters,
    BadHabits,
    GreasyCleats,
    BlessedStatueOfNuffle,
    MolesUnderThePitch,
    PerfectPassing,
    FanInteraction,
    NecessaryViolence,
    FoulingFrenzy,
    ThrowARock,
    UnderScrutiny,
    IntensiveTraining,
}

impl TypeName for PrayerToNuffle {}
impl TranslatedName for PrayerToNuffle {}

impl PrayerToNuffle {
    pub fn roll() -> Self {
        match Dice::D16.roll() {
            1 => Self::TreacherousTrapdoor,
            2 => Self::FriendsWithTheRef,
            3 => Self::Stiletto,
            4 => Self::IronMan,
            5 => Self::KnuckleDusters,
            6 => Self::BadHabits,
            7 => Self::GreasyCleats,
            8 => Self::BlessedStatueOfNuffle,
            9 => Self::MolesUnderThePitch,
            10 => Self::PerfectPassing,
            11 => Self::FanInteraction,
            12 => Self::NecessaryViolence,
            13 => Self::FoulingFrenzy,
            14 => Self::ThrowARock,
            15 => Self::UnderScrutiny,
            _ => Self::IntensiveTraining,
        }
    }

    pub fn list() -> Vec<Self> {
        vec![
            Self::TreacherousTrapdoor,
            Self::FriendsWithTheRef,
            Self::Stiletto,
            Self::IronMan,
            Self::KnuckleDusters,
            Self::BadHabits,
            Self::GreasyCleats,
            Self::BlessedStatueOfNuffle,
            Self::MolesUnderThePitch,
            Self::PerfectPassing,
            Self::FanInteraction,
            Self::NecessaryViolence,
            Self::FoulingFrenzy,
            Self::ThrowARock,
            Self::UnderScrutiny,
            Self::IntensiveTraining,
        ]
    }
}
