#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StepIterator<T> {
    pub beg: T,
    pub end: T,
    pub step: T,
}
use std::ops::{Add};
impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
    }
}
impl<T> std::iter::Iterator for StepIterator<T>
where T: Add<Output = T> + PartialEq<T> + Clone + PartialOrd {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let result: T;
        if self.beg > self.end {
            return None;
        }else{
            result = self.beg.clone();
            self.beg = self.beg.clone() + self.step.clone();
        }
        return Some(result);
    }
}