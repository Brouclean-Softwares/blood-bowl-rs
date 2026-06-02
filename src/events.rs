use crate::actions::Success;
use crate::dices::Dice;
use crate::errors::Error;
use crate::games::Game;
use crate::inducements::{Inducement, TreasuryAndPettyCash};
use crate::injuries::Injury;
use crate::positions::{Keyword, Position};
use crate::prayers::PrayerToNuffle;
use crate::teams::Team;
use crate::versions::Version;
use crate::weather::Weather;
use serde::{Deserialize, Serialize};

pub struct GameEventWithScoreAndCasualties {
    pub game_event: GameEvent,
    pub score: (usize, usize),
    pub casualties: (usize, usize),
}

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
    ExtraTime,
    Penalties {
        first_team_score: usize,
        second_team_score: usize,
    },

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

impl Game {
    pub fn events_sequence_with_score_and_casualties(
        &self,
    ) -> Vec<GameEventWithScoreAndCasualties> {
        let mut events_with_score_and_casualties: Vec<GameEventWithScoreAndCasualties> =
            Vec::with_capacity(self.events.len());
        let mut score = (0, 0);
        let mut casualties = (0, 0);

        for event in self.events.iter() {
            match event {
                GameEvent::Success {
                    team_id,
                    success: Success::Touchdown,
                    ..
                } => {
                    if self.first_team.id.eq(team_id) {
                        score.0 += 1;
                    }
                    if self.second_team.id.eq(team_id) {
                        score.1 += 1;
                    }
                }

                GameEvent::Success {
                    team_id,
                    success: Success::Casualty,
                    ..
                } => {
                    if self.first_team.id.eq(team_id) {
                        casualties.0 += 1;
                    }
                    if self.second_team.id.eq(team_id) {
                        casualties.1 += 1;
                    }
                }

                _ => {}
            }

            events_with_score_and_casualties.push(GameEventWithScoreAndCasualties {
                game_event: event.clone(),
                score: score.clone(),
                casualties: casualties.clone(),
            })
        }

        events_with_score_and_casualties
    }

    pub fn cancel_last_event(&mut self) -> Result<Option<GameEvent>, Error> {
        let last_event = self.events.pop();

        match last_event.clone() {
            Some(GameEvent::BuyInducement {
                team_id,
                used_money,
                inducement,
            }) => {
                if self.first_team.id.eq(&team_id) {
                    if used_money.treasury > 0 {
                        self.first_team.treasury += used_money.treasury;
                    }

                    match inducement {
                        Inducement::StarPlayer(position) | Inducement::MegaStarPlayer(position) => {
                            let index = self
                                .first_team
                                .players
                                .iter()
                                .position(|(_, player)| player.position.eq(&position));

                            if let Some(index) = index {
                                self.first_team.players.remove(index);
                            }
                        }
                        _ => {}
                    };
                }
                if self.second_team.id.eq(&team_id) {
                    if used_money.treasury > 0 {
                        self.second_team.treasury += used_money.treasury;
                    }

                    match inducement {
                        Inducement::StarPlayer(position) | Inducement::MegaStarPlayer(position) => {
                            let index = self
                                .second_team
                                .players
                                .iter()
                                .position(|(_, player)| player.position.eq(&position));

                            if let Some(index) = index {
                                self.second_team.players.remove(index);
                            }
                        }
                        _ => {}
                    };
                }
                Ok(last_event)
            }

            Some(GameEvent::Winnings {
                team_id,
                earned_money,
            }) => {
                if self.first_team.id.eq(&team_id) {
                    self.first_team.treasury = self.first_team.treasury - earned_money as i32;
                }
                if self.second_team.id.eq(&team_id) {
                    self.second_team.treasury = self.second_team.treasury - earned_money as i32;
                }
                Ok(last_event)
            }

            Some(GameEvent::ExpensiveMistakes {
                team_id,
                lost_money,
            }) => {
                if self.first_team.id.eq(&team_id) {
                    self.first_team.treasury = self.first_team.treasury + lost_money as i32;
                }
                if self.second_team.id.eq(&team_id) {
                    self.second_team.treasury = self.second_team.treasury + lost_money as i32;
                }
                Ok(last_event)
            }

            Some(GameEvent::DedicatedFansUpdate { team_id, delta }) => {
                if self.first_team.id.eq(&team_id) {
                    self.first_team.dedicated_fans =
                        (self.first_team.dedicated_fans as i8 - delta) as u8;
                }
                if self.second_team.id.eq(&team_id) {
                    self.second_team.dedicated_fans =
                        (self.second_team.dedicated_fans as i8 - delta) as u8;
                }
                Ok(last_event)
            }

            Some(GameEvent::Journeyman { team_id }) => {
                if self.first_team.id.eq(&team_id) {
                    let index = self
                        .first_team
                        .players
                        .iter()
                        .position(|(_, player)| player.position.eq(&Position::Journeyman));

                    if let Some(index) = index {
                        self.first_team.players.remove(index);
                    }
                }
                if self.second_team.id.eq(&team_id) {
                    let index = self
                        .second_team
                        .players
                        .iter()
                        .position(|(_, player)| player.position.eq(&Position::Journeyman));

                    if let Some(index) = index {
                        self.second_team.players.remove(index);
                    }
                }
                Ok(last_event)
            }

            Some(GameEvent::Injury {
                team_id,
                player_id,
                injury,
            }) => {
                if self.first_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .first_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.remove_injury(injury.clone());
                    }
                }
                if self.second_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .second_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.remove_injury(injury.clone());
                    }
                }
                Ok(last_event)
            }

            Some(GameEvent::Hatred {
                team_id,
                player_id,
                keyword,
            }) => {
                if self.first_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .first_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.remove_hatred(keyword.clone());
                    }
                }
                if self.second_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .second_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.remove_hatred(keyword.clone());
                    }
                }
                Ok(last_event)
            }

            Some(GameEvent::Success {
                team_id,
                player_id,
                star_player_points,
                ..
            }) => {
                if self.first_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .first_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.star_player_points =
                            player.star_player_points - star_player_points as i32;
                    }
                }
                if self.second_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .second_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.star_player_points =
                            player.star_player_points - star_player_points as i32;
                    }
                }
                Ok(last_event)
            }

            _ => Ok(last_event),
        }
    }

    pub(crate) fn process_event(&mut self, game_event: GameEvent) -> Result<(), Error> {
        if !self.started {
            return Err(Error::StartGameBeforeAddingEvents);
        }

        match (self.version, game_event.clone()) {
            (Version::V1 | Version::V2 | Version::V3 | Version::V4, _) => {
                return Err(Error::UnsupportedVersion);
            }

            (_, GameEvent::GameEnd) => {
                if self.needs_winner && !self.is_having_winner() {
                    return Err(Error::GameNeedsAWinner);
                }
            }

            (
                _,
                GameEvent::Penalties {
                    first_team_score,
                    second_team_score,
                },
            ) => {
                if first_team_score == second_team_score {
                    return Err(Error::GamePenaltiesShouldHaveAWinner);
                }
            }

            (
                _,
                GameEvent::BuyInducement {
                    team_id,
                    used_money,
                    ..
                },
            ) => {
                if self.first_team.id.eq(&team_id) && used_money.treasury > 0 {
                    self.first_team.treasury = self.first_team.treasury - used_money.treasury;
                }
                if self.second_team.id.eq(&team_id) && used_money.treasury > 0 {
                    self.second_team.treasury = self.second_team.treasury - used_money.treasury;
                }
            }

            (
                _,
                GameEvent::Winnings {
                    team_id,
                    earned_money,
                },
            ) => {
                if self.first_team.id.eq(&team_id) {
                    self.first_team.treasury += earned_money as i32;
                }
                if self.second_team.id.eq(&team_id) {
                    self.second_team.treasury += earned_money as i32;
                }
            }

            (
                _,
                GameEvent::ExpensiveMistakes {
                    team_id,
                    lost_money,
                },
            ) => {
                if self.first_team.id.eq(&team_id) {
                    self.first_team.treasury = self.first_team.treasury - lost_money as i32;
                }
                if self.second_team.id.eq(&team_id) {
                    self.second_team.treasury = self.second_team.treasury - lost_money as i32;
                }
            }

            (_, GameEvent::DedicatedFansUpdate { team_id, delta }) => {
                if self.first_team.id.eq(&team_id) {
                    self.first_team.dedicated_fans =
                        (self.first_team.dedicated_fans as i8 + delta) as u8;
                }
                if self.second_team.id.eq(&team_id) {
                    self.second_team.dedicated_fans =
                        (self.second_team.dedicated_fans as i8 + delta) as u8;
                }
            }

            (
                _,
                GameEvent::Injury {
                    team_id,
                    player_id,
                    injury,
                },
            ) => {
                if self.first_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .first_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.receive_injury(injury.clone());
                    }
                }
                if self.second_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .second_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.receive_injury(injury);
                    }
                }
            }

            (
                _,
                GameEvent::Hatred {
                    team_id,
                    player_id,
                    keyword,
                },
            ) => {
                if self.first_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .first_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.receive_hatred(keyword.clone());
                    }
                }
                if self.second_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .second_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.receive_hatred(keyword);
                    }
                }
            }

            (
                _,
                GameEvent::Success {
                    team_id,
                    player_id,
                    star_player_points,
                    ..
                },
            ) => {
                if self.first_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .first_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.star_player_points += star_player_points as i32;
                    }
                }
                if self.second_team.id.eq(&team_id) {
                    if let Some((_, player)) = self
                        .second_team
                        .players
                        .iter_mut()
                        .find(|(_, player)| player_id.eq(&player.id))
                    {
                        player.star_player_points += star_player_points as i32;
                    }
                }
            }

            (Version::V5, _) => {}
            (Version::V5S3, _) => {}
        };

        self.events.push(game_event);

        Ok(())
    }
}
