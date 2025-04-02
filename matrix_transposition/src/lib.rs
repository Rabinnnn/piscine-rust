#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));  // ✅ Public fields

pub fn transpose(m: Matrix) -> Matrix {
    Matrix((m.0 .0, m.1 .0), (m.0 .1, m.1 .1))
}