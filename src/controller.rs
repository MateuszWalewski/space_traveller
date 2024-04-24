use crate::{constants, interfaces, tools, Player};
use std::error::Error;

pub fn run<R, V>(model: &mut Player, reader: &mut Box<R>, view: &Box<V>)
where
    R: interfaces::InputReader + 'static + ?Sized,
    V: interfaces::View + 'static + ?Sized,
{
    loop {
        view.display_user_screen(&model.name(), model.boosts(), model.score());
        let choice = match tools::take_user_option(reader) {
            Ok(num) => num,
            Err(_) => {
                view.display_delayed_dialog("Wrong input... Enter a number please!", 1000);
                continue;
            }
        };

        match choice {
            1 => {
                match launch_boost_rockets(model) {
                    Ok(()) => {
                        view.display_first_gameplay_option_dialog();
                        break;
                    }
                    Err(e) => {
                        view.display_delayed_dialog(&e.to_string(), 1000);
                    }
                };
            }
            2 => {
                let event = explore_the_universe(model);
                view.display_second_gemaplay_option_dialog(event.0, event.1);
                break;
            }
            _ => {
                view.display_delayed_dialog("Option unavailable. Try again!", 1000);
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
        for _ in 0..constants::ENGINE_BOOSTS {
            _ = launch_boost_rockets(&mut player);
        }
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
