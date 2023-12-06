const TOLERANCE: f64 = 0.0001;

pub trait Zeq<T> {
    fn zeq(&self, other: T) -> bool;

    fn zneg(&self, other: T) -> bool {
        !self.zeq(other)
    }
}

impl Zeq<Self> for f64 {
    fn zeq(&self, other: Self) -> bool {
        let epsilon = TOLERANCE;
        (*self - other).abs() < epsilon
    }
}

#[macro_export]
macro_rules! assert_zeq {
    ($left:expr,$right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left, right) => {
                if left.zneg(*right) {
                    panic!(
                        "asserting zequality between {:?} and {:?} failed",
                        left, right
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_nzeq {
    ($left:expr,$right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left, right) => {
                if left.zeq(*right) {
                    panic!(
                        "asserting inzequality between {:?} and {:?} failed",
                        left, right
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zequality() {
        let a = 0.001;
        let b = 0.002;
        assert_nzeq!(a, b);
    }
    #[test]
    fn nzequality() {
        let a = 0.001;
        let b = 0.00101;
        assert_zeq!(a, b);
    }
}
