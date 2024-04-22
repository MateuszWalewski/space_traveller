use crate::constants;
use crate::tools;
use crate::Player;

// Check if it is acccessible only from player module

pub fn take_user_option<R: tools::InputReader>(reader: &mut R) -> usize {
    loop {
        let mut choice = String::new();
        reader
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

pub fn run<R: tools::InputReader>(player: &mut Player, reader: &mut R) {
    // ---------- Move it to View part  ------------------------
    println!("---------------------------------");
    println!("Now it's your turn {}!", player.name);
    println!("Take one of the following actions:");
    println!("---------------------------------");
    println!(
        "1. Launch boost rockets! (+5 pts, {} units left)",
        player.engine_boost_launches
    );
    println!("2. Draw an event");
    // --------------------------------------------
    let mut choice = take_user_option(reader);
    loop {
        match choice {
            1 => {
                //TODO: Wrap this part in the separte function returing Result.
                // Afterwards, return Result from current func as well for higher level
                if player.engine_boost_launches > 0 {
                    player.update_score(constants::ROCKET_ENGINE_BOOST_PREMIUM);
                    player.use_rocket_launch();
                    break;
                } else {
                    println!("You've used all launches. Draw an event!");
                    choice = take_user_option(reader);
                }
            }
            2 => {
                let event = tools::draw_event();
                println!("{}: {} pts", event.0, event.1);
                player.update_score(event.1);
                break;
            }
            _ => {
                println!("Option unavailable. Try again!");
                choice = take_user_option(reader);
            }
        }
    }
}
