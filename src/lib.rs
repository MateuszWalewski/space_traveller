mod constants;
mod custom_iteration;
mod player;
pub mod tools;

use crate::custom_iteration::CycleIter;
use crate::player::Player;

pub struct PreGameManager {
    players: Vec<Player>,
}

pub struct GameManager {
    players: Vec<Player>,
}

pub struct PostGameManager {
    players: Vec<Player>,
}

impl PreGameManager {
    pub fn add_players<R: tools::InputReader>(mut self, reader: &mut R) -> GameManager {
        for _ in 0..constants::MAX_NUMBER_OF_PLAYERS {
            println!("Please type a nick for the new player");
            let mut nick = String::new();
            reader
                .read_line(&mut nick)
                .expect("Failed to parse a choice");

            println!("The player: {} has been added.", nick);
            self.players.push(Player::new(nick));
        }
        GameManager {
            players: self.players,
        }
    }
}

impl GameManager {
    pub fn new() -> PreGameManager {
        PreGameManager {
            players: Vec::new(),
        }
    }

    pub fn number_of_players(&self) -> usize {
        self.players.len()
    }

    pub fn start_game<R: tools::InputReader>(mut self, reader: &mut R) -> PostGameManager {
        for player in self.players.cycle_iter() {
            player.take_turn(reader);
            if player.score() >= constants::WINNING_VALUE {
                player.mark_as_winner();
                return PostGameManager {
                    players: self.players,
                };
            }
        }
        PostGameManager {
            players: self.players,
        }
    }
}

impl PostGameManager {
    pub fn get_winner(&self) -> &Player {
        self.players
            .iter()
            .find(|x| x.is_winner())
            .expect("On this stage we have the winner for sure!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addition_of_players_within_specified_limit_works() {
        let pred_input = "tom78\njohn99\n";
        let mut custom_input_reader = tools::CustomInputReader::new(pred_input);
        let gm = GameManager::new();
        let gc = gm.add_players(&mut custom_input_reader);
        assert_eq!(gc.number_of_players(), 2);
    }

}
