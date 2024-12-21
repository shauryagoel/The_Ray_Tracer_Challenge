use crate::Material;
use crate::Ray;
use crate::Shape;
use crate::{point, Matrix, Tuple};
use crate::{Intersection, Intersections};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    // TODO: add `id` to it as described in the book
    center: Tuple,
    radius: f64,
    transform: Matrix,      // Transformation matrix
    pub material: Material, // Material of the sphere
}

impl Sphere {
    pub fn new(center: Tuple, radius: f64, transform: Matrix, material: Material) -> Self {
        Self {
            center,
            radius,
            transform,
            material,
        }
    }
}

impl Shape for Sphere {
    fn set_transform(&mut self, t: Matrix) {
        self.transform = t;
    }

    fn get_transform(&self) -> Matrix {
        self.transform
    }

    // Returns the time(s) at which the `ray` intersects the sphere
    // Ray is assumed to be in object space
    fn local_intersect(&self, ray: Ray) -> Intersections {
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        let mut intersections: Intersections = Default::default();

        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            intersections.push(Intersection::new(t1, *self));
            intersections.push(Intersection::new(t2, *self));
        }
        intersections
    }

    // Compute normal at the point `p`
    // `p` is assumed to be in the object space
    fn local_normal_at(&self, p: Tuple) -> Tuple {
        p - point(0.0, 0.0, 0.0)
    }
}

impl Default for Sphere {
    // Create a sphere centered at origin, of radius 1, with identity transformation matrix and with default material
    fn default() -> Self {
        Self::new(point(0.0, 0.0, 0.0), 1.0, Matrix::I(), Material::default())
    }
}

// impl PartialEq for Sphere {
//     fn eq(&self, other: &Self) -> bool {
//         self.center == other.center && self.radius.eq(other.radius)
//     }
// }

#[cfg(test)]
mod sphere_tests {
    use super::*;
    use crate::vector;

    #[test]
    fn sphere_ray_intersection1() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.local_intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn sphere_ray_intersection2() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.local_intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn sphere_ray_intersection3() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn sphere_ray_intersection_ray_origin_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.local_intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_ray_intersection_before_ray_origin() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.local_intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn sphere_ray_intersection_object_property() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.local_intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, Sphere::default());
        assert_eq!(xs[1].object, Sphere::default());
    }

    #[test]
    fn sphere_normal_at_x_axis() {
        let s: Sphere = Default::default();
        let n = s.local_normal_at(point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn sphere_normal_at_y_axis() {
        let s: Sphere = Default::default();
        let n = s.local_normal_at(point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn sphere_normal_at_z_axis() {
        let s: Sphere = Default::default();
        let n = s.local_normal_at(point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn sphere_normal_at_non_axial_point() {
        let s: Sphere = Default::default();
        let val: f64 = f64::sqrt(3.0) / 3.0;
        let n = s.local_normal_at(point(val, val, val));
        assert_eq!(n, vector(val, val, val));
    }

    #[test]
    fn sphere_normal_is_normalized() {
        let s: Sphere = Default::default();
        let val: f64 = f64::sqrt(3.0) / 3.0;
        let n = s.local_normal_at(point(val, val, val));
        assert_eq!(n, n.normalize());
    }
}
