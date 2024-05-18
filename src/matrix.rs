use anyhow::{anyhow, Result};
use std::{
    fmt::{Debug, Display},
    ops::{AddAssign, Mul},
};

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Self {
        assert_eq!(rows * cols, data.len());
        Self { data, rows, cols }
    }
}

pub fn matrix<T>(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Mul<Output = T> + AddAssign,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix dimensions mismatch"));
    }
    let rows = a.rows;
    let cols = b.cols;
    let mut data = vec![T::default(); rows * cols];
    for i in 0..rows {
        for j in 0..cols {
            for k in 0..a.cols {
                data[i * cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }
    Ok(Matrix { data, rows, cols })
}

// Display like this: {{1, 2}, {3, 4}} for a 2x2 matrix
impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            write!(f, "{{")?;
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j < self.cols - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "}}")?;
            if i < self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl<T: Display> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_2x2() {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![5, 6, 7, 8], 2, 2);
        let c = matrix(a, b).unwrap();
        assert_eq!(format!("{:?}", c), "{{19, 22}, {43, 50}}");
    }

    #[test]
    fn test_matrix_2x3() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![10, 11, 20, 21, 30, 31], 3, 2);
        let c = matrix(a, b).unwrap();
        assert_eq!(format!("{:?}", c), "{{140, 146}, {320, 335}}");
    }
}
