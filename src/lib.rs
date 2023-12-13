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
        let (px1, py1, pz1, _) = self.0;
        let (px2, py2, pz2, _) = rhs.0;
        Vector::new(px1-px2, py1-py2, pz1-pz2)
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
        let (vx1, vy1, vz1, _) = self.0;
        let (vx2, vy2, vz2, _) = rhs.0;
        Vector::new(vx1+vx2, vy1+vy2, vz1+vz2)
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
        let (vx1, vy1, vz1, _) = self.0;
        let (vx2, vy2, vz2, _) = rhs.0;
        Vector::new(vx1-vx2, vy1-vy2, vz1-vz2)
    }
}
