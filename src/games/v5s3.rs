use crate::errors::Error;
use crate::games::Game;

pub fn generate_winnings(
    game: &mut Game,
    first_team_stalled: bool,
    second_team_stalled: bool,
) -> Result<Option<(u32, u32)>, Error> {
    if let Some(fans) = game.fans() {
        let (first_team_score, second_team_score) = game.score();
        let (first_team_current_winnings, second_team_current_winnings) = game.winnings();

        let mut first_team_winnings = (fans / 2) + (first_team_score * 10000) as u32;

        if !first_team_stalled {
            first_team_winnings += 10000;
        }

        if first_team_current_winnings.is_none() {
            game.push_winnings(game.first_team.id, first_team_winnings)?;
        }

        let mut second_team_winnings = (fans / 2) + (second_team_score * 10000) as u32;

        if !second_team_stalled {
            second_team_winnings += 10000;
        }

        if second_team_current_winnings.is_none() {
            game.push_winnings(game.second_team.id, second_team_winnings)?;
        }

        Ok(Some((first_team_winnings, second_team_winnings)))
    } else {
        Ok(None)
    }
}
