use std::io;
mod custom_iteration;

pub use crate::custom_iteration::CycleIter;

struct Player {
    name: String,
    score: u32,
}

pub struct GameController {
    players: Vec<Player>,
}

impl GameController {
    pub fn new() -> Self {
        GameController {
            players: Vec::new(),
        }
    }

    pub fn add_players(&mut self) {
        while self.players.len() < 2 {
            println!("Please enter player nick:");
            let mut nick = String::new();
            io::stdin()
                .read_line(&mut nick)
                .expect("Failed to read nick");
            self.players.push(Player {
                name: nick,
                score: 0,
            });
        }
        println!("Players added! You can start the game!");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addition_of_players_works() {
        let gc = GameController::new();


        assert_eq!(4, 4);
    }
}
