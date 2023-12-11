pub type Tuple4X = (f64, f64, f64, f64);

#[derive(Debug)]
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

#[derive(Debug)]
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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
