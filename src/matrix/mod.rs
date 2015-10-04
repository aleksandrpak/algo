pub struct Mat<T> {
    n: usize,
    m: usize,
    data: Vec<T>
}

impl <T: Default> Mat<T> {
    pub fn new(n: usize, m: usize) -> Mat<T> {
        Mat {
            n: n,
            m: m,
            data: Vec::with_capacity(n * m),
        }
    }
}
