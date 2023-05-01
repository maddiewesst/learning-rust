use lalgebra_scalar::Scalar;
// use matrix::Matrix;
use std::ops::{ Add, Sub };

impl<T> Add for Matrix<T> 
where 
T: Scalar<Item=T> + Clone + Add<Output = T> {

    type Output = Option<Matrix<T>>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let mut result = Matrix::<T>::zero(self.0[0].len(), self.0.len());
        for row in 0..self.0.len() {
            for col in 0..self.0.len(){
                result.0[row][col] = self.0[row][col].clone() + other.0[row][col].clone(); 
            }
        }
        return Some(result);
    
    }
}

impl<T> Sub for Matrix<T>
where
T: Scalar<Item=T> + Sub<Output = T> + Clone {
    type Output = Option<Matrix<T>>;


    fn sub(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let mut result = Matrix::<T>::zero(self.0[0].len(), self.0.len());
        for row in 0..self.0.len() {
            for col in 0..self.0.len(){
                result.0[row][col] = self.0[row][col].clone() - other.0[row][col].clone(); 
            }
        }
        return Some(result);
        }


}






#[derive(Debug, PartialEq, Eq)]
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



// let self_row_len = self.0.len();
// let self_col_len = self.0[0].len();
// let other_row_len = other.0.len();

// let mut result = Matrix::<T>::zero(self_row_len, self_col_len);
// for row in 0..self_row_len{
//     for col in 0..self_col_len{
//         result.0[row][col] = self.0[row][col].clone() + other.0[row][col].clone(); 
//     }
// }
// return Some(result);



// let mut result = Vec::with_capacity(self.0.len());
// for i in 0..self.0.len() {
//     let mut row = Vec::with_capacity(self.0[0].len());
//     for j in 0..self.0[0].len() {
//         row.push(self.0[i][j].clone() - other.0[i][j].clone());
//     }
//     result.push(row);
// }
// Some(Matrix(result))