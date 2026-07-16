use crate::actions::Success;
use crate::coaches::Coach;
use crate::errors::Error;
use crate::events::GameEvent;
use crate::players::Player;
use crate::teams::Team;
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub mod v5;
pub mod v5s3;

mod game_sequence;
mod post_game_sequence;
mod pre_game_sequence;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub title: Option<String>,
    pub version: Version,
    pub created_by: Option<Coach>,
    pub game_at: NaiveDateTime,
    pub started: bool,
    pub closed: bool,
    pub first_team: Team,
    pub second_team: Team,
    pub events: Vec<GameEvent>,
    pub needs_winner: bool,
}

impl Game {
    pub fn create(
        id: i32,
        created_by: Option<Coach>,
        version: Version,
        game_at: NaiveDateTime,
        team_a: &Team,
        team_b: &Team,
        needs_winner: bool,
    ) -> Result<Self, Error> {
        let game = Self {
            id,
            title: None,
            version,
            created_by,
            game_at,
            started: false,
            closed: false,
            first_team: team_a.clone(),
            second_team: team_b.clone(),
            events: vec![],
            needs_winner,
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

    pub fn score(&self) -> (usize, usize) {
        let mut first_team_count = 0;
        let mut second_team_count = 0;

        for event in self.events.iter() {
            match event {
                GameEvent::Success {
                    team_id,
                    success: Success::Touchdown,
                    ..
                } => {
                    if self.first_team.id.eq(team_id) {
                        first_team_count += 1;
                    }
                    if self.second_team.id.eq(team_id) {
                        second_team_count += 1;
                    }
                }

                _ => {}
            }
        }

        (first_team_count, second_team_count)
    }

    pub fn casualties(&self) -> (usize, usize) {
        let mut first_team_count = 0;
        let mut second_team_count = 0;

        for event in self.events.iter() {
            match event {
                GameEvent::Success {
                    team_id,
                    success: Success::Casualty,
                    ..
                } => {
                    if self.first_team.id.eq(team_id) {
                        first_team_count += 1;
                    }
                    if self.second_team.id.eq(team_id) {
                        second_team_count += 1;
                    }
                }

                _ => {}
            }
        }

        (first_team_count, second_team_count)
    }

    pub fn winner(&self) -> (bool, bool) {
        if !self.game_finished() {
            return (false, false);
        }

        self.is_winning()
    }

    pub fn playing_players(&self) -> (Vec<(i32, Player)>, Vec<(i32, Player)>) {
        (
            self.first_team.available_players(),
            self.second_team.available_players(),
        )
    }
}

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

impl Game {
    pub fn status(&self) -> GameStatus {
        if self.closed {
            return GameStatus::Closed;
        } else if !self.started {
            return GameStatus::Scheduled;
        } else if !self.pre_game_sequence_is_finished() {
            return GameStatus::PreGameSequence;
        } else if !self.game_finished() {
            return GameStatus::GameInProgress;
        } else if !self.post_game_sequence_is_finished() {
            return GameStatus::PostGameSequence;
        } else {
            return GameStatus::WaitingForValidation;
        }
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn end_first_half(&mut self) -> Result<(), Error> {
        self.process_event(GameEvent::HalfTime)
    }

    pub fn first_half_finished(&self) -> bool {
        self.events.contains(&GameEvent::HalfTime)
    }

    pub fn start_extra_time(&mut self) -> Result<(), Error> {
        self.process_event(GameEvent::ExtraTime)
    }

    pub fn second_half_finished(&self) -> bool {
        self.game_finished() || self.events.contains(&GameEvent::ExtraTime)
    }

    pub fn end_game(&mut self) -> Result<(), Error> {
        self.process_event(GameEvent::GameEnd)
    }

    pub fn game_finished(&self) -> bool {
        self.events.contains(&GameEvent::GameEnd)
    }

    pub fn close_game(&mut self) -> Result<(), Error> {
        let _ = self.process_event(GameEvent::GameClosure)?;

        self.closed = true;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inducements::Inducement;
    use crate::injuries::Injury;
    use crate::players::Player;
    use crate::positions::Position;
    use crate::prayers::PrayerToNuffle;
    use crate::rosters::Roster;
    use crate::staffs::Staff;
    use crate::versions::Version;
    use std::collections::HashMap;

    #[test]
    fn new_game_v5() {
        let coach_a = Coach {
            id: Some(1),
            name: "Me".to_string(),
            elo: None,
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
                (
                    1,
                    Player::new(Version::V5, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    2,
                    Player::new(Version::V5, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    3,
                    Player::new(Version::V5, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    4,
                    Player::new(Version::V5, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    5,
                    Player::new(Version::V5, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    6,
                    Player::new(Version::V5, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    7,
                    Player::new(Version::V5, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    8,
                    Player::new(Version::V5, Position::Thrower, Roster::WoodElf),
                ),
                (
                    9,
                    Player::new(Version::V5, Position::Thrower, Roster::WoodElf),
                ),
                (
                    10,
                    Player::new(Version::V5, Position::Wardancer, Roster::WoodElf),
                ),
                (
                    11,
                    Player::new(Version::V5, Position::Wardancer, Roster::WoodElf),
                ),
            ],
            dedicated_fans: 4,
            under_creation: false,
            in_offseason: false,
        };

        let coach_b = Coach {
            id: Some(2),
            name: "Him".to_string(),
            elo: None,
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
                (
                    1,
                    Player::new(Version::V5, Position::EagleWarriorLinewoman, Roster::Amazon),
                ),
                (
                    2,
                    Player::new(Version::V5, Position::EagleWarriorLinewoman, Roster::Amazon),
                ),
                (
                    3,
                    Player::new(Version::V5, Position::EagleWarriorLinewoman, Roster::Amazon),
                ),
                (
                    4,
                    Player::new(Version::V5, Position::EagleWarriorLinewoman, Roster::Amazon),
                ),
                (
                    5,
                    Player::new(Version::V5, Position::EagleWarriorLinewoman, Roster::Amazon),
                ),
                (
                    6,
                    Player::new(Version::V5, Position::PythonWarriorThrower, Roster::Amazon),
                ),
                (
                    7,
                    Player::new(Version::V5, Position::PythonWarriorThrower, Roster::Amazon),
                ),
                (
                    8,
                    Player::new(Version::V5, Position::PiranhaWarriorBlitzer, Roster::Amazon),
                ),
                (
                    9,
                    Player::new(Version::V5, Position::PiranhaWarriorBlitzer, Roster::Amazon),
                ),
                (
                    10,
                    Player::new(Version::V5, Position::JaguarWarriorBlocker, Roster::Amazon),
                ),
                (
                    11,
                    Player::new(Version::V5, Position::JaguarWarriorBlocker, Roster::Amazon),
                ),
            ],
            dedicated_fans: 2,
            under_creation: false,
            in_offseason: false,
        };

        let played_at_str = "2020-09-05 23:56:04";
        let played_at = NaiveDateTime::parse_from_str(played_at_str, "%Y-%m-%d %H:%M:%S").unwrap();

        let mut game = Game::create(
            -1,
            None,
            Version::V5,
            played_at.clone(),
            &team_a,
            &team_b,
            false,
        )
        .unwrap();
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
        assert_eq!(
            player,
            Player::new_journeyman(-3, Version::V5, Roster::WoodElf)
        );
        let (number, player) = other_game.second_team.players.pop().unwrap();
        assert_eq!(number, 0);
        assert_eq!(
            player,
            Player::new_journeyman(-2, Version::V5, Roster::Amazon)
        );

        let petty_cash = game.petty_cash().unwrap();
        assert_eq!(petty_cash, (10000, 0));

        let mut other_game = Game::create(
            -1,
            None,
            Version::V5,
            played_at.clone(),
            &team_b,
            &team_a,
            false,
        )
        .unwrap();
        other_game.start();
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
            .push_prayer(game.first_team.id, PrayerToNuffle::roll(&game.version))
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
        let agility = player.agility().unwrap();
        let _ = game.push_injury(
            game.first_team.id.clone(),
            player.id.clone(),
            Injury::NeckInjury,
        );
        assert_eq!(game.first_team.players[0].1.agility().unwrap(), agility + 1);
        game.cancel_last_event().unwrap();
        assert_eq!(player.agility().unwrap(), agility);

        game.push_success(
            game.first_team.id,
            game.first_team.players[9].1.id,
            Success::Touchdown,
        )
        .unwrap();
        game.push_success(
            game.first_team.id,
            game.first_team.players[10].1.id,
            Success::Touchdown,
        )
        .unwrap();
        game.push_success(
            game.second_team.id,
            game.second_team.players[8].1.id,
            Success::Touchdown,
        )
        .unwrap();
        assert_eq!(game.score(), (2, 1));

        game.end_game().unwrap();

        assert_eq!(game.winning_team().unwrap().id, game.first_team.id);

        game.generate_winnings(false, false).unwrap();
        assert_eq!(
            game.winnings(),
            (Some((fans / 2) + 20000), Some((fans / 2) + 10000))
        );

        let (first_delta, second_delta) = game.generate_dedicated_fans_updates().unwrap();
        assert!(first_delta >= 0);
        assert!(second_delta <= 0);
        assert_eq!(game.first_team.dedicated_fans, (4 + first_delta) as u8);
        assert_eq!(game.second_team.dedicated_fans, (2 + second_delta) as u8);
    }

    #[test]
    fn new_game_v5s3() {
        let coach_a = Coach {
            id: Some(1),
            name: "Me".to_string(),
            elo: None,
        };

        let team_a = Team {
            id: 1,
            version: Version::V5S3,
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
                (
                    1,
                    Player::new(Version::V5S3, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    2,
                    Player::new(Version::V5S3, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    3,
                    Player::new(Version::V5S3, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    4,
                    Player::new(Version::V5S3, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    5,
                    Player::new(Version::V5S3, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    6,
                    Player::new(Version::V5S3, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    7,
                    Player::new(Version::V5S3, Position::WoodElfLineman, Roster::WoodElf),
                ),
                (
                    8,
                    Player::new(Version::V5S3, Position::Thrower, Roster::WoodElf),
                ),
                (
                    9,
                    Player::new(Version::V5S3, Position::Thrower, Roster::WoodElf),
                ),
                (
                    10,
                    Player::new(Version::V5S3, Position::Wardancer, Roster::WoodElf),
                ),
                (
                    11,
                    Player::new(Version::V5S3, Position::Wardancer, Roster::WoodElf),
                ),
            ],
            dedicated_fans: 4,
            under_creation: false,
            in_offseason: false,
        };

        let coach_b = Coach {
            id: Some(2),
            name: "Him".to_string(),
            elo: None,
        };

        let team_b = Team {
            id: 2,
            version: Version::V5S3,
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
                (
                    1,
                    Player::new(
                        Version::V5S3,
                        Position::EagleWarriorLinewoman,
                        Roster::Amazon,
                    ),
                ),
                (
                    2,
                    Player::new(
                        Version::V5S3,
                        Position::EagleWarriorLinewoman,
                        Roster::Amazon,
                    ),
                ),
                (
                    3,
                    Player::new(
                        Version::V5S3,
                        Position::EagleWarriorLinewoman,
                        Roster::Amazon,
                    ),
                ),
                (
                    4,
                    Player::new(
                        Version::V5S3,
                        Position::EagleWarriorLinewoman,
                        Roster::Amazon,
                    ),
                ),
                (
                    5,
                    Player::new(
                        Version::V5S3,
                        Position::EagleWarriorLinewoman,
                        Roster::Amazon,
                    ),
                ),
                (
                    6,
                    Player::new(
                        Version::V5S3,
                        Position::PythonWarriorThrower,
                        Roster::Amazon,
                    ),
                ),
                (
                    7,
                    Player::new(
                        Version::V5S3,
                        Position::PythonWarriorThrower,
                        Roster::Amazon,
                    ),
                ),
                (
                    8,
                    Player::new(
                        Version::V5S3,
                        Position::PiranhaWarriorBlitzer,
                        Roster::Amazon,
                    ),
                ),
                (
                    9,
                    Player::new(
                        Version::V5S3,
                        Position::PiranhaWarriorBlitzer,
                        Roster::Amazon,
                    ),
                ),
                (
                    10,
                    Player::new(
                        Version::V5S3,
                        Position::JaguarWarriorBlocker,
                        Roster::Amazon,
                    ),
                ),
                (
                    11,
                    Player::new(
                        Version::V5S3,
                        Position::JaguarWarriorBlocker,
                        Roster::Amazon,
                    ),
                ),
            ],
            dedicated_fans: 2,
            under_creation: false,
            in_offseason: false,
        };

        let played_at_str = "2020-09-05 23:56:04";
        let played_at = NaiveDateTime::parse_from_str(played_at_str, "%Y-%m-%d %H:%M:%S").unwrap();

        let mut game = Game::create(
            1,
            None,
            Version::V5S3,
            played_at.clone(),
            &team_a,
            &team_b,
            false,
        )
        .unwrap();

        let _ = game.start();
        assert!(game.started);
        assert!(matches!(game.status(), GameStatus::PreGameSequence));

        let fans = game.generate_fans().unwrap();
        assert_eq!(game.fans().unwrap(), fans);

        let weather = game.generate_weather().unwrap();
        assert_eq!(game.weather().unwrap(), weather);

        let journey_men = game.generate_journeymen().unwrap();
        assert_eq!(journey_men, (0, 0));
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
        game.push_success(
            game.first_team.id,
            game.first_team.players[9].1.id,
            Success::Touchdown,
        )
        .unwrap();
        game.push_success(
            game.first_team.id,
            game.first_team.players[10].1.id,
            Success::Touchdown,
        )
        .unwrap();
        game.push_success(
            game.second_team.id,
            game.second_team.players[8].1.id,
            Success::Touchdown,
        )
        .unwrap();
        assert_eq!(game.score(), (2, 1));

        game.end_game().unwrap();

        assert_eq!(game.winning_team().unwrap().id, game.first_team.id);

        game.generate_winnings(false, true).unwrap();
        assert_eq!(
            game.winnings(),
            (Some((fans / 2) + 20000 + 10000), Some((fans / 2) + 10000))
        );
    }
}
