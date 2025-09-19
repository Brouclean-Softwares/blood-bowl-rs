use crate::errors::Error;
use crate::events::GameEvent;
use crate::games::Game;

pub fn process_game_event(game: &mut Game, game_event: GameEvent) -> Result<(), Error> {
    match game_event {
        event => game.events.push(event),
    };

    Ok(())
}
