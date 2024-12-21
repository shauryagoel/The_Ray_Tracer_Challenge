use crate::Matrix;
use crate::Tuple;

impl Matrix {
    pub fn get_translation_matrix(x: f64, y: f64, z: f64) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[0][3] = x;
        transformation_matrix[1][3] = y;
        transformation_matrix[2][3] = z;
        transformation_matrix
    }

    pub fn get_scaling_matrix(x: f64, y: f64, z: f64) -> Self {
        let mut transformation_matrix: Matrix = Default::default();
        transformation_matrix[0][0] = x;
        transformation_matrix[1][1] = y;
        transformation_matrix[2][2] = z;
        transformation_matrix[3][3] = 1.0;
        transformation_matrix
    }

    // Get rotation matrix around +x direction, r is in radians
    pub fn get_rotation_x_matrix(r: f64) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[1][1] = r.cos();
        transformation_matrix[1][2] = -r.sin();
        transformation_matrix[2][1] = r.sin();
        transformation_matrix[2][2] = r.cos();
        transformation_matrix
    }

    // Get rotation matrix around +y direction, r is in radians
    pub fn get_rotation_y_matrix(r: f64) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[0][0] = r.cos();
        transformation_matrix[0][2] = r.sin();
        transformation_matrix[2][0] = -r.sin();
        transformation_matrix[2][2] = r.cos();
        transformation_matrix
    }

    // Get rotation matrix around +z direction, r is in radians
    pub fn get_rotation_z_matrix(r: f64) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[0][0] = r.cos();
        transformation_matrix[0][1] = -r.sin();
        transformation_matrix[1][0] = r.sin();
        transformation_matrix[1][1] = r.cos();
        transformation_matrix
    }

    pub fn get_shearing_matrix(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[0][1] = xy;
        transformation_matrix[0][2] = xz;
        transformation_matrix[1][0] = yx;
        transformation_matrix[1][2] = yz;
        transformation_matrix[2][0] = zx;
        transformation_matrix[2][1] = zy;
        transformation_matrix
    }

    /// Compute the view transform matrix to move the eye
    ///
    /// * `from`: position where the eye is
    /// * `to`: the point to look at
    /// * `up`: vector indicating the up direction
    pub fn get_view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
        let forward = (to - from).normalize();
        let left = forward.cross(&up.normalize());
        let true_up = left.cross(&forward);
        let mut orientation = Matrix::I();
        orientation[0][0] = left.x;
        orientation[0][1] = left.y;
        orientation[0][2] = left.z;
        orientation[1][0] = true_up.x;
        orientation[1][1] = true_up.y;
        orientation[1][2] = true_up.z;
        orientation[2][0] = -forward.x;
        orientation[2][1] = -forward.y;
        orientation[2][2] = -forward.z;
        orientation * Matrix::get_translation_matrix(-from.x, -from.y, -from.z)
    }
}

#[cfg(test)]
mod transformation_tests {
    use super::*;
    use crate::{point, vector};
    use std::f64::consts::PI;

    #[test]
    fn point_translation1() {
        let transform = Matrix::get_translation_matrix(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        let _p = point(2.0, 1.0, 7.0);

        assert_eq!(transform * p, _p);
    }

    #[test]
    fn point_translation2() {
        let transform = Matrix::get_translation_matrix(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        let _p = point(-8.0, 7.0, 3.0);

        assert_eq!(inv * p, _p);
    }

    #[test]
    fn vector_translation() {
        let transform = Matrix::get_translation_matrix(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn point_scaling() {
        let transform = Matrix::get_scaling_matrix(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        let _p = point(-8.0, 18.0, 32.0);

        assert_eq!(transform * p, _p);
    }

    #[test]
    fn vector_scaling() {
        let transform = Matrix::get_scaling_matrix(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        let _v = vector(-8.0, 18.0, 32.0);

        assert_eq!(transform * v, _v);
    }

    #[test]
    fn inverse_scaling() {
        let transform = Matrix::get_scaling_matrix(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = vector(-4.0, 6.0, 8.0);
        let _v = vector(-2.0, 2.0, 2.0);

        assert_eq!(inv * v, _v);
    }

    #[test]
    fn reflection_scaling() {
        let transform = Matrix::get_scaling_matrix(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(-2.0, 3.0, 4.0);

        assert_eq!(transform * p, _p);
    }

    #[test]
    fn rotation_x() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::get_rotation_x_matrix(PI / 4.0);
        let full_quarter = Matrix::get_rotation_x_matrix(PI / 2.0);
        let _p1 = point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
        let _p2 = point(0.0, 0.0, 1.0);

        assert_eq!(half_quarter * p, _p1);
        assert_eq!(full_quarter * p, _p2);
    }

    #[test]
    fn inverse_rotation_x() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::get_rotation_x_matrix(PI / 4.0);
        let inv = half_quarter.inverse();
        let _p = point(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt()) / 2.0);
        assert_eq!(inv * p, _p);
    }

    #[test]
    fn rotation_y() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::get_rotation_y_matrix(PI / 4.0);
        let full_quarter = Matrix::get_rotation_y_matrix(PI / 2.0);
        let _p1 = point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
        let _p2 = point(1.0, 0.0, 0.0);

        assert_eq!(half_quarter * p, _p1);
        assert_eq!(full_quarter * p, _p2);
    }

    #[test]
    fn rotation_z() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::get_rotation_z_matrix(PI / 4.0);
        let full_quarter = Matrix::get_rotation_z_matrix(PI / 2.0);
        let _p1 = point(-(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let _p2 = point(-1.0, 0.0, 0.0);

        assert_eq!(half_quarter * p, _p1);
        assert_eq!(full_quarter * p, _p2);
    }

    #[test]
    fn shear_x_according_y() {
        let transform = Matrix::get_shearing_matrix(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(5.0, 3.0, 4.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_x_according_z() {
        let transform = Matrix::get_shearing_matrix(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(6.0, 3.0, 4.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_y_according_x() {
        let transform = Matrix::get_shearing_matrix(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(2.0, 5.0, 4.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_y_according_z() {
        let transform = Matrix::get_shearing_matrix(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(2.0, 7.0, 4.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_z_according_x() {
        let transform = Matrix::get_shearing_matrix(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(2.0, 3.0, 6.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_z_according_y() {
        let transform = Matrix::get_shearing_matrix(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(2.0, 3.0, 7.0);
        assert_eq!(transform * p, _p);
    }

    #[allow(non_snake_case)]
    #[test]
    fn chaining_transformations1() {
        let p = point(1.0, 0.0, 1.0);
        let A = Matrix::get_rotation_x_matrix(PI / 2.0);
        let B = Matrix::get_scaling_matrix(5.0, 5.0, 5.0);
        let C = Matrix::get_translation_matrix(10.0, 5.0, 7.0);

        let p2 = A * p;
        assert_eq!(p2, point(1.0, -1.0, 0.0));

        let p3 = B * p2;
        assert_eq!(p3, point(5.0, -5.0, 0.0));

        let p4 = C * p3;
        assert_eq!(p4, point(15.0, 0.0, 7.0));
    }

    #[allow(non_snake_case)]
    #[test]
    fn chaining_transformations2() {
        let p = point(1.0, 0.0, 1.0);
        let A = Matrix::get_rotation_x_matrix(PI / 2.0);
        let B = Matrix::get_scaling_matrix(5.0, 5.0, 5.0);
        let C = Matrix::get_translation_matrix(10.0, 5.0, 7.0);
        let T = C * B * A;

        assert_eq!(T * p, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn default_view_transform() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = Matrix::get_view_transform(from, to, up);
        assert_eq!(t, Matrix::I());
    }

    #[test]
    fn view_transform_looking_in_positive_z_direction() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = Matrix::get_view_transform(from, to, up);
        assert_eq!(t, Matrix::get_scaling_matrix(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transform_moves_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = Matrix::get_view_transform(from, to, up);
        assert_eq!(t, Matrix::get_translation_matrix(0.0, 0.0, -8.0));
    }

    #[test]
    fn view_transform_arbitrary() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);
        let t = Matrix::get_view_transform(from, to, up);
        let mut tm = Matrix::I();
        tm[0][0] = -0.50709;
        tm[0][1] = 0.50709;
        tm[0][2] = 0.67612;
        tm[0][3] = -2.36643;
        tm[1][0] = 0.76772;
        tm[1][1] = 0.60609;
        tm[1][2] = 0.12122;
        tm[1][3] = -2.82843;
        tm[2][0] = -0.35857;
        tm[2][1] = 0.59761;
        tm[2][2] = -0.71714;
        assert_eq!(t, tm);
    }
}
