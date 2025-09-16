use crate::players::Player;
use crate::teams::Team;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameEvent {
    // Pre-game sequence
    Fans {
        team: Team,
        number: u32,
    },
    Weather,
    JourneyMan {
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
    ExpensiveMistake,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Action {}
