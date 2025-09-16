use crate::events::GameEvent;
use crate::players::Player;
use crate::teams::Team;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: Option<i32>,
    pub teams: Vec<Team>,
    pub playing_players: HashMap<Team, Vec<Player>>,
    pub game_events: Vec<GameEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::positions::Position;
    use crate::rosters::{Roster, Staff};
    use crate::versions::Version;

    #[test]
    fn new_game() {
        let team_a = Team {
            id: None,
            version: Version::V5,
            roster: Roster::WoodElf,
            name: "Woodies".to_string(),
            coach_id: None,
            coach_name: "Moi".to_string(),
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

        team_a.check_if_rules_compliant().unwrap();
    }
}
