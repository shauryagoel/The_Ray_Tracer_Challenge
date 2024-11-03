// Utility functions or structs or traits

pub const EPSILON: f32 = 1.0e-5;

pub trait Compare {
    fn eq(self, other: f32) -> bool;
    fn neq(self, other: f32) -> bool;
}

impl Compare for f32 {
    fn eq(self, other: f32) -> bool {
        (self - other).abs() < EPSILON
    }

    fn neq(self, other: f32) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod utility_test {
    use super::*;

    #[test]
    fn float_compare_equal1() {
        let a: f32 = 3.3;
        let b: f32 = 3.3;
        assert!(a.eq(b));
    }

    #[test]
    fn float_compare_equal2() {
        let a: f32 = 0.0;
        let b: f32 = 0.000005;
        assert!(a.eq(b));
    }

    #[test]
    fn float_compare_not_equal1() {
        let a: f32 = 3.3;
        let b: f32 = 3.2;
        assert!(a.neq(b));
    }

    #[test]
    fn float_compare_not_equal2() {
        let a: f32 = 3.3;
        let b: f32 = 3.2;
        assert!(a.neq(b));
    }
}
