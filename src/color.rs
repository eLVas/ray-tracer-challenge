use std::ops;
use crate::approx_eq::ApproxEq;
use crate::Tuple3X;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color(pub Tuple3X);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color((r, g, b))
    }
}

impl ApproxEq for Color {
    fn approx_eq(&self, other: &Self) -> bool {
        self.0.approx_eq(&other.0)
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.0;
        let (r2, g2, b2) = rhs.0;

        Color::new(r1+r2, g1+g2, b1+b2)
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.0;
        let (r2, g2, b2) = rhs.0;

        Color::new(r1-r2, g1-g2, b1-b2)
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.0;
        let (r2, g2, b2) = rhs.0;

        Color::new(r1*r2, g1*g2, b1*b2)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        let (r1, g1, b1) = self.0;

        Color::new(r1*rhs, g1*rhs, b1*rhs)
    }
}



