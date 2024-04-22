use std::{error::Error, io};

mod constants;
mod custom_iteration;
mod player;
pub mod tools;

use crate::custom_iteration::CycleIter;
use crate::player::Player;

pub struct GameController {
    players: Vec<Player>,
}

impl GameController {
    pub fn new() -> Self {
        GameController {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, nick: String) -> Result<(), Box<dyn Error>> {
        if self.players.len() >= constants::MAX_NUMBER_OF_PLAYERS {
            let mut errorMsg = String::from("The maximum number of players is: ");
            errorMsg.push_str(constants::MAX_NUMBER_OF_PLAYERS.to_string().as_str());
            return Err(errorMsg.into());
        }
        self.players.push(Player::new(nick));
        Ok(())
    }

    pub fn number_of_players(&self) -> usize {
        self.players.len()
    }

    pub fn start_game<R: tools::InputReader>(&mut self, reader: &mut R) -> Option<&Player> {
        for player in self.players.cycle_iter() {
            player.take_turn(reader);
            if player.score() == constants::WINNING_VALUE {
                return Some(player);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addition_of_players_within_specified_limit_works() {
        let player = String::from("steve12");
        let mut gc = GameController::new();
        for _ in 0..constants::MAX_NUMBER_OF_PLAYERS {
            gc.add_player(player.clone()).unwrap_or_else(|err| {
                eprintln!("Error occured: {err}.");
            });
        }

        assert_eq!(gc.number_of_players(), 2);
    }

    #[test]
    fn addition_of_more_than_specified_number_of_players_doesnt_make_any_effect_works() {
        let player = String::from("steve12");
        let mut gc = GameController::new();
        for _ in 0..constants::MAX_NUMBER_OF_PLAYERS + 5 {
            gc.add_player(player.clone()).unwrap_or_else(|err| {
                eprintln!("Error occured: {err}.");
            });
        }

        assert_eq!(gc.number_of_players(), 2);
    }
}
