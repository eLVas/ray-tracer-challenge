use std::ops;

pub type Tuple4X = (f64, f64, f64, f64);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point(pub Tuple4X);

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

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        let (vx1, vy1, vz1, _) = self.0;
        let (vx2, vy2, vz2, _) = rhs.0;
        Vector::new(vx1-vx2, vy1-vy2, vz1-vz2)
    }
}
