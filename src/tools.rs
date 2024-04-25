use crate::{
    constants,
    interfaces::{self, InputReader},
};
use rand::Rng;
use std::{error::Error, io};

pub fn draw_event() -> (&'static str, i32) {
    let number = rand::thread_rng().gen_range(0..=constants::EVENTS.len() - 1);
    return constants::EVENTS[number as usize];
}

pub fn take_user_option<R>(reader: &mut Box<R>) -> Result<usize, Box<dyn Error>>
where
    R: interfaces::InputReader + 'static + ?Sized,
{
    let mut choice = String::new();
    reader.read_line(&mut choice)?;
    let choice = choice.trim().parse()?;
    Ok(choice)
}

pub fn take_user_name<R>(reader: &mut Box<R>) -> Result<String, Box<dyn Error>>
where
    R: interfaces::InputReader + 'static + ?Sized,
{
    let mut nick = String::new();
    reader.read_line(&mut nick)?;
    nick = nick.trim().to_string();
    Ok(nick)
}
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

pub struct StdInputReader;
impl InputReader for StdInputReader {
    fn read_line(&mut self, buffer: &mut String) -> io::Result<usize> {
        io::stdin().read_line(buffer)
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
}
