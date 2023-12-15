mod point;
mod vector;
pub mod approx_eq;
mod color;

pub use point::Point;
pub use vector::Vector;
pub use color::Color;
use crate::approx_eq::ApproxEq;

pub type Tuple4X = (f64, f64, f64, f64);

impl ApproxEq for Tuple4X {
    fn approx_eq(&self, other: &Self) -> bool {
        let (x, y, z, w) = self;
        let (x_o, y_o, z_o, w_o) = other;

        x.approx_eq(x_o) && y.approx_eq(y_o) && z.approx_eq(z_o) && w.approx_eq(w_o)
    }
}
pub type Tuple3X = (f64, f64, f64);

impl ApproxEq for Tuple3X {
    fn approx_eq(&self, other: &Self) -> bool {
        let (x, y, z) = self;
        let (x_o, y_o, z_o) = other;

        x.approx_eq(x_o) && y.approx_eq(y_o) && z.approx_eq(z_o)
    }
}