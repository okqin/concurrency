use anyhow::{anyhow, Result};
use std::ops::{AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn vector_point<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Mul<Output = T> + AddAssign + Send + 'static,
{
    if a.len() != b.len() {
        return Err(anyhow!("Vector dimensions mismatch"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}
