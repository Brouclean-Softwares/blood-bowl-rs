use crate::actions::Success;
use crate::errors::Error;
use crate::events::GameEvent;
use crate::games::{Game, v5, v5s3};
use crate::players::Player;
use crate::versions::Version;

impl Game {
    pub fn generate_winnings(
        &mut self,
        first_team_stalled: bool,
        second_team_stalled: bool,
    ) -> Result<Option<(u32, u32)>, Error> {
        match self.version {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 => Err(Error::UnsupportedVersion),
            Version::V5 => v5::generate_winnings(self),
            Version::V5S3 => v5s3::generate_winnings(self, first_team_stalled, second_team_stalled),
        }
    }

    pub fn push_winnings(&mut self, team_id: i32, earned_money: u32) -> Result<(), Error> {
        self.process_event(GameEvent::Winnings {
            team_id,
            earned_money,
        })
    }

    pub fn winnings(&self) -> (Option<u32>, Option<u32>) {
        let mut first_team_winnings: Option<u32> = None;
        let mut second_team_winnings: Option<u32> = None;

        for event in self.events.iter() {
            match event {
                GameEvent::Winnings {
                    team_id,
                    earned_money,
                } => {
                    if self.first_team.id.eq(team_id) {
                        first_team_winnings = Some(first_team_winnings.unwrap_or(0) + earned_money);
                    }
                    if self.second_team.id.eq(team_id) {
                        second_team_winnings =
                            Some(second_team_winnings.unwrap_or(0) + earned_money);
                    }
                }

                _ => {}
            }
        }

        (first_team_winnings, second_team_winnings)
    }

    pub fn generate_dedicated_fans_updates(&mut self) -> Result<(i8, i8), Error> {
        let first_team_delta = GameEvent::roll_dedicated_fans_delta(&self, &self.first_team);
        self.push_dedicated_fans_update(self.first_team.id, first_team_delta)?;

        let second_team_delta = GameEvent::roll_dedicated_fans_delta(&self, &self.second_team);
        self.push_dedicated_fans_update(self.second_team.id, second_team_delta)?;

        Ok((first_team_delta, second_team_delta))
    }

    pub fn push_dedicated_fans_update(&mut self, team_id: i32, delta: i8) -> Result<(), Error> {
        self.process_event(GameEvent::DedicatedFansUpdate { team_id, delta })
    }

    pub fn dedicated_fans_updates(&self) -> (Option<i8>, Option<i8>) {
        let mut first_team_delta: Option<i8> = None;
        let mut second_team_delta: Option<i8> = None;

        for event in self.events.iter() {
            match event {
                GameEvent::DedicatedFansUpdate { team_id, delta } => {
                    if self.first_team.id.eq(team_id) {
                        first_team_delta = Some(first_team_delta.unwrap_or(0) + delta);
                    }
                    if self.second_team.id.eq(team_id) {
                        second_team_delta = Some(second_team_delta.unwrap_or(0) + delta);
                    }
                }

                _ => {}
            }
        }

        (first_team_delta, second_team_delta)
    }

    pub fn most_valuable_players(&self) -> (Vec<Player>, Vec<Player>) {
        let mut first_team_mvps: Vec<Player> = vec![];
        let mut second_team_mvps: Vec<Player> = vec![];

        for event in self.events.iter() {
            match event {
                GameEvent::Success {
                    team_id,
                    player_id,
                    success,
                    ..
                } => {
                    if matches!(success, Success::MostValuablePlayer) {
                        if self.first_team.id.eq(team_id) {
                            if let Some((_, player)) =
                                self.first_team.player_by_id(player_id.clone())
                            {
                                first_team_mvps.push(player);
                            }
                        }
                        if self.second_team.id.eq(team_id) {
                            if let Some((_, player)) =
                                self.second_team.player_by_id(player_id.clone())
                            {
                                second_team_mvps.push(player);
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        (first_team_mvps, second_team_mvps)
    }

    pub fn push_expensive_mistakes(&mut self, team_id: i32, lost_money: i32) -> Result<(), Error> {
        let lost_money: u32 = if lost_money >= 0 {
            lost_money
        } else {
            -1 * lost_money
        } as u32;

        self.process_event(GameEvent::ExpensiveMistakes {
            team_id,
            lost_money,
        })
    }

    pub fn expensive_mistakes(&self) -> (Option<u32>, Option<u32>) {
        let mut first_team_loss: Option<u32> = None;
        let mut second_team_loss: Option<u32> = None;

        for event in self.events.iter() {
            match event {
                GameEvent::ExpensiveMistakes {
                    team_id,
                    lost_money,
                } => {
                    if self.first_team.id.eq(team_id) {
                        first_team_loss = Some(first_team_loss.unwrap_or(0) + lost_money);
                    }
                    if self.second_team.id.eq(team_id) {
                        second_team_loss = Some(second_team_loss.unwrap_or(0) + lost_money);
                    }
                }

                _ => {}
            }
        }

        (first_team_loss, second_team_loss)
    }

    pub fn post_game_sequence_is_finished(&self) -> bool {
        self.expensive_mistakes().0.is_some() && self.expensive_mistakes().1.is_some()
    }
}
