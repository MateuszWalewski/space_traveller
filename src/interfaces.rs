use std::io;

pub trait InputReader {
    fn read_line(&mut self, buffer: &mut String) -> io::Result<usize>;
}

pub trait View {
    fn display_delayed_dialog(&self, message: &str, millis: u64);
    fn display_user_addition_prompt(&self, player_number: usize);
    fn display_user_added_dialog(&self, name: &str);
    fn display_user_stats_bar(&self, name: &str, score: &str);
    fn display_first_option_menu(&self, rockets: &str);
    fn display_second_option_menu(&self);
    fn display_secret_gameplay_option_dialog(&self);
    fn display_user_screen(&self, name: &str, rockets: usize, score: i32);
    fn display_first_gameplay_option_dialog(&self);
    fn display_second_gemaplay_option_dialog(&self, event: &str, points: i32);
    fn display_winner(&self, name: &str);
}
