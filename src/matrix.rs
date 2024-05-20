use crate::vector::{vector_point, Vector};
use anyhow::{anyhow, Result};
use oneshot::Sender;
use std::{
    fmt::{Debug, Display},
    ops::{AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUMBER_THREADS: usize = 4;

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

struct Msginput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

struct Msgoutput<T> {
    idx: usize,
    value: T,
}

struct Msg<T> {
    input: Msginput<T>,
    output: Sender<Msgoutput<T>>,
}

impl<T> Msginput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msgoutput<T> {
    fn new(idx: usize, value: T) -> Self {
        Self { idx, value }
    }
}

impl<T> Msg<T> {
    fn new(input: Msginput<T>, output: Sender<Msgoutput<T>>) -> Self {
        Self { input, output }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Mul<Output = T> + AddAssign + Send + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix dimensions mismatch"));
    }

    let sender = (0..NUMBER_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = vector_point(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.output.send(Msgoutput::new(msg.input.idx, value)) {
                        eprintln!("Failed to send output message: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let rows = a.rows;
    let cols = b.cols;
    let matrix_size = rows * cols;
    let mut data = vec![T::default(); matrix_size];
    let mut receivers = Vec::with_capacity(matrix_size);
    for i in 0..rows {
        for j in 0..cols {
            let vec_a = Vector::new(a.data[i * a.cols..(i + 1) * a.cols].to_vec());
            let vec_b = Vector::new(
                b.data[j..]
                    .iter()
                    .step_by(b.cols)
                    .copied()
                    .collect::<Vec<_>>(),
            );
            let (tx, rx) = oneshot::channel();
            let idx = i * cols + j;
            let msg = Msg::new(Msginput::new(idx, vec_a, vec_b), tx);
            if let Err(e) = sender[idx % NUMBER_THREADS].send(msg) {
                eprintln!("Failed to send input message: {:?}", e);
            }
            receivers.push(rx);
        }
    }
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
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

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Mul<Output = T> + AddAssign + Send + 'static,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Failed to multiply matrices")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_2x2() {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![5, 6, 7, 8], 2, 2);
        let c = a * b;
        assert_eq!(format!("{:?}", c), "{{19, 22}, {43, 50}}");
    }

    #[test]
    fn test_matrix_2x3() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![10, 11, 20, 21, 30, 31], 3, 2);
        let c = a * b;
        assert_eq!(format!("{:?}", c), "{{140, 146}, {320, 335}}");
    }

    #[test]
    #[should_panic]
    fn test_matrix_not_square() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![10, 11, 20, 21], 2, 2);
        let _c = a * b;
    }
}
