use super::Range;
use std::ops::Add;

impl<T> Range<T>
where
    T: Copy + PartialOrd + From<u8> + Add<Output = T>,
{
    pub fn iter(&self) -> RangeIter<T> {
        RangeIter {
            current: self.start,
            end: self.end,
            done: false,
        }
    }
}

pub struct RangeIter<T> {
    current: T,
    end: T,
    done: bool,
}

impl<T> Iterator for RangeIter<T>
where
    T: Copy + PartialOrd + From<u8> + Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current;

        if result >= self.end {
            self.done = true;
        } else {
            self.current = self.current + T::from(1u8);
        }

        Some(result)
    }
}
