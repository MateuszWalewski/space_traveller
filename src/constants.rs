pub const MAX_NUMBER_OF_PLAYERS: usize = 2;
pub const WINNING_VALUE: i32 = 10;
pub const ROCKET_ENGINE_BOOST_PREMIUM: i32 = 5;
pub const ROCKET_ENGINE_LAUNCHES: usize = 5;

pub static EVENT1: &str = "You found a gravity well! You stop and take a rest";
pub static EVENT2: &str = "Your ship has been hit by meteorite! You have to make a stop for repair";
pub static EVENT3: &str = "Solar storm began! You had to stop and rest";
pub static EVENT4: &str = "You regenerate energy from your solar panels! Can go faster";

pub static EVENTS: [(&str, i32); 4] = [(EVENT1, 10), (EVENT2, -5), (EVENT3, -7), (EVENT4, 7)];
