use std::fmt::{Display, Formatter};
use std::ops;
use crate::Tuple4X;
use crate::approx_eq::ApproxEq;
use crate::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point(pub Tuple4X);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point((x,y,z,1.0))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x,y,z,_) = self.0;

        write!(f, "•  ({}, {}, {})", x, y, z)
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

impl ops::Sub for Point {
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