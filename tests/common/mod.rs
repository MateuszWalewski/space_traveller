use space_traveller::interfaces::InputReader;
use space_traveller::interfaces::View;
use space_traveller::take_user_option;
use std::io;

pub struct CustomInputReader<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> CustomInputReader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0 as usize,
        }
    }
}

impl<'a> InputReader for CustomInputReader<'a> {
    fn read_line(&mut self, buffer: &mut String) -> io::Result<usize> {
        if self.pos >= self.input.len() {
            return Ok(0);
        }

        let remaining_input = &self.input[self.pos..];
        let end_of_line = remaining_input.find('\n').unwrap_or(remaining_input.len());
        buffer.push_str(&remaining_input[..end_of_line]);
        self.pos += end_of_line + 1;
        Ok(end_of_line)
    }
}

pub struct TestView;
impl View for TestView {
    fn display_delayed_dialog(&self, _message: &str, _millis: u64) {}
    fn display_user_addition_prompt(&self, _player_number: usize) {}
    fn display_user_added_dialog(&self, _name: &str) {}
    fn display_user_stats_bar(&self, _name: &str, _score: &str) {}
    fn display_first_option_menu(&self, _rockets: &str) {}
    fn display_second_option_menu(&self) {}
    fn display_secret_gameplay_option_dialog(&self) {}
    fn display_user_screen(&self, _name: &str, _rockets: usize, _score: i32) {}
    fn display_first_gameplay_option_dialog(&self) {}
    fn display_second_gemaplay_option_dialog(&self, _event: &str, _points: i32) {}
    fn display_winner(&self, _name: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn predefined_string_user_input_reader_works() {
        let pred_input = "1\njohn99\n2\n";
        let mut pred_input_reader = CustomInputReader::new(pred_input);
        let mut line1 = String::new();
        let mut line2 = String::new();
        let mut line3 = String::new();
        let mut line4 = String::new();
        _ = pred_input_reader.read_line(&mut line1);
        _ = pred_input_reader.read_line(&mut line2);
        _ = pred_input_reader.read_line(&mut line3);
        _ = pred_input_reader.read_line(&mut line4);
        assert_eq!(line1, "1");
        assert_eq!(line2, "john99");
        assert_eq!(line3, "2");
        assert_eq!(line4, "");
    }

    #[test]
    fn user_input_checker_function_works() {
        let pred_input = "a\nljzx\n*&\n1";
        let mut pred_reader: Box<dyn InputReader> = Box::new(CustomInputReader::new(pred_input));
        let result = take_user_option(&mut pred_reader);
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid digit found in string"
        );
        let result = take_user_option(&mut pred_reader);
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid digit found in string"
        );
        let result = take_user_option(&mut pred_reader);
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid digit found in string"
        );

        let result = take_user_option(&mut pred_reader);
        assert!(result.is_ok());
    }
}
