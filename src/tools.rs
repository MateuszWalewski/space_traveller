use crate::constants;
use rand::Rng;
use std::error::Error;
use std::io; //check realtive and absolute option

pub fn draw_event() -> (&'static str, i32) {
    let number = rand::thread_rng().gen_range(0..=3);
    return constants::EVENTS[number as usize];
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

pub fn take_user_option(reader: &mut Box<dyn InputReader>) -> Result<usize, Box<dyn Error>> {
    let mut choice = String::new();
    reader.read_line(&mut choice)?;
    let choice = choice.trim().parse()?;
    Ok(choice)
}

// TODO: Add the documentation!
pub struct CirculatingIterator<'a, T> {
    container: &'a mut Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for CirculatingIterator<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = if !self.container.is_empty() {
            unsafe { Some(&mut *(&mut self.container[self.index] as *mut T)) }
        } else {
            None
        };
        self.index = (self.index + 1) % self.container.len();
        result
    }
}

pub trait CycleIter<T> {
    fn cycle_iter(&mut self) -> CirculatingIterator<T>;
}

impl<T> CycleIter<T> for Vec<T> {
    fn cycle_iter(&mut self) -> CirculatingIterator<T> {
        CirculatingIterator {
            container: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circular_iteration_works() {
        let mut container = vec![1, 4, 5, 7];
        let circular_sub_container: Vec<_> = container.cycle_iter().take(6).collect();
        let expected = vec![&1, &4, &5, &7, &1, &4];
        assert_eq!(circular_sub_container, expected);
    }

    #[test]
    fn circular_mut_iteration_works() {
        let mut container = vec![1, 4, 5, 7];
        for i in container.cycle_iter().take(4) {
            *i += 1;
        }
        let expected = vec![2, 5, 6, 8];
        assert_eq!(container, expected);
    }

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
