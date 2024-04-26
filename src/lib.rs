//! # Space Traveller
//!
//! `space_traveller` is a simple command-line game engine based on the MVC architecture, centered around cosmic space.
//! It servers as a starting point for crafting epic, old-school RPGs enriching the
//! knowledgle of the universe.

pub mod constants;
pub mod interfaces;
pub mod view;

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
    /// Adds players to the game. It consumes the previous state making
    /// another, representing the next phase of the game.
    pub fn add_players(mut self) -> GameManager {
        for i in 1..constants::MAX_NUMBER_OF_PLAYERS + 1 {
            self.view.display_user_addition_prompt(i);
            let nick =
                tools::take_user_name(&mut self.reader).expect("Any string sequence should work");
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
    /// The main game object creator. Custom reader and viewer injection is
    /// performed using public trait objects, allowing for full customization,
    /// even from external crates.
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
    /// Starts the turn-based game. It iterates over the players
    /// in a cycle until a winning condition is met.
    /// Consumes the current game state and return the next-phase object.
    pub fn start_game(mut self) -> PostGameManager {
        for player in self.players.cycle_iter() {
            controller::run(player, &mut self.reader, &self.view);
            if player.is_winner() {
                return PostGameManager {
                    // must return since it is an infinite loop
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
    /// Allows fetching the winner from the last game-phase object.
    fn get_winner(&self) -> &Player {
        self.players
            .iter()
            .find(|x| x.is_winner())
            .expect("At this stage we have the winner for sure!")
    }
    pub fn finish_game(&self) -> &Player {
        let winner = self.get_winner();
        self.view.display_winner(&winner.name());
        winner
    }
}
