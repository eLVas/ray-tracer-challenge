pub trait ApproxEq {
    const EPSILON: f64 = 0.00001;
    fn approx_eq(&self, other: &Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        f64::abs(self - other) < <Self as ApproxEq>::EPSILON
    }
}