use crate::utils::EPSILON;
use crate::Ray;
use crate::Sphere;
use crate::Tuple;

// Store data for ray intersection with a object in the scene
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,         // At what time hit occured
    pub object: Sphere, // TODO: use dynamic data type
}

// Store vector of all intersections
#[derive(Default)]
pub struct Intersections {
    data: Vec<Intersection>,
}

// Store some precomputations for the intersection
pub struct Computation {
    t: f64,
    pub object: Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    inside: bool,
    pub over_point: Tuple, // Just slightly above the point towards the normal to avoid `acne`
}

impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Self {
        Self { t, object }
    }

    // Create computation object for ray intersection with the object
    pub fn prepare_computations(&self, ray: &Ray) -> Computation {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(point);
        let mut inside = false;

        // when eye vector is inside the sphere, negate the normal vector
        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }
        // let over_point = point + normalv * EPSILON * 1000.0; // NOTE: why do we need this so large?? To compensate for f64 rounding errors -> use f64
        let over_point = point + normalv * EPSILON; // NOTE: use this in f64 to pass tests

        Computation {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

// impl PartialEq for Intersection {
//     fn eq(&self, other: &Self) -> bool {
//         self.t == other.t && self.object == other.object
//     }
// }

impl Intersections {
    pub fn new() -> Self {
        Default::default()
    }

    // Self has only a vector so abstract out push
    pub fn push(&mut self, intersection: Intersection) {
        self.data.push(intersection)
    }

    // Self has only a vector so abstract out length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // Append one Intersection object with the other
    // Basically, append the data vector of both
    pub fn extend(&mut self, b: Intersections) {
        self.data.extend(b.data);
    }

    // Sort the Intersections based on increasing `t` value
    pub fn sort(&mut self) {
        self.data.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Hit is the `intersection` with the lowest non-negative value.
    // Can be empty as well.
    pub fn hit(&self) -> Option<Intersection> {
        let mut result: Option<Intersection> = None;
        for &intersection in &self.data {
            if intersection.t > 0.0 {
                match result {
                    Some(prev_intersection) => {
                        if intersection.t < prev_intersection.t {
                            result = Some(intersection);
                        }
                    }
                    None => result = Some(intersection),
                }
            }
        }
        result
    }
}

// Self has only a vector so abstract out indexing
impl std::ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl std::ops::IndexMut<usize> for Intersections {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

#[cfg(test)]
mod sphere_tests {
    use super::*;
    use crate::{point, vector};
    use crate::{Matrix, Ray};

    #[test]
    fn intersection_creation() {
        let s = Default::default();
        let i = Intersection::new(3.5, Default::default());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn intersections_test() {
        let s: Sphere = Default::default(); // INFO: this will work if we remove `Sphere`. As compiler infers it from the next line
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let mut xs: Intersections = Default::default();
        xs.push(i1);
        xs.push(i2);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersection_returning_objects() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.intersect(r);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn hit1() {
        let s: Sphere = Default::default();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let mut xs = Intersections::default();
        xs.push(i2);
        xs.push(i1);
        let i = xs.hit();
        assert_eq!(i.unwrap(), i1);
    }

    #[test]
    fn hit2() {
        let s: Sphere = Default::default();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let mut xs = Intersections::default();
        xs.push(i2);
        xs.push(i1);
        let i = xs.hit();
        assert_eq!(i.unwrap(), i2);
    }

    #[test]
    fn hit3() {
        let s: Sphere = Default::default();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let mut xs = Intersections::default();
        xs.push(i2);
        xs.push(i1);
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn hit4() {
        let s: Sphere = Default::default();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let mut xs = Intersections::default();
        xs.push(i1);
        xs.push(i2);
        xs.push(i3);
        xs.push(i4);
        let i = xs.hit();
        assert_eq!(i.unwrap(), i4);
    }

    #[test]
    fn precomputing_intersection_state() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Sphere = Default::default();
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_is_outside() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Sphere = Default::default();
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_intersection_is_inside() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape: Sphere = Default::default();
        let i = Intersection::new(1.0, shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = Sphere::default();
        shape.set_transform(Matrix::get_translation_matrix(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, shape);
        let comps = i.prepare_computations(&r);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
