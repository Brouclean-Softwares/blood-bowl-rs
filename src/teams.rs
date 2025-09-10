use crate::errors::Error;
use crate::players::Player;
use crate::positions::Position;
use crate::rosters::{Roster, Staff};
use crate::versions::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod v5;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Team {
    pub version: Version,
    pub roster: Roster,
    pub name: String,
    pub coach_name: String,
    pub treasury: i32,
    pub external_logo_url: Option<String>,
    pub staff: HashMap<Staff, u8>,
    pub players: Vec<Player>,
    pub dedicated_fans: u8,
}

impl Team {
    pub fn initial_treasury(_version: Version) -> i32 {
        1000000
    }

    pub fn players_value(&self) -> Result<u32, Error> {
        let mut players_value = 0;

        for player in self.players.clone() {
            let position_price = player
                .position
                .definition(Some(self.version))
                .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?
                .cost;
            players_value += position_price;
        }

        Ok(players_value)
    }

    pub fn staff_value(&self) -> Result<u32, Error> {
        let mut staff_value = 0;

        let roster_definition = self
            .roster
            .definition(Some(self.version))
            .ok_or(Error::TeamCreationError(String::from("RosterNotExists")))?;

        for (staff, quantity) in self.staff.clone() {
            let staff_price = roster_definition
                .staff_information
                .get(&staff)
                .ok_or(Error::TeamCreationError(String::from("StaffNotInRoster")))?
                .price;

            staff_value += staff_price * quantity as u32;
        }

        Ok(staff_value)
    }

    pub fn value(&self) -> Result<u32, Error> {
        Ok(self.players_value()? + self.staff_value()?)
    }

    pub fn create_new(
        version: Version,
        roster: Roster,
        treasury: i32,
        staff_quantities: HashMap<Staff, u8>,
        team_positions: HashMap<Position, u8>,
        dedicated_fans: u8,
    ) -> Result<Self, Error> {
        if treasury < 0 {
            return Err(Error::TeamCreationError(String::from("TreasuryExceeded")));
        }

        let roster_definition = roster
            .definition(Some(version))
            .ok_or(Error::TeamCreationError(String::from("RosterNotExists")))?;

        if dedicated_fans < roster_definition.dedicated_fans_information.initial {
            return Err(Error::TeamCreationError(String::from("NotEnoughFans")));
        }

        if dedicated_fans > roster_definition.dedicated_fans_information.maximum {
            return Err(Error::TeamCreationError(String::from("TooMuchFans")));
        }

        for (staff, staff_quantity) in staff_quantities.clone() {
            let roster_staff_information = roster_definition
                .staff_information
                .get(&staff)
                .ok_or(Error::TeamCreationError(String::from("StaffNotInRoster")))?;

            if roster_staff_information.maximum < staff_quantity {
                return Err(Error::TeamCreationError(String::from(
                    "StaffExceededMaximum",
                )));
            }
        }

        let mut big_men_number = 0;

        for (team_position, team_quantity) in team_positions.clone() {
            if !roster_definition.positions.contains(&team_position) {
                return Err(Error::TeamCreationError(String::from(
                    "PositionNotInRoster",
                )));
            }

            let position_definition = team_position
                .definition(Some(version))
                .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?;

            if position_definition.maximum_quantity < team_quantity {
                return Err(Error::TeamCreationError(String::from(
                    "PositionMaxExceeded",
                )));
            }

            if position_definition.is_big_man {
                big_men_number += team_quantity;

                if big_men_number > roster_definition.maximum_big_men_quantity {
                    return Err(Error::TeamCreationError(String::from("TooMuchBigMen")));
                }
            }
        }

        let mut players: Vec<Player> = Vec::new();
        let mut number: i32 = 0;

        for (position, quantity) in team_positions {
            for _i in 0..quantity {
                number += 1;

                players.push(Player {
                    version: Version::V5,
                    position,
                    name: "".to_string(),
                    number,
                });
            }
        }

        if number < 11 {
            return Err(Error::TeamCreationError(String::from("NotEnoughPlayers")));
        }

        let team = Team {
            version: Version::V5,
            roster,
            name: "".to_string(),
            coach_name: "".to_string(),
            treasury,
            external_logo_url: None,
            staff: staff_quantities,
            players,
            dedicated_fans,
        };

        let expected_remaining_treasury = match version {
            Version::V4 => Err(Error::TeamCreationError(String::from("UnsupportedVersion")))?,
            Version::V5 => Ok(v5::remaining_treasury(&team)?)?,
        };

        if expected_remaining_treasury != team.treasury {
            return Err(Error::TeamCreationError(String::from("IncorrectTreasury")));
        }

        Ok(team)
    }
}
