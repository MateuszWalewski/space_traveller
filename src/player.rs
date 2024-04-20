use crate::constants;

mod engine;

pub struct Player {
    name: String,
    score: u32,
    engine_boost_launches: usize,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            score: 0,
            engine_boost_launches: constants::ROCKET_ENGINE_LAUNCHES,
        }
    }

    pub fn take_turn(&mut self) {
        println!("---------------------------------");
        println!("Now it's your turn {}!", self.name);
        println!("Take one of the following actions:");
        println!("---------------------------------");
        println!(
            "1. Launch boost rockets! (+5 pts, {} units left)",
            self.engine_boost_launches
        );
        println!("2. Draw an event");
        engine::run(self);
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}
