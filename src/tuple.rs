use crate::Compare;

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64, // x coordinate of the Tuple
    pub y: f64, // y coordinate of the Tuple
    pub z: f64, // z coordinate of the Tuple
    pub w: f64, // indicator whether the Tuple is a vector(w=0.0) or a point(w=1.0). It is float as we need it for computation rather than an actual indicator
}

// Factory methods for creating vector and point
pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    // Pythagoras theorem
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    // We get a unit vector after normalization
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    // Dot product of 2 vectors
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    // Cross product of 2 vectors
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            0.0, // It still is a vector
        )
    }

    // Reflection vector of a vector with respect to a normal
    // For derivation, see the chapter 6 README.md
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * 2.0 * self.dot(normal)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(other.x) && self.y.eq(other.y) && self.z.eq(other.z) && self.w.eq(other.w)
    }
}

// For Tuple + Tuple
impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

// For Tuple - Tuple
impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

// For tuple * scalar
impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, a: f64) -> Self {
        Self::new(self.x * a, self.y * a, self.z * a, self.w * a)
    }
}

// For scalar * tuple
impl std::ops::Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, a: Tuple) -> Tuple {
        Tuple::new(a.x * self, a.y * self, a.z * self, a.w * self)
    }
}

// For tuple / scalar
impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, a: f64) -> Self {
        Self::new(self.x / a, self.y / a, self.z / a, self.w / a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn point_validity() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn vector_validity() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_factory() {
        let p = point(4.0, -4.0, 3.0);
        let _p = Tuple::new(4.0, -4.0, 3.0, 1.0);
        assert_eq!(p, _p);
    }

    #[test]
    fn vector_factory() {
        let p = vector(4.0, -4.0, 3.0);
        let _p = Tuple::new(4.0, -4.0, 3.0, 0.0);
        assert_eq!(p, _p);
    }

    #[test]
    fn tuple_addition() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let _a = Tuple::new(1.0, 1.0, 6.0, 1.0);
        assert_eq!(a1 + a2, _a)
    }

    #[test]
    fn point_subtraction() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let _p = vector(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, _p);
    }

    #[test]
    fn point_vector_backward() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        let _a = point(-2.0, -4.0, -6.0);
        assert_eq!(p - v, _a);
    }

    #[test]
    fn vector_subtraction() {
        let p1 = vector(3.0, 2.0, 1.0);
        let p2 = vector(5.0, 6.0, 7.0);
        let _p = vector(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, _p);
    }

    #[test]
    fn reverse_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        let _v = vector(-1.0, 2.0, -3.0);
        assert_eq!(zero - v, _v);
    }

    #[test]
    fn negation() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let _a = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        assert_eq!(-a, _a);
    }

    #[test]
    fn scaling_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let _a = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert_eq!(a * 3.5, _a);
    }

    #[test]
    fn fraction_scaling_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let _a = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(a * 0.5, _a);
    }

    #[test]
    fn dividing_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let _a = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(a / 2.0, _a);
    }

    #[test]
    fn vector_magnitude1() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude2() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude3() {
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude4() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn vector_magnitude5() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn vector_normalize1() {
        let v = vector(4.0, 0.0, 0.0);
        let _v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.normalize(), _v);
    }

    #[test]
    fn vector_normalize2() {
        let v = vector(1.0, 2.0, 3.0);
        let _v = vector(0.26726, 0.53452, 0.80178);
        assert_eq!(v.normalize(), _v);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = vector(1.0, 2.0, 3.0);
        let mag = v.normalize().magnitude();
        assert!(mag.eq(1.0));
    }

    #[test]
    fn dot_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        let dot_product = a.dot(&b);
        assert!(dot_product.eq(20.0));
    }

    #[test]
    fn cross_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        let _v = vector(-1.0, 2.0, -1.0);
        assert_eq!(a.cross(&b), _v);
        assert_eq!(b.cross(&a), -_v);
    }

    #[test]
    fn reflecting_at_y_axis() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_at_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let n = vector(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, vector(1.0, 0.0, 0.0));
    }
}
