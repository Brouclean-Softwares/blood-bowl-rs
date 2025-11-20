use crate::errors::Error;
use crate::positions::Position;
use crate::rosters::Roster;
use crate::staffs::FamousCoachingStaff;
use crate::teams::Team;
use crate::translation::{LOCALES, TranslatedName, TypeName, language_from};
use crate::versions::Version;
use fluent_templates::Loader;
use serde::{Deserialize, Serialize};

pub mod v5;
pub mod v5s3;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    PrayersToNuffle,
    PartTimeAssistantCoaches,
    TempAgencyCheerleaders,
    TeamMascot,
    WeatherMage,
    BlitzersBestKegs,
    Bribes,
    ExtraTeamTraining,
    MortuaryAssistant,
    PlagueDoctor,
    RiotousRookies,
    WanderingApothecaries,
    HalflingMasterChef,
    StarPlayer(Position),
    MegaStarPlayer(Position),
    FamousCoachingStaff(FamousCoachingStaff),

    // Old
    BloodweiserKegs,
    SpecialPlays,
}

impl TypeName for Inducement {}

impl TranslatedName for Inducement {
    fn name(&self, lang_id: &str) -> String {
        match self {
            Inducement::StarPlayer(position) => format!("Star : {}", position.name(lang_id)),
            Inducement::MegaStarPlayer(position) => {
                format!("Mega Star : {}", position.name(lang_id))
            }
            Inducement::FamousCoachingStaff(coach) => format!("Coachs : {}", coach.name(lang_id)),
            _ => LOCALES.lookup(&language_from(lang_id), &*self.type_name()),
        }
    }
}

impl Inducement {
    pub fn list_available_for_roster(roster: &Roster, version: &Version) -> Vec<Self> {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => Vec::new(),
            Version::V5 => v5::list_available_for_roster(roster),
            Version::V5S3 => v5s3::list_available_for_roster(roster),
        }
    }

    pub fn list_buyable_for_team(team: &Team, money_left: &TreasuryAndPettyCash) -> Vec<Self> {
        let mut inducements = Self::list_available_for_roster(&team.roster, &team.version);
        inducements.retain(|inducement| {
            money_left.total() > inducement.price_for_roster(&team.roster, &team.version) as i32
        });
        inducements
    }

    pub fn maximum_for_roster(&self, roster: &Roster, version: &Version) -> usize {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => 0,
            Version::V5 => v5::inducement_maximum_for_roster(self, roster),
            Version::V5S3 => v5s3::inducement_maximum_for_roster(self, roster),
        }
    }

    pub fn maximum_for_team(&self, team: &Team) -> usize {
        self.maximum_for_roster(&team.roster, &team.version)
    }

    pub fn price_for_roster(&self, roster: &Roster, version: &Version) -> u32 {
        match version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => 0,
            Version::V5 => v5::inducement_price_for_roster(self, roster),
            Version::V5S3 => v5s3::inducement_price_for_roster(self, roster),
        }
    }

    pub fn price_for_team(&self, team: &Team) -> u32 {
        self.price_for_roster(&team.roster, &team.version)
    }
}
