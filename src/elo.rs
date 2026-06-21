use crate::games::Game;
use std::cmp::min;

pub const NAF_INITIAL_ELO: i32 = 150;

pub fn new_naf_elo_from_game(
    game: &Game,
    competition_coaches_number: Option<usize>,
    competition_maximum_coaches_number: Option<usize>,
) -> (i32, i32) {
    let first_team_elo = game.first_team.coach.elo.unwrap_or(NAF_INITIAL_ELO);
    let second_team_elo = game.second_team.coach.elo.unwrap_or(NAF_INITIAL_ELO);

    let (first_team_points, second_team_points) = match game.winner() {
        (true, false) => (1.00, 0.00),
        (false, true) => (0.00, 1.00),
        _ => (0.5, 0.5),
    };

    let first_team_new_elo = new_naf_elo(
        first_team_elo,
        second_team_elo,
        first_team_points,
        competition_coaches_number,
        competition_maximum_coaches_number,
    );

    let second_team_new_elo = new_naf_elo(
        second_team_elo,
        first_team_elo,
        second_team_points,
        competition_coaches_number,
        competition_maximum_coaches_number,
    );

    (first_team_new_elo, second_team_new_elo)
}

fn new_naf_elo(
    own_elo: i32,
    adversary_elo: i32,
    points: f64,
    competition_coaches_number: Option<usize>,
    competition_maximum_coaches_number: Option<usize>,
) -> i32 {
    let victory_probability: f64 =
        1.00 / (10f64.powf((adversary_elo as f64 - own_elo as f64) / 150.00) + 1.00);

    let k_value = f64::sqrt(min(
        competition_coaches_number.unwrap_or(2),
        competition_maximum_coaches_number.unwrap_or(2),
    ) as f64)
        * 2.00;

    own_elo + (k_value * (points - victory_probability)) as i32
}
