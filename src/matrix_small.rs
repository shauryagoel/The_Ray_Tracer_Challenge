use crate::Compare;

#[derive(Debug, Default)]
pub struct Matrix2 {
    pub data: [[f64; 2]; 2],
}

#[derive(Debug, Default)]
pub struct Matrix3 {
    pub data: [[f64; 3]; 3],
}

impl Matrix2 {
    // Initialize a new matrix with 0's
    pub fn new() -> Self {
        Default::default()
    }

    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Matrix3 {
    // Initialize a new matrix with 0's
    pub fn new() -> Self {
        Default::default()
    }

    // Generate a 2x2 submatrix from the 3x3 by removing all the data in `row_ind` row and `col_ind` column
    pub fn submatrix(&self, row_ind: usize, col_ind: usize) -> Matrix2 {
        let mut result: Matrix2 = Default::default();
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
        for i in 0..3 {
            result += self.cofactor(0, i) * self[0][i];
        }
        result
    }
}

// Use this to get a row of matrix when indexing Matrix
impl std::ops::Index<usize> for Matrix2 {
    type Output = [f64; 2];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl std::ops::IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

impl std::ops::Index<usize> for Matrix3 {
    type Output = [f64; 3];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl std::ops::IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

// TODO: Remove duplicate code for Matrix2 and Matrix3
impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..2 {
            for col in 0..2 {
                if self.data[row][col].neq(other[row][col]) {
                    false;
                }
            }
        }
        true
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.data[row][col].neq(other[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod small_matrix_tests {
    use super::*;

    #[test]
    fn matrix_init2x2() {
        let mut m: Matrix2 = Default::default();
        m[0][0] = -3.0;
        m[0][1] = 5.0;
        m[1][0] = 1.0;
        m[1][1] = -2.0;

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn matrix_init3x3() {
        let mut m: Matrix3 = Default::default();
        m[0][0] = -3.0;
        m[0][1] = 5.0;
        m[1][0] = 1.0;
        m[1][1] = -2.0;
        m[1][2] = -7.0;
        m[2][1] = 1.0;
        m[2][2] = 1.0;

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn matrix2_determinant() {
        let mut a: Matrix2 = Default::default();
        a[0][0] = 1.0;
        a[0][1] = 5.0;
        a[1][0] = -3.0;
        a[1][1] = 2.0;

        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn matrix3_submatrix() {
        let mut a: Matrix3 = Default::default();
        a[0][0] = 1.0;
        a[0][1] = 5.0;
        a[0][2] = 0.0;
        a[1][0] = -3.0;
        a[1][1] = 2.0;
        a[1][2] = 7.0;
        a[2][0] = 0.0;
        a[2][1] = 6.0;
        a[2][2] = -3.0;

        let mut b: Matrix2 = Default::default();
        b[0][0] = -3.0;
        b[0][1] = 2.0;
        b[1][0] = 0.0;
        b[1][1] = 6.0;

        assert_eq!(a.submatrix(0, 2), b);
    }

    #[test]
    fn matrix3_minor() {
        let mut a: Matrix3 = Default::default();
        a[0][0] = 3.0;
        a[0][1] = 5.0;
        a[0][2] = 0.0;
        a[1][0] = 2.0;
        a[1][1] = -1.0;
        a[1][2] = -7.0;
        a[2][0] = 6.0;
        a[2][1] = -1.0;
        a[2][2] = 5.0;

        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn matrix3_cofactor() {
        let mut a: Matrix3 = Default::default();
        a[0][0] = 3.0;
        a[0][1] = 5.0;
        a[0][2] = 0.0;
        a[1][0] = 2.0;
        a[1][1] = -1.0;
        a[1][2] = -7.0;
        a[2][0] = 6.0;
        a[2][1] = -1.0;
        a[2][2] = 5.0;

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn matrix3_determinant() {
        let mut a: Matrix3 = Default::default();
        a[0][0] = 1.0;
        a[0][1] = 2.0;
        a[0][2] = 6.0;
        a[1][0] = -5.0;
        a[1][1] = 8.0;
        a[1][2] = -4.0;
        a[2][0] = 2.0;
        a[2][1] = 6.0;
        a[2][2] = 4.0;

        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }
}
