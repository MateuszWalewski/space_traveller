pub mod interfaces;
pub mod view;

mod constants;
mod controller;
mod player;
mod tools;

pub use crate::tools::take_user_option;
pub use crate::tools::StdInputReader;

use crate::player::Player;
use crate::tools::CycleIter;
use interfaces::InputReader;
use interfaces::View;

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
            self.view.display_user_addition_prompt(i);
            let mut nick = String::new();
            self.reader
                .read_line(&mut nick)
                .expect("Failed to parse a choice");
            nick = nick.trim().to_string();
            self.view.display_user_added_dialog(&nick);
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
    pub fn new<R, V>(reader: R, view: V) -> PreGameManager
    where
        R: interfaces::InputReader + 'static,
        V: interfaces::View + 'static,
    {
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
        self.view.display_winner(&winner.name());
        winner
    }
}
