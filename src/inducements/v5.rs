use crate::inducements::{Inducement, TreasuryAndPettyCash};
use crate::positions::Position;
use crate::rosters::{Roster, SpecialRule};
use crate::staffs::{FamousCoachingStaff, Staff};
use crate::teams::Team;

pub(crate) fn inducements_buyable_for_team(
    team: &Team,
    money_left: &TreasuryAndPettyCash,
) -> Vec<Inducement> {
    let mut inducements: Vec<Inducement> = vec![];

    for inducement in vec![
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
        Inducement::StarPlayer(Position::AkhorneTheSquirrel),
        Inducement::StarPlayer(Position::AnqiPanqi),
        Inducement::StarPlayer(Position::BarikFarblast),
        Inducement::StarPlayer(Position::BilerotVomitflesh),
        Inducement::StarPlayer(Position::BoaKonSsstriktr),
        Inducement::StarPlayer(Position::BryceTheSliceCambuel),
        Inducement::StarPlayer(Position::CaptainKarinaVonRiesz),
        Inducement::StarPlayer(Position::CountLuthorVonDrakenborg),
        Inducement::StarPlayer(Position::DriblAndDrull),
        Inducement::StarPlayer(Position::EldrilSidewinder),
        Inducement::StarPlayer(Position::EstelleLaVeneaux),
        Inducement::StarPlayer(Position::FrankNStein),
        Inducement::StarPlayer(Position::FungusTheLoon),
        Inducement::StarPlayer(Position::GlartSmashrip),
        Inducement::StarPlayer(Position::GlorielSummerbloom),
        Inducement::StarPlayer(Position::GlotlStop),
        Inducement::StarPlayer(Position::GrakAndCrumbleberry),
        Inducement::StarPlayer(Position::GrashnakBlackhoof),
        Inducement::StarPlayer(Position::GretchenWachterTheBloodBowlWidow),
        Inducement::StarPlayer(Position::GrimIronjaw),
        Inducement::StarPlayer(Position::GrombrindalTheWhiteDwarf),
        Inducement::StarPlayer(Position::GufflePussmaw),
        Inducement::StarPlayer(Position::HelmutWulf),
        Inducement::StarPlayer(Position::HTharkTheUnstoppable),
        Inducement::StarPlayer(Position::IvanTAnimalDeathshroud),
        Inducement::StarPlayer(Position::IvarEriksson),
        Inducement::StarPlayer(Position::JeremiahKool),
        Inducement::StarPlayer(Position::JordellFreshbreeze),
        Inducement::StarPlayer(Position::KarlaVonKill),
        Inducement::StarPlayer(Position::KirothKrakeneye),
        Inducement::StarPlayer(Position::LordBorakTheDespoiler),
        Inducement::StarPlayer(Position::MapleHighgrove),
        Inducement::StarPlayer(Position::MaxSpleenripper),
        Inducement::StarPlayer(Position::MightyZug),
        Inducement::StarPlayer(Position::NobblaBlackwart),
        Inducement::StarPlayer(Position::PuggyBaconbreath),
        Inducement::StarPlayer(Position::RashnakBackstabber),
        Inducement::StarPlayer(Position::RipperBolgrot),
        Inducement::StarPlayer(Position::RodneyRoachbait),
        Inducement::StarPlayer(Position::RowanaForestfoot),
        Inducement::StarPlayer(Position::RoxannaDarknail),
        Inducement::StarPlayer(Position::RumbelowSheepskin),
        Inducement::StarPlayer(Position::ScrappaSorehead),
        Inducement::StarPlayer(Position::ScylaAnfingrimm),
        Inducement::StarPlayer(Position::SkitterStabStab),
        Inducement::StarPlayer(Position::SkrorgSnowpelt),
        Inducement::StarPlayer(Position::SkrullHalfheight),
        Inducement::StarPlayer(Position::SwiftvineGlimmershard),
        Inducement::StarPlayer(Position::TheBlackGobbo),
        Inducement::StarPlayer(Position::TheSwiftTwins),
        Inducement::StarPlayer(Position::ThorssonStoutmead),
        Inducement::StarPlayer(Position::VaragGhoulChewer),
        Inducement::StarPlayer(Position::WilhelmChaney),
        Inducement::StarPlayer(Position::WillowRosebark),
        Inducement::StarPlayer(Position::WithergraspDoubledrool),
        Inducement::StarPlayer(Position::ZolcathTheZoat),
        Inducement::StarPlayer(Position::ZzhargMadeye),
        Inducement::MegaStarPlayer(Position::BomberDribblesnot),
        Inducement::MegaStarPlayer(Position::CindyPiewhistle),
        Inducement::MegaStarPlayer(Position::DeeprootStrongbranch),
        Inducement::MegaStarPlayer(Position::GriffOberwald),
        Inducement::MegaStarPlayer(Position::HakflemSkuttlespike),
        Inducement::MegaStarPlayer(Position::KreekTheVerminatorRustgouger),
        Inducement::MegaStarPlayer(Position::MorgNThorg),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::AyleenAndar),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::FinkDaFixer),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::GalandrilSilverwater),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::JosefBugman),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::KariColdsteel),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::KrotShockwhisker),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::MungoSpinecracker),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::PapaSkullbones),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::ProfessorFronkelheim),
        Inducement::FamousCoachingStaff(FamousCoachingStaff::SchielundScharlitan),
    ] {
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
        (Inducement::StarPlayer(_), _, _) => 1,
        (Inducement::MegaStarPlayer(_), _, _) => 1,
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::AyleenAndar), _, _) => 1,
        (
            Inducement::FamousCoachingStaff(FamousCoachingStaff::FinkDaFixer),
            _,
            Some(roster_definition),
        ) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::BadlandsBrawl)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::UnderworldChallenge)
            {
                1
            } else {
                0
            }
        }
        (
            Inducement::FamousCoachingStaff(FamousCoachingStaff::GalandrilSilverwater),
            _,
            Some(roster_definition),
        ) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::ElvenKingdomsLeague)
            {
                1
            } else {
                0
            }
        }
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::JosefBugman), _, _) => 1,
        (
            Inducement::FamousCoachingStaff(FamousCoachingStaff::KariColdsteel),
            _,
            Some(roster_definition),
        ) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::ElvenKingdomsLeague)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::LustrianSuperleague)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::OldWorldClassic)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::WorldsEdgeSuperleague)
            {
                1
            } else {
                0
            }
        }
        (
            Inducement::FamousCoachingStaff(FamousCoachingStaff::KrotShockwhisker),
            _,
            Some(roster_definition),
        ) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::UnderworldChallenge)
            {
                1
            } else {
                0
            }
        }
        (
            Inducement::FamousCoachingStaff(FamousCoachingStaff::MungoSpinecracker),
            _,
            Some(roster_definition),
        ) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::BadlandsBrawl)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::OldWorldClassic)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::UnderworldChallenge)
            {
                1
            } else {
                0
            }
        }
        (
            Inducement::FamousCoachingStaff(FamousCoachingStaff::PapaSkullbones),
            _,
            Some(roster_definition),
        ) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::FavouredOf)
                || roster_definition
                    .special_rules
                    .contains(&SpecialRule::UnderworldChallenge)
            {
                1
            } else {
                0
            }
        }
        (
            Inducement::FamousCoachingStaff(FamousCoachingStaff::ProfessorFronkelheim),
            _,
            Some(roster_definition),
        ) => {
            if roster_definition
                .special_rules
                .contains(&SpecialRule::SylvanianSpotlight)
            {
                1
            } else {
                0
            }
        }
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::SchielundScharlitan), _, Some(_)) => {
            1
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
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::AyleenAndar), _, _) => 100000,
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::FinkDaFixer), _, _) => 90000,
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::GalandrilSilverwater), _, _) => 40000,
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::JosefBugman), _, _) => 100000,
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::KariColdsteel), _, _) => 50000,
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::KrotShockwhisker), _, _) => 70000,
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::MungoSpinecracker), _, _) => 80000,
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::PapaSkullbones), _, _) => 80000,
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::ProfessorFronkelheim), _, _) => {
            130000
        }
        (Inducement::FamousCoachingStaff(FamousCoachingStaff::SchielundScharlitan), _, _) => 90000,
        (_, _, _) => 0,
    }
}
