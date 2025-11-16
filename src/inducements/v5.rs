use crate::inducements::Inducement;
use crate::rosters::{Roster, SpecialRule};
use crate::staffs::{FamousCoachingStaff, Staff};
use crate::versions::Version;

const VERSION: Version = Version::V5;

pub(crate) fn list_available_for_roster(roster: &Roster) -> Vec<Inducement> {
    let mut inducements = vec![
        Inducement::TempAgencyCheerleaders,
        Inducement::PartTimeAssistantCoaches,
        Inducement::WeatherMage,
        Inducement::BloodweiserKegs,
        Inducement::SpecialPlays,
        Inducement::ExtraTeamTraining,
        Inducement::Bribes,
        Inducement::WanderingApothecaries,
        Inducement::MortuaryAssistant,
        Inducement::PlagueDoctor,
        Inducement::RiotousRookies,
        Inducement::HalflingMasterChef,
    ];

    for star_position in crate::stars::star_position_list(&VERSION) {
        inducements.push(Inducement::StarPlayer(star_position));
    }

    for megastar_position in crate::stars::mega_star_position_list(&VERSION) {
        inducements.push(Inducement::MegaStarPlayer(megastar_position));
    }

    for famous_coaching_staff in FamousCoachingStaff::list(&VERSION) {
        inducements.push(Inducement::FamousCoachingStaff(famous_coaching_staff));
    }

    inducements.retain(|inducement| inducement_maximum_for_roster(inducement, roster) > 0);

    inducements
}

pub(crate) fn inducement_maximum_for_roster(inducement: &Inducement, roster: &Roster) -> usize {
    match (inducement, roster, roster.definition(VERSION)) {
        (Inducement::TempAgencyCheerleaders, _, _) => 4,
        (Inducement::PartTimeAssistantCoaches, _, Some(_)) => 3,
        (Inducement::WeatherMage, _, _) => 1,
        (Inducement::BloodweiserKegs, _, _) => 2,
        (Inducement::SpecialPlays, _, _) => 5,
        (Inducement::ExtraTeamTraining, _, _) => 8,
        (Inducement::Bribes, _, _) => 3,
        (Inducement::WanderingApothecaries, _, Some(roster_definition)) => {
            if roster_definition.contains_staff(&Staff::Apothecary) {
                2
            } else {
                0
            }
        }
        (Inducement::MortuaryAssistant, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::SylvanianSpotlight)
            {
                1
            } else {
                0
            }
        }
        (Inducement::PlagueDoctor, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::FavouredOfNurgle)
            {
                1
            } else {
                0
            }
        }
        (Inducement::RiotousRookies, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::LowCostLinemen)
            {
                1
            } else {
                0
            }
        }
        (Inducement::HalflingMasterChef, _, _) => 1,

        (Inducement::StarPlayer(position) | Inducement::MegaStarPlayer(position), roster, _) => {
            crate::stars::star_maximum_for_roster(position, roster, &VERSION)
        }

        (Inducement::FamousCoachingStaff(coaching_staff), roster, _) => {
            coaching_staff.maximum_for_roster(&roster, &VERSION)
        }

        (_, _, _) => 0,
    }
}

pub fn inducement_price_for_roster(inducement: &Inducement, roster: &Roster) -> u32 {
    match (inducement, roster, roster.definition(VERSION)) {
        (Inducement::TempAgencyCheerleaders, _, _) => 20000,
        (Inducement::PartTimeAssistantCoaches, _, _) => 20000,
        (Inducement::WeatherMage, _, _) => 30000,
        (Inducement::BloodweiserKegs, _, _) => 50000,
        (Inducement::SpecialPlays, _, _) => 100000,
        (Inducement::ExtraTeamTraining, _, _) => 100000,
        (Inducement::Bribes, _, Some(roster_definition)) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::BriberyAndCorruption)
            {
                50000
            } else {
                100000
            }
        }
        (Inducement::WanderingApothecaries, _, _) => 100000,
        (Inducement::MortuaryAssistant, _, _) => 100000,
        (Inducement::PlagueDoctor, _, _) => 100000,
        (Inducement::RiotousRookies, _, _) => 100000,
        (Inducement::HalflingMasterChef, Roster::Halfling, Some(_)) => 100000,
        (Inducement::HalflingMasterChef, _, _) => 300000,

        (Inducement::StarPlayer(position), roster, _)
        | (Inducement::MegaStarPlayer(position), roster, _) => {
            if let Some(definition) = position.definition(VERSION, *roster) {
                definition.cost
            } else {
                0
            }
        }

        (Inducement::FamousCoachingStaff(famous_coaching_staff), _, _) => {
            famous_coaching_staff.price(&VERSION)
        }

        (_, _, _) => 0,
    }
}
