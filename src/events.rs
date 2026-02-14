use crate::actions::Success;
use crate::dices::Dice;
use crate::games::Game;
use crate::inducements::{Inducement, TreasuryAndPettyCash};
use crate::injuries::Injury;
use crate::positions::Keyword;
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
        used_money: TreasuryAndPettyCash,
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
    Success {
        team_id: i32,
        player_id: i32,
        success: Success,
        star_player_points: u32,
    },
    Injury {
        team_id: i32,
        player_id: i32,
        injury: Injury,
    },
    Hatred {
        team_id: i32,
        player_id: i32,
        keyword: Keyword,
    },
    SentOff {
        team_id: i32,
        player_id: i32,
    },
    //TurnEnd,
    //TurnOver,

    // End of drive sequence
    //SecretWeaponOut,
    //RecoverKnockedOut,
    HalfTime,

    // Post-game sequence
    GameEnd,
    Winnings {
        team_id: i32,
        earned_money: u32,
    },
    DedicatedFansUpdate {
        team_id: i32,
        delta: i8,
    },
    /*PlayerAdvancement,
    Hiring,
    Firing,
    TemporarilyRetiring,*/
    ExpensiveMistakes {
        team_id: i32,
        lost_money: u32,
    },

    GameClosure,
}

impl GameEvent {
    pub fn roll_fan_factor(team: &Team) -> u8 {
        team.dedicated_fans + Dice::D3.roll() as u8
    }

    pub fn roll_toss_winner(game: &Game) -> i32 {
        if Dice::D2.roll() <= 1 {
            game.first_team.id
        } else {
            game.second_team.id
        }
    }

    pub fn roll_dedicated_fans_delta(game: &Game, team: &Team) -> i8 {
        let dice_result = Dice::D6.roll();

        if let Some(winning_team) = game.winning_team() {
            if team.id.eq(&winning_team.id) && dice_result >= team.dedicated_fans as usize {
                1
            } else if team.id.ne(&winning_team.id) && dice_result < team.dedicated_fans as usize {
                -1
            } else {
                0
            }
        } else {
            0
        }
    }
}
