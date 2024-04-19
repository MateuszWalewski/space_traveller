pub struct CirculatingIterator<'a, T> {
    container: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for CirculatingIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.container.len() != 0 {
            Some(&self.container[self.index])
        } else {
            None
        };
        self.index = (self.index + 1) % self.container.len();
        result
    }
}

pub trait CycleIter<T> {
    fn cycle_iter(&self) -> CirculatingIterator<T>;
}

impl<T> CycleIter<T> for Vec<T> {
    fn cycle_iter(&self) -> CirculatingIterator<T> {
        CirculatingIterator {
            container: &self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circular_iteration_works() {
        let container = vec![1, 4, 5, 7];
        let circular_sub_container: Vec<_> = container.cycle_iter().take(6).collect();
        let expected = vec![&1, &4, &5, &7, &1, &4];
        assert_eq!(circular_sub_container, expected);
    }
}
