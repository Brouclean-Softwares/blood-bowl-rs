use crate::coaches::Coach;
use crate::errors::Error;
use crate::games::GameSummary;
use crate::players::Player;
use crate::positions::Position;
use crate::rosters::{Roster, RosterDefinition, Staff, StaffInformation};
use crate::versions::Version;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub mod v5;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamSummary {
    pub id: i32,
    pub version: Version,
    pub roster: Roster,
    pub name: String,
    pub coach_name: String,
    pub external_logo_url: Option<String>,
    pub value: u32,
    pub current_value: u32,
    pub treasury: i32,
    pub last_game_played_date_time: Option<NaiveDateTime>,
    pub is_playing_a_game: bool,
}

impl Default for TeamSummary {
    fn default() -> Self {
        Self {
            id: 0,
            version: Version::V4,
            roster: Roster::Amazon,
            name: "".to_string(),
            coach_name: "".to_string(),
            external_logo_url: None,
            value: 0,
            current_value: 0,
            treasury: 0,
            last_game_played_date_time: None,
            is_playing_a_game: false,
        }
    }
}

impl PartialEq for TeamSummary {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TeamSummary {}

impl Hash for TeamSummary {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl From<&Team> for TeamSummary {
    fn from(team: &Team) -> Self {
        let cloned_team = team.clone();

        let value = cloned_team.value().unwrap_or_default();
        let current_value = cloned_team.current_value().unwrap_or_default();
        let last_game_played_date_time = cloned_team
            .last_game_played()
            .and_then(|game| game.closed_at);

        Self {
            id: cloned_team.id,
            version: cloned_team.version,
            roster: cloned_team.roster,
            name: cloned_team.name,
            coach_name: cloned_team.coach.name,
            external_logo_url: cloned_team.external_logo_url,
            value,
            current_value,
            treasury: cloned_team.treasury,
            last_game_played_date_time,
            is_playing_a_game: cloned_team.game_playing.is_some(),
        }
    }
}

impl From<&TeamSummary> for Team {
    fn from(team_summary: &TeamSummary) -> Self {
        let cloned_team_summary = team_summary.clone();

        Self {
            id: cloned_team_summary.id,
            version: cloned_team_summary.version,
            roster: cloned_team_summary.roster,
            name: cloned_team_summary.name,
            coach: Coach::from_name(&*cloned_team_summary.coach_name),
            treasury: cloned_team_summary.treasury,
            external_logo_url: cloned_team_summary.external_logo_url,
            staff: Default::default(),
            players: vec![],
            games_played: vec![],
            game_playing: None,
            games_scheduled: vec![],
            dedicated_fans: 0,
            under_creation: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Team {
    pub id: i32,
    pub version: Version,
    pub roster: Roster,
    pub name: String,
    pub coach: Coach,
    pub treasury: i32,
    pub external_logo_url: Option<String>,
    pub staff: HashMap<Staff, u8>,
    pub players: Vec<(i32, Player)>,
    pub games_played: Vec<GameSummary>,
    pub game_playing: Option<GameSummary>,
    pub games_scheduled: Vec<GameSummary>,
    pub dedicated_fans: u8,
    pub under_creation: bool,
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Team {}

impl Hash for Team {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Team {
    pub fn initial_treasury(_version: &Version) -> i32 {
        1000000
    }

    pub fn minimum_players(_version: &Version) -> usize {
        11
    }

    pub fn maximum_players(_version: &Version) -> usize {
        16
    }

    pub fn remaining_available_players_number(&self) -> usize {
        Team::maximum_players(&self.version) - self.players.len()
    }

    pub fn roster_definition(&self) -> Result<RosterDefinition, Error> {
        self.roster.definition(self.version)
    }

    pub fn staff_information(&self, staff: &Staff) -> Result<StaffInformation, Error> {
        let staff_information = match self.roster_definition()?.staff_information.get(&staff) {
            None => Err(Error::StaffNotInRoster),
            Some(&staff_information) => Ok(staff_information),
        }?;

        Ok(staff_information)
    }

    pub fn staff_quantity(&self, staff: &Staff) -> u8 {
        self.staff
            .get(&staff)
            .and_then(|&quantity| Some(quantity))
            .unwrap_or(0)
    }

    pub fn buy_staff(&mut self, staff: &Staff) -> Result<u8, Error> {
        let new_staff_quantity = self.staff_quantity(staff) + 1;
        let staff_maximum = self.staff_information(staff)?.maximum;
        let staff_price = self.staff_information(staff)?.price(self.under_creation);
        let treasury = self.treasury;

        if new_staff_quantity > staff_maximum {
            return Err(Error::StaffExceededMaximum);
        }

        if treasury < staff_price as i32 {
            return Err(Error::TreasuryExceeded);
        }

        self.staff.insert(*staff, new_staff_quantity);
        self.treasury = treasury - staff_price as i32;

        Ok(new_staff_quantity)
    }

    pub fn can_buy_staff(&self, staff: &Staff) -> Result<bool, Error> {
        let current_staff_quantity = self.staff_quantity(staff);
        let staff_maximum = self.staff_information(staff)?.maximum;
        let staff_price = self.staff_information(staff)?.price(self.under_creation);
        let treasury = self.treasury;

        Ok(current_staff_quantity < staff_maximum && treasury >= staff_price as i32)
    }

    pub fn position_number_under_contract(&self, position: &Position) -> u8 {
        let mut position_number_under_contract = 0;

        for (_, player) in self.players.iter() {
            if player.position.eq(position) {
                position_number_under_contract += 1;
            }
        }

        position_number_under_contract
    }

    pub fn big_men_number_under_contract(&self) -> Result<u8, Error> {
        let mut big_men_number_under_contract = 0;

        for (_, player) in self.players.iter() {
            let position_definition = player.position.definition(self.version, self.roster)?;

            if position_definition.is_big_man {
                big_men_number_under_contract += 1;
            }
        }

        Ok(big_men_number_under_contract)
    }

    pub fn buy_position(&mut self, position_to_buy: &Position) -> Result<(i32, Player), Error> {
        if self.remaining_available_players_number() <= 0 {
            return Err(Error::TooMuchPlayers);
        }

        for position in self.roster_definition()?.positions {
            if position.eq(position_to_buy) {
                let position_definition = position.definition(self.version, self.roster)?;
                let position_cost = position_definition.cost;
                let max_big_men = self
                    .roster
                    .definition(self.version)?
                    .maximum_big_men_quantity;

                if self.treasury < position_cost as i32 {
                    return Err(Error::TreasuryExceeded);
                }

                if self.position_number_under_contract(&position)
                    >= position_definition.maximum_quantity
                {
                    return Err(Error::PositionMaxExceeded);
                }

                if position_definition.is_big_man
                    && self.big_men_number_under_contract()? >= max_big_men
                {
                    return Err(Error::TooMuchBigMen);
                }

                let player_to_buy = Player::new(self.version, *position_to_buy);
                let number = 0;

                self.players.push((number, player_to_buy.clone()));

                self.treasury = self.treasury - position_cost as i32;

                return Ok((number, player_to_buy));
            }
        }

        Err(Error::PositionNotInRoster)
    }

    pub fn can_buy_player(&self) -> Result<bool, Error> {
        for (_, _, buyable) in self.positions_buyable()? {
            if buyable {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn can_buy_position(&self, position_to_buy: &Position) -> Result<bool, Error> {
        for (position, _, buyable) in self.positions_buyable()? {
            if position.eq(position_to_buy) && buyable {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn positions_buyable(&self) -> Result<Vec<(Position, u32, bool)>, Error> {
        let mut positions_buyable: Vec<(Position, u32, bool)> = Vec::new();

        if self.remaining_available_players_number() > 0 {
            for position in self.roster_definition()?.positions {
                let position_definition = position.definition(self.version, self.roster)?;
                let position_cost = position_definition.cost;
                let max_big_men = self
                    .roster
                    .definition(self.version)?
                    .maximum_big_men_quantity;

                let position_cost_is_ok = self.treasury >= position_cost as i32;

                let position_number_is_ok = self.position_number_under_contract(&position)
                    < position_definition.maximum_quantity;

                let big_men_number_is_ok = !position_definition.is_big_man
                    || (position_definition.is_big_man
                        && self.big_men_number_under_contract()? < max_big_men);

                let position_buyable =
                    position_cost_is_ok && position_number_is_ok && big_men_number_is_ok;

                if position_number_is_ok && big_men_number_is_ok {
                    positions_buyable.push((position, position_cost, position_buyable));
                }
            }
        }

        Ok(positions_buyable)
    }

    pub fn players_value(&self) -> Result<u32, Error> {
        let mut players_value = 0;

        for (_, player) in self.players.iter() {
            players_value += player.value(&self.roster)?;
        }

        Ok(players_value)
    }

    pub fn players_current_value(&self) -> Result<u32, Error> {
        let mut players_value = 0;

        for (_, player) in self.players.iter() {
            players_value += player.current_value(&self.roster)?;
        }

        Ok(players_value)
    }

    pub fn number_of_players(&self) -> u8 {
        self.players.len() as u8
    }

    pub fn number_of_available_players(&self) -> u8 {
        self.available_players().len() as u8
    }

    pub fn available_players(&self) -> Vec<(i32, Player)> {
        let mut available_players: Vec<(i32, Player)> = Vec::new();

        for (number, player) in self.players.iter() {
            if player.available() {
                available_players.push((number.clone(), player.clone()));
            }
        }

        available_players
    }

    pub fn sort_players_by_number(&self) -> Vec<(i32, Player)> {
        let mut sorted_players = self.players.clone();
        sorted_players.sort_by(|(number_a, _), (number_b, _)| number_a.cmp(number_b));
        sorted_players
    }

    pub fn staff_value(&self) -> Result<u32, Error> {
        let mut staff_value = 0;

        let roster_definition = self.roster.definition(self.version)?;

        for (staff, quantity) in self.staff.clone() {
            let staff_price = roster_definition
                .staff_information
                .get(&staff)
                .ok_or(Error::StaffNotInRoster)?
                .price(true);

            staff_value += staff_price * quantity as u32;
        }

        Ok(staff_value)
    }

    pub fn value(&self) -> Result<u32, Error> {
        Ok(self.players_value()? + self.staff_value()?)
    }

    pub fn current_value(&self) -> Result<u32, Error> {
        Ok(self.players_current_value()? + self.staff_value()?)
    }

    pub fn last_game_played(&self) -> Option<GameSummary> {
        let mut sorted_games_played = self.games_played.clone();
        sorted_games_played.sort();
        sorted_games_played.pop()
    }

    pub fn create_new(
        coach: Coach,
        version: Version,
        roster: Roster,
        treasury: i32,
        staff_quantities: HashMap<Staff, u8>,
        team_positions: HashMap<Position, u8>,
        dedicated_fans: u8,
    ) -> Result<Self, Error> {
        let mut players: Vec<(i32, Player)> = Vec::new();
        let mut number: i32 = 0;

        for (position, quantity) in team_positions {
            for _i in 0..quantity {
                number += 1;

                players.push((number, Player::new(version, position)));
            }
        }

        let team = Team {
            id: -1,
            version,
            roster,
            name: "".to_string(),
            coach,
            treasury,
            external_logo_url: None,
            staff: staff_quantities,
            players,
            games_played: vec![],
            game_playing: None,
            games_scheduled: vec![],
            dedicated_fans,
            under_creation: true,
        };

        team.check_if_rules_compliant()?;

        Ok(team)
    }

    pub fn check_if_rules_compliant(&self) -> Result<(), Error> {
        if self.treasury < 0 {
            return Err(Error::TreasuryExceeded);
        }

        let roster_definition = self.roster.definition(self.version)?;

        if self.dedicated_fans < roster_definition.dedicated_fans_information.initial {
            return Err(Error::NotEnoughFans);
        }

        if self.dedicated_fans > roster_definition.dedicated_fans_information.maximum {
            return Err(Error::TooMuchFans);
        }

        for (staff, staff_quantity) in self.staff.iter() {
            let roster_staff_information = roster_definition
                .staff_information
                .get(&staff)
                .ok_or(Error::StaffNotInRoster)?;

            if roster_staff_information.maximum < *staff_quantity {
                return Err(Error::StaffExceededMaximum);
            }
        }

        if self.number_of_players() > Team::maximum_players(&self.version) as u8 {
            return Err(Error::TooMuchPlayers);
        }

        if self.big_men_number_under_contract()? > roster_definition.maximum_big_men_quantity {
            return Err(Error::TooMuchBigMen);
        }

        for (_, player) in self.players.iter() {
            if self.position_number_under_contract(&player.position)
                > player
                    .position
                    .definition(self.version, self.roster)?
                    .maximum_quantity
            {
                return Err(Error::PositionMaxExceeded);
            }
        }

        if self.under_creation {
            if self.number_of_players() < Team::minimum_players(&self.version) as u8 {
                return Err(Error::NotEnoughPlayers);
            }

            let expected_remaining_treasury = match self.version {
                Version::V4 => Err(Error::UnsupportedVersion)?,
                Version::V5 => Ok(v5::expected_remaining_treasury_at_creation(&self)?)?,
            };

            if expected_remaining_treasury != self.treasury {
                return Err(Error::IncorrectTreasury);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::positions::Position;
    use crate::rosters::{Roster, Staff};
    use crate::versions::Version;

    #[test]
    fn team_ok() {
        let team_a = Team {
            id: 1,
            version: Version::V5,
            roster: Roster::WoodElf,
            name: "Woodies".to_string(),
            coach: Coach {
                id: None,
                name: "Moi".to_string(),
            },
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
            games_scheduled: vec![],
            dedicated_fans: 4,
            under_creation: false,
        };

        team_a.check_if_rules_compliant().unwrap();
    }
}
