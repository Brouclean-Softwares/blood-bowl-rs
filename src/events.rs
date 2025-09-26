use crate::dices::Dice;
use crate::errors::Error;
use crate::games::Game;
use crate::rosters::{Roster, SpecialRule, Staff};
use crate::teams::Team;
use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameEvent {
    // Pre-game sequence
    FanFactor {
        team_id: i32,
        fan_factor: u32,
    },
    Weather(Weather),
    Journeyman {
        team_id: i32,
    },
    BuyInducement {
        team_id: i32,
        inducement: Inducement,
        money_used: TreasuryAndPettyCash,
    },
    PrayerToNuffle {
        team_id: i32,
        prayer_to_nuffle: PrayerToNuffle,
    },
    KickingTeam {
        team_id: i32,
    },
    // Start of drive sequence
    /*SetUp,
    KickOff,
    KickOffEvent,

    // Team turns
    TurnStart,
    Action {
        team: Team,
        player: Player,
        action: Action,
        star_player_points: u8,
    },
    TurnEnd,
    TurnOver,

    // End of drive sequence
    SecretWeaponOut,
    RecoverKnockedOut,

    // Post-game sequence
    Winnings {
        team: Team,
        money_earned: u32,
    },
    DedicatedFansUpdate {
        team: Team,
        delta: u8,
    },
    PlayerAdvancement,
    Hiring,
    Firing,
    TemporarilyRetiring,
    ExpensiveMistake,*/
}

impl GameEvent {
    pub fn roll_fan_factor(team: &Team) -> u32 {
        (team.dedicated_fans as u32 + Dice::D3.roll() as u32) * 10000
    }

    pub fn roll_kicking_team(game: &Game) -> i32 {
        if Dice::D2.roll() <= 1 {
            game.first_team.id
        } else {
            game.second_team.id
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Weather {
    SwelteringHeat,
    VerySunny,
    PerfectConditions,
    PouringRain,
    Blizzard,
}

impl TypeName for Weather {}
impl TranslatedName for Weather {}

impl Weather {
    pub fn roll() -> Self {
        match Dice::D6x2.roll() {
            2 => Self::SwelteringHeat,
            3 => Self::VerySunny,
            11 => Self::PouringRain,
            12 => Self::Blizzard,
            _ => Self::PerfectConditions,
        }
    }
}

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
}

impl TypeName for Inducement {}
impl TranslatedName for Inducement {}

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
            (_, _, Err(_)) => 0,
        }
    }
}

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
}
