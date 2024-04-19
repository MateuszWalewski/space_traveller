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

pub trait CircleIter<T> {
    fn circle_iter(&self) -> CirculatingIterator<T>;
}

impl<T> CircleIter<T> for Vec<T> {
    fn circle_iter(&self) -> CirculatingIterator<T> {
        CirculatingIterator {
            container: &self,
            index: 0,
        }
    }
}
