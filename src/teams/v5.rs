use crate::errors::Error;
use crate::players::Player;
use crate::positions::Position;
use crate::rosters::{Roster, Staff};
use crate::teams::Team;
use crate::versions::Version;
use std::collections::HashMap;

pub(crate) fn create_new(
    roster: Roster,
    treasury: i32,
    staff_quantities: HashMap<Staff, u8>,
    positions: HashMap<Position, u8>,
    dedicated_fans: u8,
) -> Result<Team, Error> {
    let mut players: Vec<Player> = Vec::new();
    let mut number: i32 = 0;

    for (position, quantity) in positions {
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

    Ok(Team {
        version: Version::V5,
        roster,
        name: "".to_string(),
        coach_name: "".to_string(),
        treasury,
        external_logo_url: None,
        staff: staff_quantities,
        players,
        dedicated_fans,
    })
}
