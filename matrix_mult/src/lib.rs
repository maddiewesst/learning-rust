use std::ops::Mul;

// use lalgebra_scalar::Scalar;



use std::ops::{Add, Sub, Div};
pub trait Scalar: Add + Sub + Div + Mul + Sized {
        type Item;
        fn zero() -> Self::Item;
        fn one() -> Self::Item;
    }
    
    impl Scalar for u32 {
        type Item = u32;
        fn zero() -> Self::Item {
            return 0;
        }
        fn one() -> Self::Item {
            return 1;
        }
    }
    impl Scalar for u64 {
        type Item = u64;
        fn zero() -> Self::Item {
            return 0;
        }
        fn one() -> Self::Item {
            return 1;
        }
    }
    impl Scalar for i32 {
        type Item = i32;
        fn zero() -> Self::Item {
            return 0;
        }
        fn one() -> Self::Item {
            return 1;
        }
    }
    impl Scalar for i64 {
        type Item = i64;
        fn zero() -> Self::Item {
            return 0;
        }
        fn one() -> Self::Item {
            return 1;
        }
    }
    impl Scalar for f32 {
        type Item = f32;
        fn zero() -> Self::Item {
            return 0.0;
        }
        fn one() -> Self::Item {
            return 1.0;
        }
    }
    impl Scalar for f64 {
        type Item = f64;
        fn zero() -> Self::Item {
            return 0.0;
        }
        fn one() -> Self::Item {
            return 1.0;
        }
    }


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    
	pub fn number_of_cols(&self) -> usize {
        self.0.len()
	}

	pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
	}

	pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
	}

	pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
	}
}

impl<T> Mul for Matrix<T> 
where
T: Scalar<Item=T> + Clone + Mul<Output = T> + Add<Output = T> 
{
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Self) -> Self::Output {
        if self.0.len() != other.0[0].len() {
            return None;
        }

        let mut result = vec![vec![T::zero(); other.0.len()]; self.0[0].len()];

        for i in 0..self.0[0].len() {
            for j in 0..other.0.len() {
                let mut sum = T::zero();
                for k in 0..self.0.len() {
                    sum = sum + self.0[i][k].clone() * other.0[k][j].clone();
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}
    

// let mut result = Vec::with_capacity(self.0.len());
// for i in 0..self.0.len() {
//     let mut row = Vec::with_capacity(self.0[0].len());
//     for j in 0..self.0[0].len() {
//         row.push(self.0[i][j].clone() * other.0[i][j].clone());
//     }
//     result.push(row);
// }
// Some(Matrix(result))