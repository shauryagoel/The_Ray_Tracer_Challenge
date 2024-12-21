use crate::Compare;
use crate::Matrix3;
use crate::Tuple;

const MATRIX_SIZE: usize = 4;

#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix {
    data: [[f64; MATRIX_SIZE]; MATRIX_SIZE], // TODO: create this into 1D array to increase performance
}

impl Matrix {
    // Initialize a new matrix with 0's
    pub fn new() -> Self {
        Default::default()
    }

    // Initialize identity matrix
    #[allow(non_snake_case)] // For usage of I
    pub fn I() -> Self {
        let mut identity_matrix: Matrix = Default::default();
        for i in 0..MATRIX_SIZE {
            identity_matrix[i][i] = 1.0;
        }
        identity_matrix
    }

    // Create a copy of input and transpose it
    pub fn transpose(&self) -> Self {
        let mut result = self.clone(); // TODO: fix clippy lint
        for i in 1..MATRIX_SIZE {
            for j in 0..i {
                (result[i][j], result[j][i]) = (result[j][i], result[i][j]);
            }
        }
        result
    }

    // Generate a 3x3 submatrix from the self matrix by removing all the data in `row_ind` row and `col_ind` column
    pub fn submatrix(&self, row_ind: usize, col_ind: usize) -> Matrix3 {
        let mut result: Matrix3 = Default::default();
        let mut orig_row_ind: usize = 0;
        for i in 0..result.data.len() {
            if orig_row_ind == row_ind {
                orig_row_ind += 1;
            }
            let mut orig_col_ind: usize = 0;
            for j in 0..result.data[0].len() {
                if orig_col_ind == col_ind {
                    orig_col_ind += 1;
                }
                result[i][j] = self[orig_row_ind][orig_col_ind];
                orig_col_ind += 1;
            }
            orig_row_ind += 1;
        }
        result
    }

    // Minor is determinant of a submatrix
    pub fn minor(&self, row_ind: usize, col_ind: usize) -> f64 {
        self.submatrix(row_ind, col_ind).determinant()
    }

    // Cofactor is minor with possibly a sign depending on row and col index
    pub fn cofactor(&self, row_ind: usize, col_ind: usize) -> f64 {
        let sign = (-1_f64).powi((row_ind + col_ind) as i32);
        sign * self.minor(row_ind, col_ind)
    }

    // Find determinant using cofactors
    pub fn determinant(&self) -> f64 {
        let mut result: f64 = 0.0;
        for i in 0..MATRIX_SIZE {
            result += self.cofactor(0, i) * self[0][i];
        }
        result
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    // Calculate inverse of the matrix, panic if no inverse possible
    pub fn inverse(&self) -> Self {
        let determinant = self.determinant();
        if !self.is_invertible() {
            panic!("Matrix is not invertible");
        }
        let mut result: Matrix = Default::default();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                result[j][i] = self.cofactor(i, j) / determinant; // Also doing the transpose
            }
        }
        result
    }
}

// Use this to get a row of matrix when indexing Matrix
impl std::ops::Index<usize> for Matrix {
    type Output = [f64; MATRIX_SIZE];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..MATRIX_SIZE {
            for col in 0..MATRIX_SIZE {
                if self.data[row][col].neq(other[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

// matrix * matrix
impl std::ops::Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result: Matrix = Default::default();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                for k in 0..MATRIX_SIZE {
                    result[i][j] += self[i][k] * other[k][j];
                }
            }
        }
        result
    }
}

// matrix * tuple
impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self[0][0] * other.x
                + self[0][1] * other.y
                + self[0][2] * other.z
                + self[0][3] * other.w,
            self[1][0] * other.x
                + self[1][1] * other.y
                + self[1][2] * other.z
                + self[1][3] * other.w,
            self[2][0] * other.x
                + self[2][1] * other.y
                + self[2][2] * other.z
                + self[2][3] * other.w,
            self[3][0] * other.x
                + self[3][1] * other.y
                + self[3][2] * other.z
                + self[3][3] * other.w,
        )
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn matrix_init4x4() {
        let mut m: Matrix = Default::default();
        m[0][0] = 1.0;
        m[0][1] = 2.0;
        m[0][2] = 3.0;
        m[0][3] = 4.0;

        m[1][0] = 5.5;
        m[1][1] = 6.5;
        m[1][2] = 7.5;
        m[1][3] = 8.5;

        m[2][0] = 9.0;
        m[2][1] = 10.0;
        m[2][2] = 11.0;
        m[2][3] = 12.0;

        m[3][0] = 13.5;
        m[3][1] = 14.5;
        m[3][2] = 15.5;
        m[3][3] = 16.5;

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn matrix_compare1() {
        let mut m1: Matrix = Default::default();
        m1[0][0] = 1.0;
        m1[0][1] = 2.0;
        m1[0][2] = 3.0;
        m1[0][3] = 4.0;

        m1[1][0] = 5.0;
        m1[1][1] = 6.0;
        m1[1][2] = 7.0;
        m1[1][3] = 8.0;

        m1[2][0] = 9.0;
        m1[2][1] = 8.0;
        m1[2][2] = 7.0;
        m1[2][3] = 6.0;

        m1[3][0] = 5.0;
        m1[3][1] = 4.0;
        m1[3][2] = 3.0;
        m1[3][3] = 2.0;

        let mut m2: Matrix = Default::default();
        m2[0][0] = 1.0;
        m2[0][1] = 2.0;
        m2[0][2] = 3.0;
        m2[0][3] = 4.0;

        m2[1][0] = 5.0;
        m2[1][1] = 6.0;
        m2[1][2] = 7.0;
        m2[1][3] = 8.0;

        m2[2][0] = 9.0;
        m2[2][1] = 8.0;
        m2[2][2] = 7.0;
        m2[2][3] = 6.0;

        m2[3][0] = 5.0;
        m2[3][1] = 4.0;
        m2[3][2] = 3.0;
        m2[3][3] = 2.0;

        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_compare2() {
        let mut m1: Matrix = Default::default();
        m1[0][0] = 1.0;
        m1[0][1] = 2.0;
        m1[0][2] = 3.0;
        m1[0][3] = 4.0;

        m1[1][0] = 5.0;
        m1[1][1] = 6.0;
        m1[1][2] = 7.0;
        m1[1][3] = 8.0;

        m1[2][0] = 9.0;
        m1[2][1] = 8.0;
        m1[2][2] = 7.0;
        m1[2][3] = 6.0;

        m1[3][0] = 5.0;
        m1[3][1] = 4.0;
        m1[3][2] = 3.0;
        m1[3][3] = 2.0;

        let mut m2: Matrix = Default::default();
        m2[0][0] = 2.0;
        m2[0][1] = 3.0;
        m2[0][2] = 4.0;
        m2[0][3] = 5.0;

        m2[1][0] = 6.0;
        m2[1][1] = 7.0;
        m2[1][2] = 8.0;
        m2[1][3] = 9.0;

        m2[2][0] = 8.0;
        m2[2][1] = 7.0;
        m2[2][2] = 6.0;
        m2[2][3] = 5.0;

        m2[3][0] = 4.0;
        m2[3][1] = 3.0;
        m2[3][2] = 2.0;
        m2[3][3] = 1.0;

        assert_ne!(m1, m2);
    }

    #[test]
    fn matrix_multiplication() {
        let mut m1: Matrix = Default::default();
        m1[0][0] = 1.0;
        m1[0][1] = 2.0;
        m1[0][2] = 3.0;
        m1[0][3] = 4.0;

        m1[1][0] = 5.0;
        m1[1][1] = 6.0;
        m1[1][2] = 7.0;
        m1[1][3] = 8.0;

        m1[2][0] = 9.0;
        m1[2][1] = 8.0;
        m1[2][2] = 7.0;
        m1[2][3] = 6.0;

        m1[3][0] = 5.0;
        m1[3][1] = 4.0;
        m1[3][2] = 3.0;
        m1[3][3] = 2.0;

        let mut m2: Matrix = Default::default();
        m2[0][0] = -2.0;
        m2[0][1] = 1.0;
        m2[0][2] = 2.0;
        m2[0][3] = 3.0;

        m2[1][0] = 3.0;
        m2[1][1] = 2.0;
        m2[1][2] = 1.0;
        m2[1][3] = -1.0;

        m2[2][0] = 4.0;
        m2[2][1] = 3.0;
        m2[2][2] = 6.0;
        m2[2][3] = 5.0;

        m2[3][0] = 1.0;
        m2[3][1] = 2.0;
        m2[3][2] = 7.0;
        m2[3][3] = 8.0;

        let mut m3: Matrix = Default::default();
        m3[0][0] = 20.0;
        m3[0][1] = 22.0;
        m3[0][2] = 50.0;
        m3[0][3] = 48.0;

        m3[1][0] = 44.0;
        m3[1][1] = 54.0;
        m3[1][2] = 114.0;
        m3[1][3] = 108.0;

        m3[2][0] = 40.0;
        m3[2][1] = 58.0;
        m3[2][2] = 110.0;
        m3[2][3] = 102.0;

        m3[3][0] = 16.0;
        m3[3][1] = 26.0;
        m3[3][2] = 46.0;
        m3[3][3] = 42.0;

        assert_eq!(m1 * m2, m3);
    }

    #[test]
    fn matrix_multiplication_with_tuple() {
        let mut a: Matrix = Default::default();
        a[0][0] = 1.0;
        a[0][1] = 2.0;
        a[0][2] = 3.0;
        a[0][3] = 4.0;

        a[1][0] = 2.0;
        a[1][1] = 4.0;
        a[1][2] = 4.0;
        a[1][3] = 2.0;

        a[2][0] = 8.0;
        a[2][1] = 6.0;
        a[2][2] = 4.0;
        a[2][3] = 1.0;

        a[3][0] = 0.0;
        a[3][1] = 0.0;
        a[3][2] = 0.0;
        a[3][3] = 1.0;

        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let result = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(a * b, result);
    }

    #[test]
    fn matrix_multiplication_with_identity() {
        let mut a: Matrix = Default::default();
        a[0][0] = 0.0;
        a[0][1] = 1.0;
        a[0][2] = 2.0;
        a[0][3] = 4.0;

        a[1][0] = 1.0;
        a[1][1] = 2.0;
        a[1][2] = 4.0;
        a[1][3] = 8.0;

        a[2][0] = 2.0;
        a[2][1] = 4.0;
        a[2][2] = 8.0;
        a[2][3] = 16.0;

        a[3][0] = 4.0;
        a[3][1] = 8.0;
        a[3][2] = 16.0;
        a[3][3] = 32.0;

        let i = Matrix::I();
        assert_eq!(a * i, a);
    }

    #[test]
    fn tuple_multiplication_with_identity() {
        let i = Matrix::I();
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(i * a, a);
    }

    #[test]
    fn transpose_matrix() {
        let mut a: Matrix = Default::default();
        a[0][0] = 0.0;
        a[0][1] = 9.0;
        a[0][2] = 3.0;
        a[0][3] = 0.0;

        a[1][0] = 9.0;
        a[1][1] = 8.0;
        a[1][2] = 0.0;
        a[1][3] = 8.0;

        a[2][0] = 1.0;
        a[2][1] = 8.0;
        a[2][2] = 5.0;
        a[2][3] = 3.0;

        a[3][0] = 0.0;
        a[3][1] = 0.0;
        a[3][2] = 5.0;
        a[3][3] = 8.0;

        let mut b: Matrix = Default::default();
        b[0][0] = 0.0;
        b[0][1] = 9.0;
        b[0][2] = 1.0;
        b[0][3] = 0.0;

        b[1][0] = 9.0;
        b[1][1] = 8.0;
        b[1][2] = 8.0;
        b[1][3] = 0.0;

        b[2][0] = 3.0;
        b[2][1] = 0.0;
        b[2][2] = 5.0;
        b[2][3] = 5.0;

        b[3][0] = 0.0;
        b[3][1] = 8.0;
        b[3][2] = 3.0;
        b[3][3] = 8.0;
        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn transpose_identity_matrix() {
        assert_eq!(Matrix::I(), Matrix::I().transpose());
    }

    #[test]
    fn matrix_submatrix() {
        let mut a: Matrix = Default::default();
        a[0][0] = -6.0;
        a[0][1] = 1.0;
        a[0][2] = 1.0;
        a[0][3] = 6.0;

        a[1][0] = -8.0;
        a[1][1] = 5.0;
        a[1][2] = 8.0;
        a[1][3] = 6.0;

        a[2][0] = -1.0;
        a[2][1] = 0.0;
        a[2][2] = 8.0;
        a[2][3] = 2.0;

        a[3][0] = -7.0;
        a[3][1] = 1.0;
        a[3][2] = -1.0;
        a[3][3] = 1.0;

        let mut b: Matrix3 = Default::default();
        b[0][0] = -6.0;
        b[0][1] = 1.0;
        b[0][2] = 6.0;
        b[1][0] = -8.0;
        b[1][1] = 8.0;
        b[1][2] = 6.0;
        b[2][0] = -7.0;
        b[2][1] = -1.0;
        b[2][2] = 1.0;

        assert_eq!(a.submatrix(2, 1), b);
    }

    #[test]
    fn matrix_determinant() {
        let mut a: Matrix = Default::default();
        a[0][0] = -2.0;
        a[0][1] = -8.0;
        a[0][2] = 3.0;
        a[0][3] = 5.0;

        a[1][0] = -3.0;
        a[1][1] = 1.0;
        a[1][2] = 7.0;
        a[1][3] = 3.0;

        a[2][0] = 1.0;
        a[2][1] = 2.0;
        a[2][2] = -9.0;
        a[2][3] = 6.0;

        a[3][0] = -6.0;
        a[3][1] = 7.0;
        a[3][2] = 7.0;
        a[3][3] = -9.0;

        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn invertible_test1() {
        let mut a: Matrix = Default::default();
        a[0][0] = 6.0;
        a[0][1] = 4.0;
        a[0][2] = 4.0;
        a[0][3] = 4.0;

        a[1][0] = 5.0;
        a[1][1] = 5.0;
        a[1][2] = 7.0;
        a[1][3] = 6.0;

        a[2][0] = 4.0;
        a[2][1] = -9.0;
        a[2][2] = 3.0;
        a[2][3] = -7.0;

        a[3][0] = 9.0;
        a[3][1] = 1.0;
        a[3][2] = 7.0;
        a[3][3] = -6.0;

        assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    #[test]
    fn invertible_test2() {
        let mut a: Matrix = Default::default();
        a[0][0] = -4.0;
        a[0][1] = 2.0;
        a[0][2] = -2.0;
        a[0][3] = -3.0;

        a[1][0] = 9.0;
        a[1][1] = 6.0;
        a[1][2] = 2.0;
        a[1][3] = 6.0;

        a[2][0] = 0.0;
        a[2][1] = -5.0;
        a[2][2] = 1.0;
        a[2][3] = -5.0;

        a[3][0] = 0.0;
        a[3][1] = 0.0;
        a[3][2] = 0.0;
        a[3][3] = 0.0;

        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn inverse_test1() {
        let mut a: Matrix = Default::default();
        a[0][0] = -5.0;
        a[0][1] = 2.0;
        a[0][2] = 6.0;
        a[0][3] = -8.0;

        a[1][0] = 1.0;
        a[1][1] = -5.0;
        a[1][2] = 1.0;
        a[1][3] = 8.0;

        a[2][0] = 7.0;
        a[2][1] = 7.0;
        a[2][2] = -6.0;
        a[2][3] = -7.0;

        a[3][0] = 1.0;
        a[3][1] = -3.0;
        a[3][2] = 7.0;
        a[3][3] = 4.0;

        let b = a.inverse();

        let mut _b: Matrix = Default::default();
        _b[0][0] = 0.21805;
        _b[0][1] = 0.45113;
        _b[0][2] = 0.24060;
        _b[0][3] = -0.04511;

        _b[1][0] = -0.80827;
        _b[1][1] = -1.45677;
        _b[1][2] = -0.44361;
        _b[1][3] = 0.52068;

        _b[2][0] = -0.07895;
        _b[2][1] = -0.22368;
        _b[2][2] = -0.05263;
        _b[2][3] = 0.19737;

        _b[3][0] = -0.52256;
        _b[3][1] = -0.81391;
        _b[3][2] = -0.30075;
        _b[3][3] = 0.30639;

        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert!(b[3][2].eq(-160.0 / 532.0));
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert!(b[2][3].eq(105.0 / 532.0));
        assert_eq!(b, _b);
    }

    #[test]
    fn inverse_test2() {
        let mut a: Matrix = Default::default();
        a[0][0] = 8.0;
        a[0][1] = -5.0;
        a[0][2] = 9.0;
        a[0][3] = 2.0;

        a[1][0] = 7.0;
        a[1][1] = 5.0;
        a[1][2] = 6.0;
        a[1][3] = 1.0;

        a[2][0] = -6.0;
        a[2][1] = 0.0;
        a[2][2] = 9.0;
        a[2][3] = 6.0;

        a[3][0] = -3.0;
        a[3][1] = 0.0;
        a[3][2] = -9.0;
        a[3][3] = -4.0;

        let mut _a: Matrix = Default::default();
        _a[0][0] = -0.15385;
        _a[0][1] = -0.15385;
        _a[0][2] = -0.28205;
        _a[0][3] = -0.53846;

        _a[1][0] = -0.07692;
        _a[1][1] = 0.12308;
        _a[1][2] = 0.02564;
        _a[1][3] = 0.03077;

        _a[2][0] = 0.35897;
        _a[2][1] = 0.35897;
        _a[2][2] = 0.43590;
        _a[2][3] = 0.92308;

        _a[3][0] = -0.69231;
        _a[3][1] = -0.69231;
        _a[3][2] = -0.76923;
        _a[3][3] = -1.92308;

        assert_eq!(a.inverse(), _a);
    }

    #[test]
    fn inverse_test3() {
        let mut a: Matrix = Default::default();
        a[0][0] = 9.0;
        a[0][1] = 3.0;
        a[0][2] = 0.0;
        a[0][3] = 9.0;

        a[1][0] = -5.0;
        a[1][1] = -2.0;
        a[1][2] = -6.0;
        a[1][3] = -3.0;

        a[2][0] = -4.0;
        a[2][1] = 9.0;
        a[2][2] = 6.0;
        a[2][3] = 4.0;

        a[3][0] = -7.0;
        a[3][1] = 6.0;
        a[3][2] = 6.0;
        a[3][3] = 2.0;

        let mut _a: Matrix = Default::default();
        _a[0][0] = -0.04074;
        _a[0][1] = -0.07778;
        _a[0][2] = 0.14444;
        _a[0][3] = -0.22222;

        _a[1][0] = -0.07778;
        _a[1][1] = 0.03333;
        _a[1][2] = 0.36667;
        _a[1][3] = -0.33333;

        _a[2][0] = -0.02901;
        _a[2][1] = -0.14630;
        _a[2][2] = -0.10926;
        _a[2][3] = 0.12963;

        _a[3][0] = 0.17778;
        _a[3][1] = 0.06667;
        _a[3][2] = -0.26667;
        _a[3][3] = 0.33333;

        assert_eq!(a.inverse(), _a);
    }

    #[test]
    fn mulitplication_by_inverse() {
        let mut a: Matrix = Default::default();
        a[0][0] = 3.0;
        a[0][1] = -9.0;
        a[0][2] = 7.0;
        a[0][3] = 3.0;

        a[1][0] = 3.0;
        a[1][1] = -8.0;
        a[1][2] = 2.0;
        a[1][3] = -9.0;

        a[2][0] = -4.0;
        a[2][1] = 4.0;
        a[2][2] = 4.0;
        a[2][3] = 1.0;

        a[3][0] = -6.0;
        a[3][1] = 5.0;
        a[3][2] = -1.0;
        a[3][3] = 1.0;

        let mut b: Matrix = Default::default();
        b[0][0] = 8.0;
        b[0][1] = 2.0;
        b[0][2] = 2.0;
        b[0][3] = 2.0;

        b[1][0] = 3.0;
        b[1][1] = -1.0;
        b[1][2] = 7.0;
        b[1][3] = 0.0;

        b[2][0] = 7.0;
        b[2][1] = 0.0;
        b[2][2] = 5.0;
        b[2][3] = 4.0;

        b[3][0] = 6.0;
        b[3][1] = -2.0;
        b[3][2] = 0.0;
        b[3][3] = 5.0;

        let c = a * b;
        assert_eq!(c * b.inverse(), a);
    }
}
