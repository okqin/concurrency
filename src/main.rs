use concurrency::Matrix;

fn main() {
    let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
    let b = Matrix::new(vec![5, 6, 7, 8], 2, 2);
    println!("a * b = {:?}", a * b);
}
