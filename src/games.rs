use crate::teams::Team;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub team_a: Team,
    pub team_b: Team,
}

pub enum GameEvent {}
