use crate::coaches::Coach;
use crate::errors::Error;
use crate::players::Player;
use crate::positions::Position;
use crate::rosters::{Roster, RosterDefinition};
use crate::staffs::{Staff, StaffInformation};
use crate::versions::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub mod v5;

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
        let staff_information = match self.roster_definition()?.get_staff_information(&staff) {
            None => Err(Error::StaffNotInRoster),
            Some(staff_information) => Ok(staff_information),
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
            let position_definition = player.position_definition()?;

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

                let player_to_buy = Player::new(self.version, *position_to_buy, self.roster);
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
            players_value += player.value()?;
        }

        Ok(players_value)
    }

    pub fn players_current_value(&self) -> Result<u32, Error> {
        let mut players_value = 0;

        for (_, player) in self.players.iter() {
            players_value += player.current_value()?;
        }

        Ok(players_value)
    }

    pub fn player_by_id(&self, player_id: i32) -> Option<(i32, Player)> {
        self.players
            .iter()
            .find(|(_, player)| player_id.eq(&player.id))
            .and_then(|player| Some(player.clone()))
    }

    pub fn update_player_number(&mut self, player_id: i32, player_number: i32) {
        for (number, player) in self.players.iter_mut() {
            if player.id.eq(&player_id) {
                *number = player_number;

                return;
            }
        }
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

    pub fn min_players_id(&self) -> Option<i32> {
        let mut sorted_players = self.players.clone();
        sorted_players.sort_by(|(_, player_a), (_, player_b)| player_b.id.cmp(&player_a.id));
        sorted_players.pop().and_then(|(_, player)| Some(player.id))
    }

    pub fn can_buy_journeyman(&self) -> Result<bool, Error> {
        self.can_buy_position(&self.roster_definition()?.journeyman_position)
    }

    pub fn add_journeyman_with_number(&mut self, team_number: i32) -> Player {
        let player = Player::new_journeyman(
            self.min_players_id().unwrap_or(0) - 1,
            self.version,
            self.roster,
        );
        self.players.push((team_number, player.clone()));

        player
    }

    pub fn journeymen_number(&self) -> u8 {
        self.available_players()
            .iter()
            .filter(|(_, player)| player.is_journeyman)
            .count() as u8
    }

    pub fn stars_number(&self) -> u8 {
        self.available_players()
            .iter()
            .filter(|(_, player)| player.is_star_player)
            .count() as u8
    }

    pub fn add_star_player_with_number(&mut self, team_number: i32, position: Position) -> Player {
        let player = Player::new_star_player(
            self.min_players_id().unwrap_or(0) - 1,
            self.version,
            position,
            self.roster,
        );
        self.players.push((team_number, player.clone()));

        player
    }

    pub fn staff_value(&self) -> Result<u32, Error> {
        let mut staff_value = 0;

        let roster_definition = self.roster.definition(self.version)?;

        for (staff, quantity) in self.staff.clone() {
            let staff_price = roster_definition
                .get_staff_information(&staff)
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

                players.push((number, Player::new(version, position, roster)));
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
                .get_staff_information(&staff)
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
                > player.position_definition()?.maximum_quantity
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
    use crate::rosters::Roster;
    use crate::versions::Version;

    #[test]
    fn team_ok() {
        let mut team_a = Team {
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
        };

        team_a.check_if_rules_compliant().unwrap();

        assert_eq!(team_a.players[5].0, 6);
        team_a.players[5].1.id = 10;
        team_a.update_player_number(10, 50);
        assert_eq!(team_a.players[5].0, 50);
    }
}
