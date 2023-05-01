use std::ops::Mul;


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
T: std::ops::Mul + Mul<Output = T> + Clone {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Self) -> Self::Output {
    let mut result = Vec::with_capacity(self.0.len());
    for i in 0..self.0.len() {
        let mut row = Vec::with_capacity(self.0[0].len());
        for j in 0..self.0[0].len() {
            row.push(self.0[i][j].clone() * other.0[i][j].clone());
        }
        result.push(row);
    }
    Some(Matrix(result))
    }
}
    
// let mut result = Matrix::<T>::zero(self_row_len, self_col_len);
// for row in 0..self_row_len{
//     for col in 0..self_col_len{
//         result.0[row][col] = self.0[row][col].clone() + other.0[row][col].clone(); 
//     }
// }
// return Some(result);