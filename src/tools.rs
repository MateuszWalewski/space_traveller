use crate::constants;
use rand::Rng;
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
