use crate::coaches::Coach;
use crate::errors::Error;
use crate::events::{GameEvent, Weather};
use crate::players::Player;
use crate::teams::{Team, TeamSummary};
use crate::versions::Version;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub mod v5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSummary {
    pub id: i32,
    pub version: Version,
    pub created_by: Option<Coach>,
    pub played_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
    pub first_team: TeamSummary,
    pub second_team: TeamSummary,
    pub first_team_score: usize,
    pub second_team_score: usize,
    pub first_team_casualties: usize,
    pub second_team_casualties: usize,
}

impl Eq for GameSummary {}

impl PartialEq<Self> for GameSummary {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<Self> for GameSummary {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for GameSummary {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.closed_at, other.closed_at) {
            (Some(closed_at), Some(other_closed_at)) => closed_at.cmp(&other_closed_at),
            (Some(closed_at), None) => closed_at.cmp(&other.played_at),
            (None, Some(other_closed_at)) => self.played_at.cmp(&other_closed_at),
            (None, None) => self.played_at.cmp(&other.played_at),
        }
    }
}

impl From<&Game> for GameSummary {
    fn from(game: &Game) -> Self {
        let cloned_game = game.clone();

        let first_team = TeamSummary::from(&cloned_game.first_team);
        let second_team = TeamSummary::from(&cloned_game.second_team);

        Self {
            id: cloned_game.id.unwrap_or_default(),
            version: cloned_game.version,
            created_by: cloned_game.created_by,
            played_at: cloned_game.played_at,
            closed_at: cloned_game.closed_at,
            first_team,
            second_team,
            first_team_score: 0,
            second_team_score: 0,
            first_team_casualties: 0,
            second_team_casualties: 0,
        }
    }
}

impl GameSummary {
    pub fn winner(&self) -> Option<&TeamSummary> {
        if self.closed_at.is_some() && self.first_team_score > self.second_team_score {
            Some(&self.first_team)
        } else if self.closed_at.is_some() && self.first_team_score < self.second_team_score {
            Some(&self.second_team)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: Option<i32>,
    pub version: Version,
    pub created_by: Option<Coach>,
    pub played_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
    pub first_team: Team,
    pub second_team: Team,
    pub first_team_playing_players: Vec<(i32, Player)>,
    pub second_team_playing_players: Vec<(i32, Player)>,
    pub events: Vec<GameEvent>,
}

impl Game {
    pub fn create(
        id: Option<i32>,
        created_by: Option<Coach>,
        version: Version,
        played_at: NaiveDateTime,
        team_a: &Team,
        team_b: &Team,
    ) -> Result<Self, Error> {
        if team_a.version.ne(&version) || team_b.version.ne(&version) {
            return Err(Error::TeamsMustMatchGameVersion);
        }

        if team_a.coach.eq(&team_b.coach) {
            return Err(Error::SameCoachForBothTeams);
        }

        if let Some(team_a_last_game) = team_a.last_game_played() {
            if team_a_last_game.played_at.gt(&played_at) {
                return Err(Error::CanNotCreateGameBeforeAnotherAlreadyPlayed);
            }
        }

        if let Some(team_b_last_game) = team_b.last_game_played() {
            if team_b_last_game.played_at.gt(&played_at) {
                return Err(Error::CanNotCreateGameBeforeAnotherAlreadyPlayed);
            }
        }

        if team_a.game_playing.is_some() || team_b.game_playing.is_some() {
            return Err(Error::TeamAlreadyPlayingGame);
        }

        team_a.check_if_rules_compliant()?;
        team_b.check_if_rules_compliant()?;

        Ok(Self {
            id,
            version,
            created_by,
            played_at,
            closed_at: None,
            first_team: team_a.clone(),
            second_team: team_b.clone(),
            first_team_playing_players: team_a.available_players(),
            second_team_playing_players: team_b.available_players(),
            events: vec![],
        })
    }

    pub fn check_if_rules_compliant(&self) -> Result<(), Error> {
        self.first_team.check_if_rules_compliant()?;
        self.second_team.check_if_rules_compliant()?;

        Ok(())
    }

    pub fn generate_fans(&mut self) -> Result<u32, Error> {
        let mut game_fans = 0;

        let fan_factor = GameEvent::roll_fan_factor(&self.first_team);
        game_fans += fan_factor;
        self.process_event(GameEvent::FanFactor(
            TeamSummary::from(&self.first_team),
            fan_factor,
        ))?;

        let fan_factor = GameEvent::roll_fan_factor(&self.second_team);
        game_fans += fan_factor;
        self.process_event(GameEvent::FanFactor(
            TeamSummary::from(&self.second_team),
            fan_factor,
        ))?;

        Ok(game_fans)
    }

    pub fn team_fan_factor(&self, team_for: &TeamSummary) -> Option<u32> {
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
            self.team_fan_factor(&TeamSummary::from(&self.first_team)),
            self.team_fan_factor(&TeamSummary::from(&self.second_team)),
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

    pub fn process_event(&mut self, event: GameEvent) -> Result<(), Error> {
        match self.version {
            Version::V4 => Err(Error::UnsupportedVersion),
            Version::V5 => v5::process_game_event(self, event),
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
            games_played: vec![],
            game_playing: None,
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
            games_played: vec![],
            game_playing: None,
            dedicated_fans: 2,
            under_creation: false,
        };

        let played_at_str = "2020-09-05 23:56:04";
        let played_at = NaiveDateTime::parse_from_str(played_at_str, "%Y-%m-%d %H:%M:%S").unwrap();

        assert!(Game::create(None, None, Version::V5, played_at, &team_a, &team_a).is_err());

        let mut game =
            Game::create(None, None, Version::V5, played_at.clone(), &team_a, &team_b).unwrap();
        assert_eq!(game.first_team_playing_players.len(), 11);
        assert_eq!(game.second_team_playing_players.len(), 11);

        let another_team_b = Team {
            games_played: vec![GameSummary::from(&game)],
            ..team_b.clone()
        };
        let played_at_str_2 = "2012-09-05 23:56:04";
        let played_at_2 =
            NaiveDateTime::parse_from_str(played_at_str_2, "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(another_team_b.games_played.len(), 1);
        assert!(played_at.gt(&played_at_2));
        assert!(
            Game::create(
                None,
                None,
                Version::V5,
                played_at_2,
                &team_a,
                &another_team_b
            )
            .is_err()
        );

        let another_team_b = Team {
            game_playing: Some(GameSummary::from(&game)),
            ..team_b.clone()
        };
        assert!(another_team_b.game_playing.is_some());
        assert!(
            Game::create(None, None, Version::V5, played_at, &team_a, &another_team_b).is_err()
        );

        let fans = game.generate_fans().unwrap();
        assert_eq!(game.fans().unwrap(), fans);

        let weather = game.generate_weather().unwrap();
        assert_eq!(game.weather().unwrap(), weather);
    }
}
