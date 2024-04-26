use crate::constants;

/// The model part.
pub struct Player {
    name: String,
    score: i32,
    boosts: usize,
    winner: bool,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            score: 0,
            boosts: constants::ENGINE_BOOSTS,
            winner: false,
        }
    }

    pub fn update_score(&mut self, points: i32) {
        self.score += points;
    }

    pub fn use_rocket_launch(&mut self) {
        self.boosts -= 1;
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
    pub fn boosts(&self) -> usize {
        self.boosts
    }
}
