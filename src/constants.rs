pub const MAX_NUMBER_OF_PLAYERS: usize = 2;

pub const WINNING_VALUE: i32 = 20;
pub const BOOST_PREMIUM: i32 = 5;
pub const ENGINE_BOOSTS: usize = 3;
pub const TELEPORT_PREMIUM: i32 = WINNING_VALUE;

pub static EVENT1: &str = "You encountered a gravity wormhole! It moves you forward dramatically!!";
pub static EVENT2: &str =
    "Your ship has been hit by a meteorite! You need to make a stop for repairs";
pub static EVENT3: &str = "A solar storm has begun! You had to stop and rest";
pub static EVENT4: &str = "You regenerate energy from your solar panels! Can now go faster";

pub static EVENTS: [(&str, i32); 4] = [(EVENT1, 8), (EVENT2, -5), (EVENT3, -7), (EVENT4, 7)];
