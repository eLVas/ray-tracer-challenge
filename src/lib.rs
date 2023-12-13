use std::{f64, ops};
pub type Tuple4X = (f64, f64, f64, f64);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point(pub Tuple4X);

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

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point((x,y,z,1.0))
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(f64::default(), f64::default(), f64::default())
    }
}

impl ApproxEq for Point {
    fn approx_eq(&self, other: &Self) -> bool {
        self.0.approx_eq(&other.0)
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        let (px, py, pz, _) = self.0;
        let (vx, vy, vz, _) = rhs.0;
        Point::new(px+vx, py+vy, pz+vz)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        let (x1, y1, z1, _) = self.0;
        let (x2, y2, z2, _) = rhs.0;
        Vector::new(x1-x2, y1-y2, z1-z2)
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        let (px, py, pz, _) = self.0;
        let (vx, vy, vz, _) = rhs.0;
        Point::new(px-vx, py-vy, pz-vz)
    }
}

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
