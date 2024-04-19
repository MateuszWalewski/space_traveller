use std::io;
use space_traveller::CycleIter;

struct Player {
    name: String,
    score: u32,
}

struct GameController {
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

fn main() {
    let mut gc = GameController::new();
    gc.add_players();

    gc.add_players();

    let container = vec![1, 4, 5, 7];

    for i in container.cycle_iter().take(9) {
        println!("item: {}", i);
    }
}
