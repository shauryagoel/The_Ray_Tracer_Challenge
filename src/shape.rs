use crate::Intersections;
use crate::Material;
use crate::Matrix;
use crate::Ray;
use crate::Tuple;

pub trait Shape {
    fn set_transform(&mut self, m: Matrix);
    // fn set_local_ray(&mut self, local_ray: Ray);
    fn get_transform(&self) -> Matrix;

    fn intersect(&self, ray: Ray) -> Intersections {
        // Transform the ray to the object space coordinates of the shape
        // This means applying inverse transformation of the shape to the ray
        let local_ray = ray.transform(self.get_transform().inverse());
        // self.set_local_ray(local_ray);
        self.local_intersect(local_ray)
    }
    fn local_intersect(&self, local_ray: Ray) -> Intersections;

    // Find normal of the shape at the world point `p`
    // Basically transform the point to the object space, find normal at that point
    // and then, transform it back to the world space
    // Derivation is given in the chapter 6 README
    fn normal_at(&self, p: Tuple) -> Tuple {
        let local_point = self.get_transform().inverse() * p;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal = self.get_transform().inverse().transpose() * local_normal;
        // This is needed as we are multiplying and transposing the complete transformation matrix
        // it can lead to weird `w` values
        world_normal.w = 0.0;
        world_normal.normalize()
    }
    fn local_normal_at(&self, p: Tuple) -> Tuple;
}

struct TestShape {
    transform: Matrix, // Transformation matrix
    material: Material, // Shape's material
                       // saved_ray: Ray,     // Store the transformed ray
}

impl Shape for TestShape {
    fn get_transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, m: Matrix) {
        self.transform = m;
    }

    // fn set_local_ray(&mut self, local_ray: Ray) {
    //     self.saved_ray = local_ray;
    // }

    fn local_intersect(&self, _local_ray: Ray) -> Intersections {
        Intersections::new()
    }

    // Convert the point to the vector
    fn local_normal_at(&self, mut point: Tuple) -> Tuple {
        point.w = 0.0;
        point
    }
}

impl Default for TestShape {
    fn default() -> Self {
        Self {
            transform: Matrix::I(),
            material: Material::default(),
            // saved_ray: Ray::new(
            //     Tuple {
            //         x: 0.0,
            //         y: 0.0,
            //         z: 0.0,
            //         w: 0.0,
            //     },
            //     Tuple {
            //         x: 0.0,
            //         y: 0.0,
            //         z: 0.0,
            //         w: 0.0,
            //     },
            // ),
        }
    }
}

#[cfg(test)]
mod testshape_tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use super::*;
    use crate::{point, vector};

    #[test]
    fn default_transformation() {
        let s = TestShape::default();
        assert_eq!(s.transform, Matrix::I());
    }

    #[test]
    fn assigning_transformation() {
        let mut s = TestShape::default();
        let m = Matrix::get_translation_matrix(2.0, 3.0, 4.0);
        s.set_transform(m);
        assert_eq!(s.transform, m);
    }

    #[test]
    fn default_material() {
        let s = TestShape::default();
        let m = s.material;
        assert_eq!(m, Material::default());
    }

    #[test]
    fn assign_material() {
        let mut s = TestShape::default();
        let m = Material {
            ambient: 1.0,
            ..Default::default()
        };
        s.material = m;
        assert_eq!(m, s.material);
    }

    // #[test]
    // fn intersection_scaled_shape_with_ray() {
    //     let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    //     let mut s = TestShape::default();
    //     s.set_transform(Matrix::get_scaling_matrix(2.0, 2.0, 2.0));
    //     s.intersect(r);
    //     assert_eq!(s.saved_ray.origin, point(0.0, 0.0, -2.5));
    //     assert_eq!(s.saved_ray.direction, vector(0.0, 0.0, 0.5));
    // }
    //
    // #[test]
    // fn intersection_translated_shape_with_ray() {
    //     let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    //     let mut s = TestShape::default();
    //     s.set_transform(Matrix::get_translation_matrix(5.0, 0.0, 0.0));
    //     s.intersect(r);
    //     assert_eq!(s.saved_ray.origin, point(-5.0, 0.0, -5.0));
    //     assert_eq!(s.saved_ray.direction, vector(0.0, 0.0, 1.0));
    // }

    #[test]
    fn normal_of_translated_shape() {
        let mut s = TestShape::default();
        s.set_transform(Matrix::get_translation_matrix(0.0, 1.0, 0.0));
        let n = s.normal_at(point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_eq!(n, vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_of_transformed_shape() {
        let mut s = TestShape::default();
        let m = Matrix::get_scaling_matrix(1.0, 0.5, 1.0) * Matrix::get_rotation_z_matrix(PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(point(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }
}
