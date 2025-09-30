use crate::actions::Action;
use crate::dices::Dice;
use crate::games::Game;
use crate::inducements::{Inducement, TreasuryAndPettyCash};
use crate::injuries::Injury;
use crate::prayers::PrayerToNuffle;
use crate::teams::Team;
use crate::weather::Weather;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GameEvent {
    // Pre-game sequence
    FanFactor {
        team_id: i32,
        fan_factor: u8,
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
    TossWinner {
        team_id: i32,
    },
    KickingTeam {
        team_id: i32,
    },

    // Start of drive sequence
    //SetUp,
    //KickOff,
    //KickOffEvent,

    // Team turns
    TurnStart {
        team_id: i32,
        number: usize,
    },
    Action {
        team_id: i32,
        player_id: i32,
        action: Action,
        star_player_points: usize,
    },
    Injury {
        team_id: i32,
        player_id: i32,
        injury: Injury,
    },
    //TurnEnd,
    //TurnOver,

    // End of drive sequence
    //SecretWeaponOut,
    //RecoverKnockedOut,

    // Post-game sequence
    GameEnd,
    /*Winnings {
        team_id: i32,
        money_earned: u32,
    },
    DedicatedFansUpdate {
        team_id: i32,
        delta: i8,
    },
    PlayerAdvancement,
    Hiring,
    Firing,
    TemporarilyRetiring,
    ExpensiveMistake,*/
}

impl GameEvent {
    pub fn roll_fan_factor(team: Team) -> u8 {
        team.dedicated_fans + Dice::D3.roll() as u8
    }

    pub fn roll_toss_winner(game: &Game) -> i32 {
        if Dice::D2.roll() <= 1 {
            game.first_team.id
        } else {
            game.second_team.id
        }
    }
}
