use crate::Tuple4X;

pub trait ApproxEq {
    const EPSILON: f64 = 0.00001;
    fn approx_eq(&self, other: &Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        f64::abs(self - other) < <Self as ApproxEq>::EPSILON
    }
}

impl ApproxEq for Tuple4X {
    fn approx_eq(&self, other: &Self) -> bool {
        let (x, y, z, w) = self;
        let (x_o, y_o, z_o, w_o) = other;

        x.approx_eq(x_o) && y.approx_eq(y_o) && z.approx_eq(z_o) && w.approx_eq(w_o)
    }
}