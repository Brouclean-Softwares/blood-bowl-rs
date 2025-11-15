use crate::inducements::{Inducement, TreasuryAndPettyCash};
use crate::rosters::{Roster, SpecialRule};
use crate::staffs::Staff;
use crate::teams::Team;
use crate::{staffs, stars};

pub(crate) fn inducements_buyable_for_team(
    team: &Team,
    money_left: &TreasuryAndPettyCash,
) -> Vec<Inducement> {
    let mut inducements: Vec<Inducement> = vec![];

    let mut possible_inducements = vec![
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

    for star_position in stars::v5::star_position_list() {
        possible_inducements.push(Inducement::StarPlayer(star_position));
    }

    for megastar_position in stars::v5::mega_star_position_list() {
        possible_inducements.push(Inducement::MegaStarPlayer(megastar_position));
    }

    for famous_coaching_staff in staffs::v5::famous_coaching_staff_list() {
        possible_inducements.push(Inducement::FamousCoachingStaff(famous_coaching_staff));
    }

    for inducement in possible_inducements {
        if money_left.total() > inducement.price_for_team(team) as i32 {
            inducements.push(inducement);
        }
    }

    inducements
}

pub(crate) fn inducement_maximum_for_team(inducement: &Inducement, team: &Team) -> usize {
    match (inducement, team.roster, team.roster_definition()) {
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

        (Inducement::StarPlayer(_) | Inducement::MegaStarPlayer(_), _, _) => 1,

        (Inducement::FamousCoachingStaff(coaching_staff), roster, roster_definition) => {
            staffs::v5::famous_coaching_staff_maximum_for_team(
                coaching_staff,
                &roster,
                &roster_definition,
            )
        }

        (_, _, _) => 0,
    }
}

pub fn inducement_price_for_team(inducement: &Inducement, team: &Team) -> u32 {
    match (inducement, team.roster, team.roster_definition()) {
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
            if let Some(definition) = position.definition(team.version, roster) {
                definition.cost
            } else {
                0
            }
        }

        (Inducement::FamousCoachingStaff(famous_coaching_staff), _, _) => {
            staffs::v5::famous_coaching_staff_price(famous_coaching_staff)
        }

        (_, _, _) => 0,
    }
}
