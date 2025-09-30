use crate::coaches::Coach;
use crate::errors::Error;
use crate::events::GameEvent;
use crate::inducements::{Inducement, TreasuryAndPettyCash};
use crate::injuries::Injury;
use crate::players::{Player, PlayerStatistics};
use crate::positions::Position;
use crate::prayers::PrayerToNuffle;
use crate::teams::Team;
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use crate::weather::Weather;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    Scheduled,
    PreGameSequence,
    GameInProgress,
    PostGameSequence,
    WaitingForValidation,
    Closed,
}

impl TypeName for GameStatus {}
impl TranslatedName for GameStatus {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub version: Version,
    pub created_by: Option<Coach>,
    pub game_at: NaiveDateTime,
    pub started: bool,
    pub closed: bool,
    pub first_team: Team,
    pub second_team: Team,
    pub events: Vec<GameEvent>,
}

impl Game {
    pub fn create(
        id: i32,
        created_by: Option<Coach>,
        version: Version,
        game_at: NaiveDateTime,
        team_a: &Team,
        team_b: &Team,
    ) -> Result<Self, Error> {
        let game = Self {
            id,
            version,
            created_by,
            game_at,
            started: false,
            closed: false,
            first_team: team_a.clone(),
            second_team: team_b.clone(),
            events: vec![],
        };

        let _ = game.check_if_rules_compliant();

        Ok(game)
    }

    pub fn check_if_rules_compliant(&self) -> Result<(), Error> {
        if self.first_team.version.ne(&self.version) || self.second_team.version.ne(&self.version) {
            return Err(Error::TeamsMustMatchGameVersion);
        }

        self.first_team.check_if_rules_compliant()?;
        self.second_team.check_if_rules_compliant()?;

        Ok(())
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn generate_fans(&mut self) -> Result<u32, Error> {
        let mut game_fans: u32 = 0;

        let fan_factor = GameEvent::roll_fan_factor(self.first_team.clone());
        game_fans += fan_factor as u32;
        self.set_team_fan_factor(self.first_team.clone(), fan_factor)?;

        let fan_factor = GameEvent::roll_fan_factor(self.second_team.clone());
        game_fans += fan_factor as u32;
        self.set_team_fan_factor(self.second_team.clone(), fan_factor)?;

        Ok(game_fans * 10000)
    }

    pub fn set_team_fan_factor(&mut self, team_for: Team, new_fan_factor: u8) -> Result<u8, Error> {
        let position = self.events.iter().position(|event| match event {
            GameEvent::FanFactor { team_id, .. } => team_for.id.eq(team_id),
            _ => false,
        });

        if let Some(position) = position {
            self.events[position] = GameEvent::FanFactor {
                team_id: team_for.id,
                fan_factor: new_fan_factor,
            };
            Ok(new_fan_factor)
        } else {
            self.process_event(GameEvent::FanFactor {
                team_id: team_for.id,
                fan_factor: new_fan_factor,
            })?;
            Ok(new_fan_factor)
        }
    }

    pub fn team_fan_factor(&self, team_for: &Team) -> Option<u8> {
        for event in self.events.iter() {
            if let GameEvent::FanFactor {
                team_id,
                fan_factor,
            } = event
                && team_id.eq(&team_for.id)
            {
                return Some(fan_factor.clone());
            }
        }

        None
    }

    pub fn fans(&self) -> Option<u32> {
        if let (Some(first_team_fan_factor), Some(second_team_fan_factor)) = (
            self.team_fan_factor(&self.first_team),
            self.team_fan_factor(&self.second_team),
        ) {
            Some(10000 * (first_team_fan_factor as u32 + second_team_fan_factor as u32))
        } else {
            None
        }
    }

    pub fn generate_weather(&mut self) -> Result<Weather, Error> {
        let weather = Weather::roll();
        self.push_weather(weather)
    }

    pub fn push_weather(&mut self, new_weather: Weather) -> Result<Weather, Error> {
        self.process_event(GameEvent::Weather(new_weather.clone()))?;
        Ok(new_weather)
    }

    pub fn weather(&self) -> Option<Weather> {
        let mut weather = None;

        for event in self.events.iter() {
            if let GameEvent::Weather(weather_from_event) = event {
                weather = Some(weather_from_event.clone());
            }
        }

        weather
    }

    pub fn generate_journeymen(&mut self) -> Result<(u8, u8), Error> {
        let players = self.first_team.number_of_available_players();
        let first_team_journeymen_number = if players < 11 { 11 - players } else { 0 };

        for _ in 0..first_team_journeymen_number {
            let _ = self.first_team.add_journeyman_with_number(0);

            self.process_event(GameEvent::Journeyman {
                team_id: self.first_team.id,
            })?;
        }

        let players = self.second_team.number_of_available_players();
        let second_team_journeymen_number = if players < 11 { 11 - players } else { 0 };

        for _ in 0..second_team_journeymen_number {
            let _ = self.second_team.add_journeyman_with_number(0);

            self.process_event(GameEvent::Journeyman {
                team_id: self.second_team.id,
            })?;
        }

        Ok((first_team_journeymen_number, second_team_journeymen_number))
    }

    pub fn journeymen_ok(&self) -> bool {
        self.first_team.available_players().len() >= 11
            && self.second_team.available_players().len() >= 11
    }

    pub fn petty_cash(&self) -> Result<(u32, u32), Error> {
        let (first_team_cost_for_added_players, second_team_cost_for_added_players) =
            self.teams_inducements_cost_for_added_players();
        let (first_team_treasury_used, second_team_treasury_used) =
            self.teams_treasury_used_for_inducements();

        let first_team_value =
            self.first_team.current_value()? - first_team_cost_for_added_players as u32;
        let second_team_value =
            self.second_team.current_value()? - second_team_cost_for_added_players as u32;

        let first_team_petty_cash = if first_team_value < second_team_value {
            second_team_value + second_team_treasury_used as u32 - first_team_value
        } else {
            0
        };

        let second_team_petty_cash = if second_team_value < first_team_value {
            first_team_value + first_team_treasury_used as u32 - second_team_value
        } else {
            0
        };

        Ok((first_team_petty_cash, second_team_petty_cash))
    }

    pub fn teams_money_left(&self) -> Result<(TreasuryAndPettyCash, TreasuryAndPettyCash), Error> {
        let (mut first_team_petty_cash_left, mut second_team_petty_cash_left) =
            self.petty_cash()?;

        for event in self.events.iter() {
            if let GameEvent::BuyInducement {
                team_id,
                money_used,
                ..
            } = event
            {
                if self.first_team.id.eq(team_id) {
                    first_team_petty_cash_left -= money_used.petty_cash;
                }

                if self.second_team.id.eq(team_id) {
                    second_team_petty_cash_left -= money_used.petty_cash;
                }
            }
        }

        Ok((
            TreasuryAndPettyCash {
                treasury: self.first_team.treasury,
                petty_cash: first_team_petty_cash_left,
            },
            TreasuryAndPettyCash {
                treasury: self.second_team.treasury,
                petty_cash: second_team_petty_cash_left,
            },
        ))
    }

    pub fn teams_inducements(&self) -> (Vec<Inducement>, Vec<Inducement>) {
        let mut first_inducements: Vec<Inducement> = vec![];
        let mut second_inducements: Vec<Inducement> = vec![];

        for event in self.events.iter() {
            if let GameEvent::BuyInducement {
                team_id,
                inducement,
                ..
            } = event
            {
                if team_id.eq(&self.first_team.id) {
                    first_inducements.push(inducement.clone());
                }

                if team_id.eq(&self.second_team.id) {
                    second_inducements.push(inducement.clone());
                }
            }
        }

        (first_inducements, second_inducements)
    }

    pub fn teams_treasury_used_for_inducements(&self) -> (i32, i32) {
        let mut first_team_treasury_used: i32 = 0;
        let mut second_team_treasury_used: i32 = 0;

        for event in self.events.iter() {
            match event {
                GameEvent::BuyInducement {
                    team_id,
                    money_used,
                    ..
                } => {
                    if team_id.eq(&self.first_team.id) {
                        first_team_treasury_used += money_used.treasury;
                    }

                    if team_id.eq(&self.second_team.id) {
                        second_team_treasury_used += money_used.treasury;
                    }
                }

                _ => {}
            }
        }

        (first_team_treasury_used, second_team_treasury_used)
    }

    pub fn teams_inducements_cost_for_added_players(&self) -> (i32, i32) {
        let mut first_team_cost: i32 = 0;
        let mut second_team_cost: i32 = 0;

        for event in self.events.iter() {
            match event {
                GameEvent::BuyInducement {
                    team_id,
                    money_used,
                    inducement: Inducement::StarPlayer(_) | Inducement::MegaStarPlayer(_),
                } => {
                    if team_id.eq(&self.first_team.id) {
                        first_team_cost += money_used.total();
                    }

                    if team_id.eq(&self.second_team.id) {
                        second_team_cost += money_used.total();
                    }
                }

                _ => {}
            }
        }

        (first_team_cost, second_team_cost)
    }

    pub fn team_inducement_type_number(
        &self,
        team_id_for: i32,
        inducement_to_check: &Inducement,
    ) -> usize {
        self.events
            .iter()
            .filter(|&event| {
                if let GameEvent::BuyInducement {
                    team_id,
                    inducement,
                    ..
                } = event
                {
                    team_id_for.eq(team_id) && inducement.eq(&inducement_to_check)
                } else {
                    false
                }
            })
            .count()
    }

    pub fn inducements_buyable_by_teams(
        &self,
    ) -> Result<(Vec<Inducement>, Vec<Inducement>), Error> {
        let (first_team_money_left, second_team_money_left) = self.teams_money_left()?;

        let mut first_team_inducements_buyable: Vec<Inducement> =
            Inducement::list_buyable_for_team(&self.first_team, &first_team_money_left);

        first_team_inducements_buyable.retain(|inducement| {
            self.team_inducement_type_number(self.first_team.id.clone(), inducement)
                .lt(&inducement.maximum_for_team(&self.first_team))
        });

        let mut second_team_inducements_buyable: Vec<Inducement> =
            Inducement::list_buyable_for_team(&self.second_team, &second_team_money_left);

        second_team_inducements_buyable.retain(|inducement| {
            self.team_inducement_type_number(self.second_team.id.clone(), inducement)
                .lt(&inducement.maximum_for_team(&self.second_team))
        });

        Ok((
            first_team_inducements_buyable,
            second_team_inducements_buyable,
        ))
    }

    pub fn team_buy_inducement(
        &mut self,
        team_id: i32,
        inducement: Inducement,
    ) -> Result<Inducement, Error> {
        if !self.journeymen_ok() {
            return Err(Error::JourneymenShouldBeOkBeforeBuyingInducements);
        }

        let (buyable_by_first_team, buyable_by_second_team) =
            self.inducements_buyable_by_teams()?;
        let (first_team_money_left, second_team_money_left) = self.teams_money_left()?;

        if team_id.eq(&self.first_team.id) && buyable_by_first_team.contains(&inducement) {
            self.process_event(GameEvent::BuyInducement {
                team_id,
                inducement: inducement.clone(),
                money_used: first_team_money_left
                    .money_used_to_buy(inducement.price_for_team(&self.first_team))?,
            })?;

            match inducement {
                Inducement::StarPlayer(position) | Inducement::MegaStarPlayer(position) => {
                    let _ = self.first_team.add_star_player_with_number(0, position);
                }
                _ => {}
            };

            return Ok(inducement);
        }

        if team_id.eq(&self.second_team.id) && buyable_by_second_team.contains(&inducement) {
            self.process_event(GameEvent::BuyInducement {
                team_id,
                inducement: inducement.clone(),
                money_used: second_team_money_left
                    .money_used_to_buy(inducement.price_for_team(&self.second_team))?,
            })?;

            match inducement {
                Inducement::StarPlayer(position) | Inducement::MegaStarPlayer(position) => {
                    let _ = self.second_team.add_star_player_with_number(0, position);
                }
                _ => {}
            };

            return Ok(inducement);
        }

        Err(Error::NotAPlayingTeam)
    }

    pub fn push_prayer(
        &mut self,
        team_id: i32,
        new_prayer: PrayerToNuffle,
    ) -> Result<PrayerToNuffle, Error> {
        self.process_event(GameEvent::PrayerToNuffle {
            team_id,
            prayer_to_nuffle: new_prayer.clone(),
        })?;
        Ok(new_prayer)
    }

    pub fn teams_prayers(&self) -> (Vec<PrayerToNuffle>, Vec<PrayerToNuffle>) {
        let mut first_prayers: Vec<PrayerToNuffle> = vec![];
        let mut second_prayers: Vec<PrayerToNuffle> = vec![];

        for event in self.events.iter() {
            if let GameEvent::PrayerToNuffle {
                team_id,
                prayer_to_nuffle,
                ..
            } = event
            {
                if team_id.eq(&self.first_team.id) {
                    first_prayers.push(prayer_to_nuffle.clone());
                }

                if team_id.eq(&self.second_team.id) {
                    second_prayers.push(prayer_to_nuffle.clone());
                }
            }
        }

        (first_prayers, second_prayers)
    }

    pub fn generate_toss_winner(&mut self) -> Result<i32, Error> {
        let team_id = GameEvent::roll_toss_winner(self);
        self.push_toss_winner(team_id)
    }

    pub fn push_toss_winner(&mut self, team_id: i32) -> Result<i32, Error> {
        self.process_event(GameEvent::TossWinner {
            team_id: team_id.clone(),
        })?;
        Ok(team_id)
    }

    pub fn toss_winner(&self) -> Option<&Team> {
        for event in self.events.iter() {
            if let GameEvent::TossWinner { team_id } = event {
                return if self.first_team.id.eq(team_id) {
                    Some(&self.first_team)
                } else if self.second_team.id.eq(team_id) {
                    Some(&self.second_team)
                } else {
                    None
                };
            }
        }

        None
    }

    pub fn push_kicking_team(&mut self, team_id: i32) -> Result<i32, Error> {
        self.process_event(GameEvent::KickingTeam { team_id })?;
        Ok(team_id)
    }

    pub fn kicking_team(&self) -> Option<&Team> {
        for event in self.events.iter() {
            if let GameEvent::KickingTeam { team_id } = event {
                return if self.first_team.id.eq(team_id) {
                    Some(&self.first_team)
                } else if self.second_team.id.eq(team_id) {
                    Some(&self.second_team)
                } else {
                    None
                };
            }
        }

        None
    }

    pub fn push_injury(
        &mut self,
        team_id: i32,
        player_id: i32,
        injury: Injury,
    ) -> Result<(), Error> {
        self.process_event(GameEvent::Injury {
            team_id,
            player_id,
            injury,
        })
    }

    pub fn suffered_injuries(&self, team_id_for: i32, player_id_for: i32) -> Vec<Injury> {
        let mut injuries: Vec<Injury> = vec![];

        for event in self.events.iter() {
            if let GameEvent::Injury {
                team_id,
                player_id,
                injury,
            } = event
            {
                if team_id_for.eq(team_id) && player_id_for.eq(player_id) {
                    injuries.push(injury.clone());
                }
            }
        }

        injuries
    }

    pub fn end_game(&mut self) -> Result<(), Error> {
        self.process_event(GameEvent::GameEnd)
    }

    pub fn cancel_last_event(&mut self) -> Result<(), Error> {
        match self.events.pop() {
            Some(GameEvent::BuyInducement {
                team_id,
                money_used,
                inducement,
            }) => {
                if self.first_team.id.eq(&team_id) {
                    if money_used.treasury > 0 {
                        self.first_team.treasury += money_used.treasury;
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
                    if money_used.treasury > 0 {
                        self.second_team.treasury += money_used.treasury;
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
                Ok(())
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
                Ok(())
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
                Ok(())
            }

            _ => Ok(()),
        }
    }

    pub fn process_event(&mut self, game_event: GameEvent) -> Result<(), Error> {
        if !self.started {
            return Err(Error::StartMatchBeforeAddingEvents);
        }

        match (self.version, game_event.clone()) {
            (Version::V4, _) => return Err(Error::UnsupportedVersion),

            (
                _,
                GameEvent::BuyInducement {
                    team_id,
                    money_used,
                    ..
                },
            ) => {
                if self.first_team.id.eq(&team_id) && money_used.treasury > 0 {
                    self.first_team.treasury = self.first_team.treasury - money_used.treasury;
                }
                if self.second_team.id.eq(&team_id) && money_used.treasury > 0 {
                    self.second_team.treasury = self.second_team.treasury - money_used.treasury;
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

            (Version::V5, _) => {}
        };

        self.events.push(game_event);

        Ok(())
    }

    pub fn pre_game_sequence_is_finished(&self) -> bool {
        self.kicking_team().is_some()
    }

    pub fn game_finished(&self) -> bool {
        self.events.contains(&GameEvent::GameEnd)
    }

    pub fn playing_players(&self) -> (Vec<(i32, Player)>, Vec<(i32, Player)>) {
        (
            self.first_team.available_players(),
            self.second_team.available_players(),
        )
    }

    pub fn player_statistics(&self, _team: &Team, _player: &Player) -> PlayerStatistics {
        let statistics = PlayerStatistics::new();

        statistics
    }

    pub fn players_statistics_for_team(&self, team: &Team) -> Vec<(i32, Player, PlayerStatistics)> {
        let mut statistics: Vec<(i32, Player, PlayerStatistics)> = vec![];

        for (number, player) in team.available_players() {
            statistics.push((
                number,
                player.clone(),
                self.player_statistics(&team, &player),
            ));
        }

        statistics
    }

    pub fn post_game_sequence_is_finished(&self) -> bool {
        false
    }

    pub fn status(&self) -> GameStatus {
        if !self.started {
            return GameStatus::Scheduled;
        } else if !self.pre_game_sequence_is_finished() {
            return GameStatus::PreGameSequence;
        } else if !self.game_finished() {
            return GameStatus::GameInProgress;
        } else if !self.post_game_sequence_is_finished() {
            return GameStatus::PostGameSequence;
        } else if !self.closed {
            return GameStatus::WaitingForValidation;
        } else {
            return GameStatus::Closed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::injuries::Injury;
    use crate::players::Player;
    use crate::positions::Position;
    use crate::rosters::{Roster, Staff};
    use crate::versions::Version;
    use std::collections::HashMap;

    #[test]
    fn new_game_v5() {
        let coach_a = Coach {
            id: Some(1),
            name: "Me".to_string(),
        };

        let team_a = Team {
            id: 1,
            version: Version::V5,
            roster: Roster::WoodElf,
            name: "Woodies".to_string(),
            coach: coach_a.clone(),
            treasury: 30000,
            external_logo_url: None,
            staff: HashMap::from([
                (Staff::Apothecary, 1),
                (Staff::ReRoll, 1),
                (Staff::Cheerleader, 0),
                (Staff::AssistantCoach, 0),
            ]),
            players: vec![
                (1, Player::new(Version::V5, Position::WoodElfLineman)),
                (2, Player::new(Version::V5, Position::WoodElfLineman)),
                (3, Player::new(Version::V5, Position::WoodElfLineman)),
                (4, Player::new(Version::V5, Position::WoodElfLineman)),
                (5, Player::new(Version::V5, Position::WoodElfLineman)),
                (6, Player::new(Version::V5, Position::WoodElfLineman)),
                (7, Player::new(Version::V5, Position::WoodElfLineman)),
                (8, Player::new(Version::V5, Position::Thrower)),
                (9, Player::new(Version::V5, Position::Thrower)),
                (10, Player::new(Version::V5, Position::Wardancer)),
                (11, Player::new(Version::V5, Position::Wardancer)),
            ],
            dedicated_fans: 4,
            under_creation: false,
        };

        let coach_b = Coach {
            id: Some(2),
            name: "Him".to_string(),
        };

        let team_b = Team {
            id: 2,
            version: Version::V5,
            roster: Roster::Amazon,
            name: "Amazons".to_string(),
            coach: coach_b.clone(),
            treasury: 200000,
            external_logo_url: None,
            staff: HashMap::from([
                (Staff::Apothecary, 1),
                (Staff::ReRoll, 3),
                (Staff::Cheerleader, 0),
                (Staff::AssistantCoach, 0),
            ]),
            players: vec![
                (1, Player::new(Version::V5, Position::EagleWarriorLinewoman)),
                (2, Player::new(Version::V5, Position::EagleWarriorLinewoman)),
                (3, Player::new(Version::V5, Position::EagleWarriorLinewoman)),
                (4, Player::new(Version::V5, Position::EagleWarriorLinewoman)),
                (5, Player::new(Version::V5, Position::EagleWarriorLinewoman)),
                (6, Player::new(Version::V5, Position::PythonWarriorThrower)),
                (7, Player::new(Version::V5, Position::PythonWarriorThrower)),
                (8, Player::new(Version::V5, Position::PiranhaWarriorBlitzer)),
                (9, Player::new(Version::V5, Position::PiranhaWarriorBlitzer)),
                (10, Player::new(Version::V5, Position::JaguarWarriorBlocker)),
                (11, Player::new(Version::V5, Position::JaguarWarriorBlocker)),
            ],
            dedicated_fans: 2,
            under_creation: false,
        };

        let played_at_str = "2020-09-05 23:56:04";
        let played_at = NaiveDateTime::parse_from_str(played_at_str, "%Y-%m-%d %H:%M:%S").unwrap();

        let mut game =
            Game::create(-1, None, Version::V5, played_at.clone(), &team_a, &team_b).unwrap();
        assert_eq!(game.first_team.available_players().len(), 11);
        assert_eq!(game.second_team.available_players().len(), 11);
        assert!(matches!(game.status(), GameStatus::Scheduled));

        assert_eq!(game.first_team.min_players_id().unwrap(), -1);

        assert_eq!(game.first_team.value().unwrap(), 1030000);
        assert_eq!(game.second_team.value().unwrap(), 1040000);

        let _ = game.start();
        assert!(game.started);
        assert!(matches!(game.status(), GameStatus::PreGameSequence));

        let fans = game.generate_fans().unwrap();
        assert_eq!(game.fans().unwrap(), fans);

        let weather = game.generate_weather().unwrap();
        assert_eq!(game.weather().unwrap(), weather);

        let journey_men = game.generate_journeymen().unwrap();
        assert_eq!(journey_men, (0, 0));

        let mut other_game = game.clone();
        let _ = other_game.first_team.players.pop();
        let _ = other_game.first_team.players.pop();
        let _ = other_game.second_team.players.pop();
        let journey_men = other_game.generate_journeymen().unwrap();
        assert_eq!(other_game.playing_players().0.len(), 11);
        assert_eq!(journey_men, (2, 1));
        assert_eq!(other_game.first_team.journeymen_number(), 2);
        assert_eq!(other_game.second_team.journeymen_number(), 1);
        assert_eq!(journey_men, (2, 1));
        assert_eq!(
            game.first_team.value().unwrap() - 110000,
            other_game.first_team.value().unwrap()
        );
        assert_eq!(
            game.second_team.value().unwrap() - 60000,
            other_game.second_team.value().unwrap()
        );
        let (number, player) = other_game.first_team.players.pop().unwrap();
        assert_eq!(number, 0);
        assert_eq!(player, Player::new_journeyman(-3, Version::V5));
        let (number, player) = other_game.second_team.players.pop().unwrap();
        assert_eq!(number, 0);
        assert_eq!(player, Player::new_journeyman(-2, Version::V5));

        let petty_cash = game.petty_cash().unwrap();
        assert_eq!(petty_cash, (10000, 0));

        let other_game =
            Game::create(-1, None, Version::V5, played_at.clone(), &team_b, &team_a).unwrap();
        let petty_cash = other_game.petty_cash().unwrap();
        assert_eq!(petty_cash, (0, 10000));

        let (team_a_money_left, team_b_money_left) = game.teams_money_left().unwrap();
        assert_eq!(team_a_money_left.total(), 40000);
        assert_eq!(team_b_money_left.total(), 200000);
        assert!(
            game.team_buy_inducement(game.first_team.id.clone(), Inducement::PlagueDoctor)
                .is_err()
        );
        assert!(
            game.team_buy_inducement(
                game.first_team.id.clone(),
                Inducement::WanderingApothecaries
            )
            .is_err()
        );
        let _ = game
            .team_buy_inducement(
                game.first_team.id.clone(),
                Inducement::TempAgencyCheerleaders,
            )
            .unwrap();
        assert_eq!(
            game.team_inducement_type_number(
                game.first_team.id.clone(),
                &Inducement::TempAgencyCheerleaders
            ),
            1
        );

        let (team_a_money_left, _) = game.teams_money_left().unwrap();
        assert_eq!(team_a_money_left.petty_cash, 0);
        assert_eq!(game.first_team.treasury, 20000);
        assert_eq!(team_a_money_left.treasury, game.first_team.treasury);

        let second_team_value = game.second_team.value().unwrap();
        let _ = game
            .team_buy_inducement(
                game.second_team.id.clone(),
                Inducement::StarPlayer(Position::AkhorneTheSquirrel),
            )
            .unwrap();
        assert_eq!(game.second_team.available_players().len(), 12);
        assert_eq!(game.second_team.value().unwrap(), second_team_value + 80000);

        game.cancel_last_event().unwrap();
        assert_eq!(game.second_team.available_players().len(), 11);
        assert_eq!(game.second_team.value().unwrap(), second_team_value);

        let prayer = game
            .push_prayer(game.first_team.id, PrayerToNuffle::roll())
            .unwrap();
        let (first_prayers, second_prayers) = game.teams_prayers();
        assert_eq!(prayer, first_prayers[0]);
        assert_eq!(first_prayers.len(), 1);
        assert_eq!(second_prayers.len(), 0);

        let toss_team_id = game.generate_toss_winner().unwrap();
        assert_eq!(game.toss_winner().unwrap().id, toss_team_id);

        let kicking_team_id = game.push_kicking_team(game.first_team.id).unwrap();
        assert_eq!(game.kicking_team().unwrap().id, kicking_team_id);

        assert!(game.pre_game_sequence_is_finished());

        let _ = game.cancel_last_event().unwrap();
        assert!(!game.pre_game_sequence_is_finished());

        let kicking_team_id = game.push_kicking_team(game.first_team.id).unwrap();
        assert_eq!(game.kicking_team().unwrap().id, kicking_team_id);

        assert!(game.pre_game_sequence_is_finished());

        let player = game.first_team.players[0].1.clone();
        let agility = player.agility(&game.first_team.roster).unwrap();
        let _ = game.push_injury(
            game.first_team.id.clone(),
            player.id.clone(),
            Injury::NeckInjury,
        );
        assert_eq!(
            game.first_team.players[0]
                .1
                .agility(&game.first_team.roster)
                .unwrap(),
            agility + 1
        );
        game.cancel_last_event().unwrap();
        assert_eq!(player.agility(&game.first_team.roster).unwrap(), agility);
    }
}
