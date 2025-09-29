use crate::errors::Error;
use crate::positions::Position;
use crate::rosters::{Roster, SpecialRule, Staff};
use crate::teams::Team;
use crate::translation::{LOCALES, TranslatedName, TypeName, language_from};
use fluent_templates::Loader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TreasuryAndPettyCash {
    pub treasury: i32,
    pub petty_cash: u32,
}

impl TreasuryAndPettyCash {
    pub fn total(&self) -> i32 {
        self.treasury + self.petty_cash as i32
    }

    pub fn money_used_to_buy(&self, amount: u32) -> Result<Self, Error> {
        if self.petty_cash >= amount {
            Ok(Self {
                petty_cash: amount,
                treasury: 0,
            })
        } else if self.total() >= amount as i32 {
            Ok(Self {
                treasury: amount as i32 - self.petty_cash as i32,
                ..*self
            })
        } else {
            Err(Error::TreasuryExceeded)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Inducement {
    TempAgencyCheerleaders,
    PartTimeAssistantCoaches,
    WeatherMage,
    BloodweiserKegs,
    SpecialPlays,
    ExtraTeamTraining,
    Bribes,
    WanderingApothecaries,
    MortuaryAssistant,
    PlagueDoctor,
    RiotousRookies,
    HalflingMasterChef,
    StarPlayer(Position),
    MegaStarPlayer(Position),
}

impl TypeName for Inducement {}

impl TranslatedName for Inducement {
    fn name(&self, lang_id: &str) -> String {
        match self {
            Inducement::StarPlayer(position) => format!("Star : {}", position.name(lang_id)),
            Inducement::MegaStarPlayer(position) => {
                format!("Mega Star : {}", position.name(lang_id))
            }
            _ => LOCALES.lookup(&language_from(lang_id), &*self.type_name()),
        }
    }
}

impl Inducement {
    pub(crate) fn list_buyable_for_team(
        team: &Team,
        money_left: &TreasuryAndPettyCash,
    ) -> Vec<Self> {
        let mut inducements: Vec<Self> = vec![];

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
        ] {
            if money_left.total() > inducement.price_for_team(team) as i32 {
                inducements.push(inducement);
            }
        }

        inducements
    }

    pub(crate) fn maximum_for_team(&self, team: &Team) -> usize {
        match (self, team.roster, team.roster_definition()) {
            (Inducement::TempAgencyCheerleaders, _, Ok(_)) => 4,
            (Inducement::PartTimeAssistantCoaches, _, Ok(_)) => 3,
            (Inducement::WeatherMage, _, Ok(_)) => 1,
            (Inducement::BloodweiserKegs, _, _) => 2,
            (Inducement::SpecialPlays, _, Ok(_)) => 5,
            (Inducement::ExtraTeamTraining, _, Ok(_)) => 8,
            (Inducement::Bribes, _, Ok(_)) => 3,
            (Inducement::WanderingApothecaries, _, Ok(roster_definition)) => {
                if roster_definition
                    .staff_information
                    .contains_key(&Staff::Apothecary)
                {
                    2
                } else {
                    0
                }
            }
            (Inducement::MortuaryAssistant, _, Ok(roster_definition)) => {
                if roster_definition
                    .special_rules
                    .contains(&SpecialRule::SylvanianSpotlight)
                {
                    1
                } else {
                    0
                }
            }
            (Inducement::PlagueDoctor, _, Ok(roster_definition)) => {
                if roster_definition
                    .special_rules
                    .contains(&SpecialRule::FavouredOfNurgle)
                {
                    1
                } else {
                    0
                }
            }
            (Inducement::RiotousRookies, _, Ok(roster_definition)) => {
                if roster_definition
                    .special_rules
                    .contains(&SpecialRule::LowCostLinemen)
                {
                    1
                } else {
                    0
                }
            }
            (Inducement::HalflingMasterChef, _, Ok(_)) => 1,
            (Inducement::StarPlayer(_), _, Ok(_)) => 1,
            (Inducement::MegaStarPlayer(_), _, Ok(_)) => 1,
            (_, _, Err(_)) => 0,
        }
    }

    pub fn price_for_team(&self, team: &Team) -> u32 {
        match (self, team.roster, team.roster_definition()) {
            (Inducement::TempAgencyCheerleaders, _, Ok(_)) => 20000,
            (Inducement::PartTimeAssistantCoaches, _, Ok(_)) => 20000,
            (Inducement::WeatherMage, _, Ok(_)) => 30000,
            (Inducement::BloodweiserKegs, _, Ok(_)) => 50000,
            (Inducement::SpecialPlays, _, Ok(_)) => 100000,
            (Inducement::ExtraTeamTraining, _, Ok(_)) => 100000,
            (Inducement::Bribes, _, Ok(roster_definition)) => {
                if roster_definition
                    .special_rules
                    .contains(&SpecialRule::BriberyAndCorruption)
                {
                    50000
                } else {
                    100000
                }
            }
            (Inducement::WanderingApothecaries, _, Ok(_)) => 100000,
            (Inducement::MortuaryAssistant, _, Ok(_)) => 100000,
            (Inducement::PlagueDoctor, _, Ok(_)) => 100000,
            (Inducement::RiotousRookies, _, Ok(_)) => 100000,
            (Inducement::HalflingMasterChef, Roster::Halfling, Ok(_)) => 100000,
            (Inducement::HalflingMasterChef, _, Ok(_)) => 300000,
            (Inducement::StarPlayer(position), _, Ok(_)) => match position {
                Position::AkhorneTheSquirrel => 80000,
                Position::AnqiPanqi => 190000,
                Position::BarikFarblast => 80000,
                Position::BilerotVomitflesh => 180000,
                Position::BoaKonSsstriktr => 200000,
                Position::BryceTheSliceCambuel => 130000,
                Position::CaptainKarinaVonRiesz => 230000,
                Position::CountLuthorVonDrakenborg => 340000,
                Position::DriblAndDrull => 190000,
                Position::EldrilSidewinder => 230000,
                Position::EstelleLaVeneaux => 190000,
                Position::FrankNStein => 250000,
                Position::FungusTheLoon => 80000,
                Position::GlartSmashrip => 195000,
                Position::GlorielSummerbloom => 150000,
                Position::GlotlStop => 270000,
                Position::GrakAndCrumbleberry => 250000,
                Position::GrashnakBlackhoof => 240000,
                Position::GretchenWachterTheBloodBowlWidow => 260000,
                Position::GrimIronjaw => 200000,
                Position::GrombrindalTheWhiteDwarf => 210000,
                Position::GufflePussmaw => 180000,
                Position::HelmutWulf => 140000,
                Position::HTharkTheUnstoppable => 300000,
                Position::IvanTAnimalDeathshroud => 190000,
                Position::IvarEriksson => 245000,
                Position::JeremiahKool => 320000,
                Position::JordellFreshbreeze => 250000,
                Position::KarlaVonKill => 210000,
                Position::KirothKrakeneye => 160000,
                Position::LordBorakTheDespoiler => 260000,
                Position::MapleHighgrove => 210000,
                Position::MaxSpleenripper => 130000,
                Position::MightyZug => 220000,
                Position::NobblaBlackwart => 120000,
                Position::PuggyBaconbreath => 120000,
                Position::RashnakBackstabber => 130000,
                Position::RipperBolgrot => 250000,
                Position::RodneyRoachbait => 70000,
                Position::RowanaForestfoot => 160000,
                Position::RoxannaDarknail => 270000,
                Position::RumbelowSheepskin => 170000,
                Position::ScrappaSorehead => 130000,
                Position::ScylaAnfingrimm => 200000,
                Position::SkitterStabStab => 150000,
                Position::SkrorgSnowpelt => 250000,
                Position::SkrullHalfheight => 150000,
                Position::SwiftvineGlimmershard => 110000,
                Position::TheBlackGobbo => 225000,
                Position::TheSwiftTwins => 340000,
                Position::ThorssonStoutmead => 170000,
                Position::VaragGhoulChewer => 280000,
                Position::WilhelmChaney => 220000,
                Position::WillowRosebark => 150000,
                Position::WithergraspDoubledrool => 170000,
                Position::ZolcathTheZoat => 230000,
                Position::ZzhargMadeye => 130000,
                _ => 0,
            },
            (Inducement::MegaStarPlayer(position), _, Ok(_)) => match position {
                Position::BomberDribblesnot => 50000,
                Position::CindyPiewhistle => 50000,
                Position::DeeprootStrongbranch => 280000,
                Position::GriffOberwald => 280000,
                Position::HakflemSkuttlespike => 210000,
                Position::KreekTheVerminatorRustgouger => 170000,
                Position::MorgNThorg => 380000,
                _ => 0,
            },
            (_, _, Err(_)) => 0,
        }
    }
}
