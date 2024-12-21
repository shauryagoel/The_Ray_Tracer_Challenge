// Utility functions or structs or traits

pub const EPSILON: f64 = 1.0e-5;

pub trait Compare {
    fn eq(self, other: f64) -> bool;
    fn neq(self, other: f64) -> bool;
}

impl Compare for f64 {
    fn eq(self, other: f64) -> bool {
        (self - other).abs() < EPSILON
    }

    fn neq(self, other: f64) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod utility_test {
    use super::*;

    #[test]
    fn float_compare_equal1() {
        let a: f64 = 3.3;
        let b: f64 = 3.3;
        assert!(a.eq(b));
    }

    #[test]
    fn float_compare_equal2() {
        let a: f64 = 0.0;
        let b: f64 = 0.000005;
        assert!(a.eq(b));
    }

    #[test]
    fn float_compare_not_equal1() {
        let a: f64 = 3.3;
        let b: f64 = 3.2;
        assert!(a.neq(b));
    }

    #[test]
    fn float_compare_not_equal2() {
        let a: f64 = 3.3;
        let b: f64 = 3.2;
        assert!(a.neq(b));
    }
}
