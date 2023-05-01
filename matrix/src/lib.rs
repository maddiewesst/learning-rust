
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T> + Clone> Matrix<T> {
	pub fn new() -> Matrix<T> {
        return Matrix(vec![vec![T::one()]])
	}


	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut mat = Matrix::zero(n, n);
        for i in 0..n {
            mat.0[i][i] = T::one();
        }
        mat
	}
}



use std::ops::{Add, Sub, Div, Mul};

pub trait Scalar: Add + Sub + Div + Mul + Sized{
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