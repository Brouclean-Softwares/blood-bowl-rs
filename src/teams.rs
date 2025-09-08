use crate::errors::Error;
use crate::players::Player;
use crate::positions::Position;
use crate::rosters::{Roster, Staff};
use crate::versions::Version;
use std::collections::HashMap;

pub mod v5;

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

    pub fn value(&self) -> i32 {
        todo!()
    }

    pub fn current_value(&self) -> i32 {
        todo!()
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

        if let Some(roster_definition) = roster.definition(Some(version)) {
            if dedicated_fans < roster_definition.dedicated_fans_information.initial {
                return Err(Error::TeamCreationError(String::from("NotEnoughFans")));
            }

            if dedicated_fans > roster_definition.dedicated_fans_information.maximum {
                return Err(Error::TeamCreationError(String::from("TooMuchFans")));
            }

            for (staff, staff_quantity) in staff_quantities.clone() {
                if let Some(roster_staff_information) =
                    roster_definition.staff_information.get(&staff)
                {
                    if roster_staff_information.maximum < staff_quantity {
                        return Err(Error::TeamCreationError(String::from(
                            "StaffExceededMaximum",
                        )));
                    }
                } else {
                    return Err(Error::TeamCreationError(String::from("StaffNotInRoster")));
                }
            }

            let mut players_number = 0;
            let mut big_men_number = 0;

            for (team_position, team_quantity) in team_positions.clone() {
                players_number += team_quantity;

                if !roster_definition.positions.contains(&team_position) {
                    return Err(Error::TeamCreationError(String::from(
                        "PositionNotInRoster",
                    )));
                }

                if let Some(position_definition) = team_position.definition(Some(version)) {
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
                } else {
                    return Err(Error::TeamCreationError(String::from("PositionNotDefined")));
                }
            }

            if players_number < 11 {
                return Err(Error::TeamCreationError(String::from("NotEnoughPlayers")));
            }
        } else {
            return Err(Error::TeamCreationError(String::from("RosterNotExists")));
        }

        match version {
            Version::V4 => Err(Error::TeamCreationError(String::from("UnsupportedVersion"))),
            Version::V5 => v5::create_new(
                roster,
                treasury,
                staff_quantities,
                team_positions,
                dedicated_fans,
            ),
        }
    }
}
