use crate::vector;
use crate::Intersection;
use crate::Intersections;
use crate::Matrix;
use crate::Ray;
use crate::Shape;
use crate::Tuple;
use std::f64::EPSILON;

// Default plane is in xz, passing through the origin
pub struct Plane {
    transform: Matrix, // Transformation applied to the plane
}

impl Plane {
    pub fn new(transform: Matrix) -> Self {
        Self { transform }
    }
}

impl Shape for Plane {
    fn set_transform(&mut self, t: Matrix) {
        self.transform = t;
    }

    fn get_transform(&self) -> Matrix {
        self.transform
    }

    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        let mut intersections = Intersections::default();
        if local_ray.direction.y.abs() < EPSILON {
            return intersections;
        }

        let t = -local_ray.origin.y / local_ray.direction.y;
        intersections.push(Intersection::new(t, *self));
        intersections
    }

    fn local_normal_at(&self, p: Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new(Matrix::I())
    }
}

#[cfg(test)]
mod plane_tests {
    use super::*;
    use crate::point;

    #[test]
    fn normal_is_constant() {
        let p = Plane::default();
        let n1 = p.local_normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(point(-5.0, 0.0, 150.0));

        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_parallel_ray() {
        let p = Plane::default();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(r);
        assert!(xs.is_empty())
    }
}
