use crate::constants;
use crate::tools;

mod controller;

pub struct Player {
    name: String,
    score: i32,
    engine_boost_launches: usize,
    winner: bool,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            score: 0,
            engine_boost_launches: constants::ROCKET_ENGINE_LAUNCHES,
            winner: false,
        }
    }

    pub fn take_turn<R: tools::InputReader>(&mut self, reader: &mut R) {
        controller::run(self, reader);
    }

    fn update_score(&mut self, points: i32) {
        self.score += points;
    }

    fn use_rocket_launch(&mut self) {
        self.engine_boost_launches -= 1;
    }

    pub fn mark_as_winner(&mut self) {
        self.winner = true
    }

    pub fn is_winner(&self) -> bool {
        self.winner
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}
