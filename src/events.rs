use crate::dices::Dice;
use crate::games::Game;
use crate::inducements::{Inducement, TreasuryAndPettyCash};
use crate::prayers::PrayerToNuffle;
use crate::teams::Team;
use crate::weather::Weather;
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
