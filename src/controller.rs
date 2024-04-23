use crate::constants;
use crate::tools;
use crate::InputReader;
use crate::Player;
use crate::View;
use std::error::Error;

//TODO: Use the "where" clause to make it clearer!
pub fn run(model: &mut Player, reader: &mut Box<dyn InputReader>, view: &Box<dyn View>) {
    loop {
        view.clear_screen();
        view.display_user_screen(
            &model.name(),
            &model.boosts().to_string(),
            &model.score().to_string(),
        );

        let choice = match tools::take_user_option(reader) {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input... Enter a number please!");
                view.delay_window(1500);
                continue;
            }
        };

        match choice {
            1 => {
                match launch_boost_rockets(model) {
                    Ok(()) => {
                        view.clear_screen();
                        view.display_fancy_message(&format!(
                            "You travel with the speed of light! The space is sqeezeed! (+{}) pts",
                            constants::BOOST_PREMIUM
                        ));
                        break;
                    }
                    Err(e) => {
                        println!("{}", e);
                        view.delay_window(2000);
                    }
                };
            }
            2 => {
                view.clear_screen();
                let event = explore_the_universe(model);
                view.display_fancy_message(&format!("{}: {} pts", event.0, event.1));
                break;
            }
            _ => {
                println!("Option unavailable. Try again!");
                view.delay_window(1500);
            }
        }
    }

    if is_winning_condition_meet(model) {
        model.mark_as_winner();
    }
}

fn launch_boost_rockets(model: &mut Player) -> Result<(), Box<dyn Error>> {
    if model.boosts() > 0 {
        model.update_score(constants::BOOST_PREMIUM);
        model.use_rocket_launch();
        Ok(())
    } else {
        Err("You've used all launches... Try other option!".into())
    }
}

fn explore_the_universe(model: &mut Player) -> (&'static str, i32) {
    let event = tools::draw_event();
    model.update_score(event.1);
    event
}

fn is_winning_condition_meet(model: &Player) -> bool {
    model.score() >= constants::WINNING_VALUE
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rocket_launches_works() {
        let name = String::from("rick56");
        let mut player = Player::new(name);
        _ = launch_boost_rockets(&mut player);
        assert_eq!(player.score(), constants::BOOST_PREMIUM);
    }

    #[test]
    fn player_wins_the_game_after_achieving_win_condition_works() {
        let name = String::from("rick56");
        let mut player = Player::new(name);
        _ = launch_boost_rockets(&mut player);
        _ = launch_boost_rockets(&mut player);
        assert!(is_winning_condition_meet(&player));
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
