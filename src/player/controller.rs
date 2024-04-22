use crate::constants;
use crate::tools;
use crate::Player;
use std::error::Error;

pub fn run<R: tools::InputReader>(model: &mut Player, reader: &mut R) {
    // ---------- Move it to View part  ------------------------
    println!("---------------------------------");
    println!("Now it's your turn {}!", model.name);
    println!("Take one of the following actions:");
    println!("---------------------------------");
    println!(
        "1. Launch boost rockets! (+5 pts, {} units left)",
        model.engine_boost_launches // use function instead of access fields directly!
    );
    println!("2. Explore the universe!");
    // --------------------------------------------
    let mut choice = tools::take_user_option(reader);
    loop {
        match choice {
            1 => {
                match launch_boost_rockets(model) {
                    Ok(()) => {
                        break;
                    }
                    Err(e) => {
                        println!("{}", e);
                        choice = tools::take_user_option(reader);
                    }
                };
            }
            2 => {
                explore_the_universe(model);
                break;
            }
            _ => {
                println!("Option unavailable. Try again!");
                choice = tools::take_user_option(reader);
            }
        }
    }
}

fn launch_boost_rockets(model: &mut Player) -> Result<(), Box<dyn Error>> {
    if model.engine_boost_launches > 0 {
        model.update_score(constants::ROCKET_ENGINE_BOOST_PREMIUM);
        model.use_rocket_launch();
        Ok(())
    } else {
        Err("You've used all launches... Try other option!".into())
    }
}

fn explore_the_universe(model: &mut Player) {
    let event = tools::draw_event();
    println!("{}: {} pts", event.0, event.1);
    model.update_score(event.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rocket_launches_works() {
        let name = String::from("rick56");
        let mut player = Player::new(name);
        _ = launch_boost_rockets(&mut player);
        assert_eq!(player.score(), constants::ROCKET_ENGINE_BOOST_PREMIUM);
    }
    #[test]
    fn using_all_rocket_launches_works() {
        let name = String::from("rick56");
        let mut player = Player::new(name);
        _ = launch_boost_rockets(&mut player);
        _ = launch_boost_rockets(&mut player);
        _ = launch_boost_rockets(&mut player);
        _ = launch_boost_rockets(&mut player);
        _ = launch_boost_rockets(&mut player);
        let result = launch_boost_rockets(&mut player);
        assert_eq!(
            result.unwrap_err().to_string(),
            "You've used all launches... Try other option!"
        );
    }

    #[test]
    fn exploring_the_space_works() {
        let name = String::from("rick56");
        let mut player = Player::new(name);
        let initial_score = player.score();
        explore_the_universe(&mut player);
        assert_ne!(player.score(), initial_score);
    }
}
