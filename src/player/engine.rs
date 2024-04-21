use crate::constants;
use crate::Player;
use std::io;
use crate::tools;

// Check if it is acccessible only from player module


pub fn take_user_option() -> usize {
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to parse a choice");

        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input... Enter a number please");
                continue;
            }
        };

        return choice;
    }
}

pub fn run(player: &mut Player) {
    let mut choice = take_user_option();
    loop {
        match choice {
            1 => {
                if player.engine_boost_launches > 0 {
                    player.score += constants::ROCKET_ENGINE_BOOST_PREMIUM;
                    player.engine_boost_launches -= 1;
                    break;
                } else {
                    println!("You've used all launches. Draw an event!");
                    choice = take_user_option();
                }
            }
            2 => {
                let event = tools::draw_event();
                println!("{}: {} pts", event.0, event.1);
                break;
            }
            _ => {
                println!("Option unavailable. Try again!");
                choice = take_user_option();
            }
        }
    }
}
