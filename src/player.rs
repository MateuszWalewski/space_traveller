use crate::constants;
use std::io;

pub struct Player {
    name: String,
    score: u32,
    engine_boost_launches: usize,
}

impl Player {
    pub fn new(name: String, score: u32) -> Player {
        Player {
            name,
            score,
            engine_boost_launches: constants::ROCKET_ENGINE_LAUNCHES,
        }
    }

    pub fn take_turn(&mut self) {
        //TODO1: Extract it to the separate module with Game Engine
        //TODO2: Change the way of displaying game so that it overwrite the previous output!
        /
        println!("---------------------------------");
        println!("Now it's your turn {}!", self.name);
        println!("Take one of the following actions:");
        println!("---------------------------------");
        println!(
            "1. Launch boost rockets! (+5 pts, {} units left)",
            self.engine_boost_launches
        );
        println!("2. Draw an event");

        loop {
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to parse a choice");

            let choice: usize = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Wrong option selected. Try one more time!");
                    continue;
                }
            };
            match choice {
                1 => {
                    if self.engine_boost_launches > 0 {
                        self.score += constants::ROCKET_ENGINE_BOOST_PREMIUM;
                        self.engine_boost_launches -= 1;
                        break;
                    } else {
                        println!("You've used all launches. Draw an event!");
                    }
                }
                2 => {
                    println!("The event has been drawn!");
                    break;
                }
                _ => println!("Option unavailable. Try again!"),
            }
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}
