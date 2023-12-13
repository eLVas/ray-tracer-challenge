use std::ops;
use crate::Tuple4X;
use crate::approx_eq::ApproxEq;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector(pub Tuple4X);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector((x, y, z, 0.0))
    }

    pub fn magnitude(&self) -> f64 {
        let (x, y, z, _) = self.0;

        f64::sqrt(x*x + y*y + z*z)
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        let (x,y,z,_) = self.0;
        Self::new(x/m, y/m, z/m)
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        let (x1, y1, z1, _) = self.0;
        let (x2, y2, z2, _) = rhs.0;

        x1 * x2 + y1 * y2 + z1 * z2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let (x1, y1, z1, _) = self.0;
        let (x2, y2, z2, _) = rhs.0;

        Vector::new(
            y1*z2 - z1*y2,
            z1*x2 - x1*z2,
            x1*y2 - y1*x2
        )
    }
}

impl ApproxEq for Vector {
    fn approx_eq(&self, other: &Self) -> bool {
        self.0.approx_eq(&other.0)
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector::new(f64::default(), f64::default(), f64::default())
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        let (x1, y1, z1, _) = self.0;
        let (x2, y2, z2, _) = rhs.0;
        Vector::new(x1+x2, y1+y2, z1+z2)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        let (x, y, z, _) = self.0;

        Vector::new(x*rhs, y * rhs, z* rhs)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        let (x,y,z,_) = self.0;

        Vector::new(x/rhs, y/rhs, z/rhs)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        let (x1, y1, z1, _) = self.0;
        let (x2, y2, z2, _) = rhs.0;
        Vector::new(x1-x2, y1-y2, z1-z2)
    }
}