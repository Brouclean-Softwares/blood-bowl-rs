use crate::coaches::Coach;
use crate::errors::Error;
use crate::events::{GameEvent, Weather};
use crate::players::{Player, PlayerStatistic};
use crate::teams::Team;
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
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
    pub scheduled_at: NaiveDateTime,
    pub started_at: Option<NaiveDateTime>,
    pub closed_at: Option<NaiveDateTime>,
    pub first_team: Team,
    pub second_team: Team,
    pub first_team_playing_players_statistics: Vec<(i32, Player, PlayerStatistic)>,
    pub second_team_playing_players_statistics: Vec<(i32, Player, PlayerStatistic)>,
    pub events: Vec<GameEvent>,
}

impl Game {
    pub fn create(
        id: i32,
        created_by: Option<Coach>,
        version: Version,
        scheduled_at: NaiveDateTime,
        team_a: &Team,
        team_b: &Team,
    ) -> Result<Self, Error> {
        let get_team_players_statistics = |team: &Team| {
            let mut players_statistics: Vec<(i32, Player, PlayerStatistic)> = vec![];

            for (number, player) in team.players.clone() {
                if player.available() {
                    players_statistics.push((number, player, PlayerStatistic::new()))
                }
            }

            players_statistics
        };

        let game = Self {
            id,
            version,
            created_by,
            scheduled_at,
            started_at: None,
            closed_at: None,
            first_team: team_a.clone(),
            second_team: team_b.clone(),
            first_team_playing_players_statistics: get_team_players_statistics(team_a),
            second_team_playing_players_statistics: get_team_players_statistics(team_b),
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

    pub fn start(&mut self) -> Result<(), Error> {
        self.start_at(self.scheduled_at.clone())
    }

    pub fn start_at(&mut self, started_at: NaiveDateTime) -> Result<(), Error> {
        self.check_if_rules_compliant()?;
        self.started_at = Some(started_at);
        Ok(())
    }

    pub fn generate_fans(&mut self) -> Result<u32, Error> {
        let mut game_fans = 0;

        let fan_factor = GameEvent::roll_fan_factor(&self.first_team);
        game_fans += fan_factor;
        self.process_event(GameEvent::FanFactor(self.first_team.clone(), fan_factor))?;

        let fan_factor = GameEvent::roll_fan_factor(&self.second_team);
        game_fans += fan_factor;
        self.process_event(GameEvent::FanFactor(self.second_team.clone(), fan_factor))?;

        Ok(game_fans)
    }

    pub fn team_fan_factor(&self, team_for: &Team) -> Option<u32> {
        let mut team_fans = None;

        for event in self.events.iter() {
            if let GameEvent::FanFactor(team, fans) = event
                && team.eq(team_for)
            {
                team_fans = Some(team_fans.unwrap_or(0) + fans);
            }
        }

        team_fans
    }

    pub fn fans(&self) -> Option<u32> {
        if let (Some(first_team_fan_factor), Some(second_team_fan_factor)) = (
            self.team_fan_factor(&self.first_team),
            self.team_fan_factor(&self.second_team),
        ) {
            Some(first_team_fan_factor + second_team_fan_factor)
        } else {
            None
        }
    }

    pub fn generate_weather(&mut self) -> Result<Weather, Error> {
        let weather = Weather::roll();
        self.process_event(GameEvent::Weather(weather.clone()))?;
        Ok(weather)
    }

    pub fn weather(&mut self) -> Option<Weather> {
        let mut weather = None;

        for event in self.events.iter() {
            if let GameEvent::Weather(weather_from_event) = event {
                weather = Some(weather_from_event.clone());
            }
        }

        weather
    }

    pub fn process_event(&mut self, game_event: GameEvent) -> Result<(), Error> {
        if self.started_at.is_none() {
            return Err(Error::StartMatchBeforeAddingEvents);
        }

        match (self.version, game_event.clone()) {
            (Version::V4, _) => return Err(Error::UnsupportedVersion),

            (Version::V5, _) => {}
        };

        self.events.push(game_event);

        Ok(())
    }

    pub fn pre_game_sequence_is_finished(&self) -> bool {
        false
    }

    pub fn game_finished(&self) -> bool {
        false
    }

    pub fn post_game_sequence_is_finished(&self) -> bool {
        false
    }

    pub fn status(&self) -> GameStatus {
        if self.started_at.is_none() {
            return GameStatus::Scheduled;
        } else if !self.pre_game_sequence_is_finished() {
            return GameStatus::PreGameSequence;
        } else if !self.game_finished() {
            return GameStatus::GameInProgress;
        } else if !self.post_game_sequence_is_finished() {
            return GameStatus::PostGameSequence;
        } else if !self.closed_at.is_none() {
            return GameStatus::WaitingForValidation;
        } else {
            return GameStatus::Closed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            treasury: 20000,
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
        assert_eq!(game.first_team_playing_players_statistics.len(), 11);
        assert_eq!(game.second_team_playing_players_statistics.len(), 11);
        assert!(matches!(game.status(), GameStatus::Scheduled));

        let _ = game.start();
        assert!(game.started_at.is_some());
        assert!(matches!(game.status(), GameStatus::PreGameSequence));

        let fans = game.generate_fans().unwrap();
        assert_eq!(game.fans().unwrap(), fans);

        let weather = game.generate_weather().unwrap();
        assert_eq!(game.weather().unwrap(), weather);
    }
}
