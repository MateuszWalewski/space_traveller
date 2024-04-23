//#[cfg(not(test))]
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::ResetColor;
use crossterm::style::SetForegroundColor;
use crossterm::terminal::{Clear, ClearType};
//#[cfg(not(test))]
use crossterm::cursor::MoveTo;
use crossterm::execute;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::{thread, time::Duration};
//-------------------- View ( from MVC. Move it to proper place in the project! )
pub trait View {
    fn display_message(&self, message: &str);
    fn display_fancy_message(&self, message: &str);
    fn display_delayed_message(&self, message: &str);
    fn display_user_screen(&self, arg1: &str, arg2: &str, arg3: &str);
    fn display_line(&self);
    fn clear_screen(&self);
    fn display_user_stats_bar(&self, message: &str, arg1: &str, arg2: &str);
    fn delay_window(&self, millis: u64); //Add only for nontest annotation!
}

pub struct ConsoleView;

impl View for ConsoleView {
    fn display_delayed_message(&self, message: &str) {
        execute!(stdout(), Print(message)).unwrap();
        //     print!("{}", message);
        thread::sleep(Duration::from_millis(1000)); //Add only for nontest annotation!
    }
    fn display_message(&self, message: &str) {
        execute!(
            stdout(),
            SetForegroundColor(Color::Magenta),
            Print(message),
            ResetColor
        )
        .unwrap();
        io::stdout().flush().unwrap();
    }

    fn display_fancy_message(&self, message: &str) {
        execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print(message),
            ResetColor,
            Print("\n")
        )
        .unwrap();
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(3000)); //Add only for nontest annotation!
    }

    fn display_user_stats_bar(&self, message: &str, arg1: &str, arg2: &str) {
        execute!(
            stdout(),
            SetForegroundColor(Color::Yellow),
            Print(message),
            ResetColor,
            SetForegroundColor(Color::White),
            Print(arg1),
            ResetColor,
            SetForegroundColor(Color::Red),
            Print(" | score: ("),
            Print(arg2),
            Print(")"),
            Print(" pts."),
            ResetColor,
            Print("\n"),
        )
        .unwrap();
        io::stdout().flush().unwrap();
    }

    fn display_line(&self) {
        self.display_message("-----------------------------------------------");
        self.display_message("\n");
        // Change it to constant!
    }

    fn display_user_screen(&self, name: &str, launches: &str, score: &str) {
        // change into options in
        //self.display_message(&format!("Now it's your turn {}!", name));
        self.display_user_stats_bar("Now it's your turn ", name, score);
        self.display_message("Take one of the following actions:");
        self.display_message("\n");
        self.display_line();
        self.display_message(&format!(
            "1. Launch boost rockets! ({} units left)",
            launches
        ));
        self.display_message("\n");
        //self.display_one_argument_message("1. Launch boost rockets! (+5 pts, {} units left)", launches);
        self.display_message("2. Explore the universe!");
        self.display_message("\n");
    }
    fn delay_window(&self, millis: u64) {
        thread::sleep(Duration::from_millis(millis)); //Add only for nontest annotation!
    }

    // #[cfg(not(test))]
    fn clear_screen(&self) {
        // Clear the console screen
        execute!(stdout(), Clear(ClearType::All)).unwrap();
        execute!(stdout(), MoveTo(0, 0)).unwrap();
    }

    //#[cfg(test)]
    //   fn clear_screen(&self) {
    //   }
}

pub struct TestView;
impl View for TestView {
    fn display_message(&self, _message: &str) {}
    fn display_fancy_message(&self, _message: &str) {}
    fn display_delayed_message(&self, _message: &str) {}
    fn display_user_screen(&self, _arg1: &str, _arg2: &str, _arg3: &str) {}
    fn display_line(&self) {}
    fn clear_screen(&self) {}
    fn display_user_stats_bar(&self, _message: &str, _arg1: &str, _arg2: &str) {}
    fn delay_window(&self, _millis: u64) {} //Add only for nontest annotation!
}
//----------------------------------------------------------------------------------
