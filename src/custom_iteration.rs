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
}
