use crate::{constants, interfaces::View};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};
use termion::{clear, color, cursor};

pub struct ConsoleView;

impl View for ConsoleView {
    fn display_delayed_dialog(&self, message: &str, millis: u64) {
        println!("{}", message);
        self.delay_window(millis);
    }
    fn display_user_addition_prompt(&self, player_number: usize) {
        println!(
            "Please type a nick for the player number {}:",
            &player_number.to_string()
        );
    }

    fn display_user_added_dialog(&self, name: &str) {
        let msg1 = format!(
            "{}{}{}",
            color::Fg(color::Green),
            name,
            color::Fg(color::Reset)
        );
        println!("The player: {} has been added.", msg1);
        self.delay_window(800);
    }

    fn display_user_stats_bar(&self, name: &str, score: &str) {
        let msg1 = format!(
            "{}{}{}",
            color::Fg(color::Red),
            name,
            color::Fg(color::Reset)
        );
        let msg2 = format!(
            "{}{}{}",
            color::Fg(color::Blue),
            score,
            color::Fg(color::Reset)
        );

        let msg3 = format!(
            "{}{}{}",
            color::Fg(color::White),
            constants::WINNING_VALUE,
            color::Fg(color::Reset)
        );

        println!(
            "Now it's your turn {}! | score: {} pts | to win: {} pts ",
            msg1, msg2, msg3
        );
    }
    fn display_winner(&self, name: &str) {
        self.clear_screen();
        self.display_short_line();
        let msg1 = format!(
            "{}{}{}",
            color::Fg(color::White),
            name,
            color::Fg(color::Reset)
        );
        print!("{}", color::Fg(color::Yellow));
        println!("The winner is {}!", msg1);
        print!("{}", color::Fg(color::Reset));
        self.display_short_line();
    }

    fn display_message(&self, message: &str) {
        println!("{}", message);
    }

    fn display_line(&self) {
        self.display_message("-----------------------------------------");
    }

    fn display_short_line(&self) {
        self.display_message("---------------------");
    }

    fn display_first_option_menu(&self, rockets: &str) {
        print!("{}", color::Fg(color::Magenta));
        println!("1. Launch boost rockets! ({} units left)", rockets);
        print!("{}", color::Fg(color::Reset));
    }
    fn display_second_option_menu(&self) {
        print!("{}", color::Fg(color::Magenta));
        println!("2. Explore the universe!");
        print!("{}", color::Fg(color::Reset));
    }

    fn display_user_screen(&self, name: &str, rockets: usize, score: i32) {
        self.clear_screen();
        self.display_user_stats_bar(name, &score.to_string());
        self.display_message("Take one of the following actions:");
        self.display_line();
        self.display_first_option_menu(&rockets.to_string());
        self.display_second_option_menu();
    }

    fn delay_window(&self, millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    fn display_first_gameplay_option_dialog(&self) {
        self.clear_screen();
        print!("{}", color::Fg(color::Red));
        println!(
            "You travel with the speed of light! The space is sqeezeed! (+{}) pts",
            constants::BOOST_PREMIUM
        );
        print!("{}", color::Fg(color::Reset));
        self.delay_window(2000);
    }
    fn display_second_gemaplay_option_dialog(&self, event: &str, points: i32) {
        self.clear_screen();
        print!("{}", color::Fg(color::Red));
        println!("{}: {} pts", event, points);
        print!("{}", color::Fg(color::Reset));
        self.delay_window(2000);
    }

    fn clear_screen(&self) {
        print!("{}", clear::All);
        print!("{}", cursor::Goto(1, 1));
        stdout().flush().expect("Failed to flush stdout");
    }
}
