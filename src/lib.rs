mod constants;
mod controller;
mod player;
pub mod tools;
pub mod view;
use crate::player::Player;
use crate::tools::CycleIter;
use crate::tools::InputReader;
use crate::view::View;

pub struct PreGameManager {
    players: Vec<Player>,
    reader: Box<dyn InputReader>,
    view: Box<dyn View>,
}

pub struct GameManager {
    players: Vec<Player>,
    reader: Box<dyn InputReader>,
    view: Box<dyn View>,
}

pub struct PostGameManager {
    players: Vec<Player>,
    view: Box<dyn View>,
}

impl PreGameManager {
    pub fn add_players(mut self) -> GameManager {
        for i in 1..constants::MAX_NUMBER_OF_PLAYERS + 1 {
            self.view.clear_screen();
            println!("Please type a nick for the player number {}:", i);
            let mut nick = String::new();
            self.reader
                .read_line(&mut nick)
                .expect("Failed to parse a choice");

            self.view
                .display_delayed_message(&format!("The player: {} has been added.", nick));
            self.players.push(Player::new(nick));
        }
        GameManager {
            players: self.players,
            reader: self.reader,
            view: self.view,
        }
    }
}

impl GameManager {
    pub fn new<R: tools::InputReader + 'static, V: view::View + 'static>(
        reader: R,
        view: V,
    ) -> PreGameManager {
        PreGameManager {
            players: Vec::new(),
            reader: Box::new(reader),
            view: Box::new(view),
        }
    }

    pub fn start_game(mut self) -> PostGameManager {
        for player in self.players.cycle_iter() {
            controller::run(player, &mut self.reader, &self.view);
            if player.is_winner() {
                return PostGameManager {
                    // we must return since it is infinite loop
                    players: self.players,
                    view: self.view,
                };
            }
        }
        PostGameManager {
            players: self.players,
            view: self.view,
        }
    }

    pub fn number_of_players(&self) -> usize {
        self.players.len()
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
}

impl PostGameManager {
    fn get_winner(&self) -> &Player {
        self.players
            .iter()
            .find(|x| x.is_winner())
            .expect("On this stage we have the winner for sure!")
    }

    pub fn finish_game(&self) -> &Player {
        let winner = self.get_winner();
        self.view.clear_screen();
        self.view.display_line();
        self.view
            .display_message(&format!("The winner is {}", winner.name()));
        self.view.display_line();
        winner
    }
}
