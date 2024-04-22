use crate::{constants, tools};
use rand::Rng;
use std::io; //check realtive and absolute option

pub fn draw_event() -> (&'static str, i32) {
    let number = rand::thread_rng().gen_range(0..=3);
    return constants::EVENTS[number as usize];
}

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


pub trait InputReader {
    fn read_line(&mut self, buffer: &mut String) -> io::Result<usize>;
}

pub struct StdInputReader;
impl InputReader for StdInputReader {
    fn read_line(&mut self, buffer: &mut String) -> io::Result<usize> {
        io::stdin().read_line(buffer)
    }
}

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
}
