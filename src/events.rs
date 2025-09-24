use crate::dices::Dice;
use crate::teams::Team;
use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameEvent {
    // Pre-game sequence
    FanFactor(Team, u32),
    Weather(Weather),
    /*JourneyMan {
        team: Team,
    },
    Inducement,
    PrayerToNuffle,
    KickingTeam {
        team: Team,
    },

    // Start of drive sequence
    SetUp,
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
pub enum Action {}
