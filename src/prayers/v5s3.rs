use crate::prayers::PrayerToNuffle;

pub fn roll_prayer() -> PrayerToNuffle {
    super::v5::roll_prayer()
}

pub fn prayers_list() -> Vec<PrayerToNuffle> {
    vec![
        PrayerToNuffle::TreacherousTrapdoor,
        PrayerToNuffle::FriendsWithTheRef,
        PrayerToNuffle::Stiletto,
        PrayerToNuffle::IronMan,
        PrayerToNuffle::KnuckleDusters,
        PrayerToNuffle::BadHabits,
        PrayerToNuffle::GreasyCleats,
        PrayerToNuffle::BlessingsOfNuffle,
        PrayerToNuffle::MolesUnderThePitch,
        PrayerToNuffle::DazzlingCatching,
        PrayerToNuffle::PerfectPassing,
        PrayerToNuffle::FanInteraction,
        PrayerToNuffle::FoulingFrenzy,
        PrayerToNuffle::ThrowARock,
        PrayerToNuffle::UnderScrutiny,
        PrayerToNuffle::IntensiveTraining,
    ]
}
