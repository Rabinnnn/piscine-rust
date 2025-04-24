use lalgebra_scalar::Scalar;
mod mult;
mod ops;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![Vec::new()])
    }
    // It returns the zero matrix of the size given by the row and
    // column parameters
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut matrix = Matrix(Vec::new());
        for _ in 0..row {
            matrix.0.push(vec![T::zero(); col]);
        }
        matrix
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Matrix::new();
        for y in 0..n {
            if y > 0 {
                matrix.0.push(Vec::new());
            }
            for x in 0..n {
                if y == x {
                    matrix.0[y].push(T::one());
                } else {
                    matrix.0[y].push(T::zero());
                }
            }
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_property() {
        let matrix: Matrix<u32> = Matrix::zero(3, 4);
        let expected: Matrix<u32> =
            Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
        assert_eq!(matrix, expected);

        let matrix: Matrix<u32> = Matrix::zero(2, 2);
        let expected: Matrix<u32> = Matrix(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn identy_matrix() {
        let matrix: Matrix<u32> = Matrix::identity(2);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(matrix, expected);

        let matrix: Matrix<u32> = Matrix::identity(3);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
        assert_eq!(matrix, expected);
    }
}