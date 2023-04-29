#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

pub trait Scalar: Sized {}
impl Scalar for i64 {}

use std::ops::Add;

impl<T: Add<Output = T> + Scalar + Clone> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut vector = Vec::new();
        for i in 0..self.0.len() {
            vector.push(self.0[i].clone() + other.0[i].clone())
        }
        return Some(Vector(vector));
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(vec![])
    }

    pub fn dot(&self, other: &Self) -> Option<i64>
    where
        i64: Scalar,
    {
        if self.0.len() != other.0.len() {
            return None;
        }
        return Some(self.0.len() as i64);
    }
}