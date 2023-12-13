use std::num::ParseFloatError;
use std::str::FromStr;
use cucumber::{given, then, when, World, Parameter};
use ray_tracer_challenge::{ApproxEq, Point, Tuple4X, Vector};

#[derive(Debug, Default, Parameter)]
#[param(regex = r"tuple\((.*)\)")]
struct Tuple4(Tuple4X);

impl FromStr for Tuple4 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<Vec<f64>, ParseFloatError> = s.split(',').map(str::trim).map(|x_str| x_str.parse::<f64>()).collect();

        if let Ok(v) = result {
            return Ok(Tuple4((v[0], v[1], v[2], v[3])))
        }

        Err(format!("Failed to parse \"{}\" as tuple", s))
    }
}

#[derive(Debug, Default, Parameter)]
#[param(regex = r"tuple\((.*)\)")]
struct Tuple3((f64, f64, f64));

impl FromStr for Tuple3 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<Vec<f64>, ParseFloatError> = s.split(',').map(str::trim).map(|x_str| x_str.parse::<f64>()).collect();

        if let Ok(v) = result {
            return Ok(Tuple3((v[0], v[1], v[2])))
        }

        Err(format!("Failed to parse \"{}\" as tuple", s))
    }
}

#[derive(Debug, Parameter)]
#[param(regex = "x|y|z|w")]
enum Axis {
    X,
    Y,
    Z,
    W,
}

impl FromStr for Axis {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            "w" => Self::W,
            invalid => return Err(format!("Invalid `Axis`: {invalid}")),
        })
    }
}

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    tuple: Tuple4X,
    point: Point,
    point_other: Point,
    vector: Vector,
    vector_other: Vector,
}

#[given(expr = "a ← {tuple4}")]
fn tuple4x(world: &mut TupleWorld, tuple: Tuple4) {
    world.tuple = tuple.0;
}

#[given(expr = "p ← point\\({float}, {float}, {float}\\)")]
fn point(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    world.point = Point::new(x, y, z);
}

#[given(expr = "p2 ← point\\({float}, {float}, {float}\\)")]
fn point_other(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    world.point_other = Point::new(x, y, z);
}

#[given(expr = "a ← point.tuple")]
fn point_tuple(world: &mut TupleWorld) {
    world.tuple = world.point.0;
}

#[given(expr = "v ← vector\\({float}, {float}, {float}\\)")]
fn vector(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    world.vector = Vector::new(x, y, z);
}

#[given(expr = "v2 ← vector\\({float}, {float}, {float}\\)")]
fn vector_other(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    world.vector_other = Vector::new(x, y, z);
}

#[given(expr = "a ← vector.tuple")]
fn vector_tuple(world: &mut TupleWorld) {
    world.tuple = world.vector.0;
}

#[when("v ← normalize(v)")]
fn normalize_v(world: &mut TupleWorld) { world.vector = world.vector.normalize()}

#[then(expr = "a.{axis} = {float}")]
fn axis_value(world: &mut TupleWorld, axis: Axis, val: f64) {
    match axis {
        Axis::X => {assert_eq!(world.tuple.0, val)}
        Axis::Y => {assert_eq!(world.tuple.1, val)}
        Axis::Z => {assert_eq!(world.tuple.2, val)}
        Axis::W => {assert_eq!(world.tuple.3, val)}
    }
}

#[then(regex = r"^a is (not |)a point$")]
fn is_point(world: &mut TupleWorld, state: String) {
    match state.as_str() {
        "not " => assert_ne!(world.tuple.3, 1.0),
        _ => assert_eq!(world.tuple.3, 1.0)
    }
}

#[then(regex = "^a is (not |)a vector$")]
fn is_vector(world: &mut TupleWorld, state: String) {
    match state.as_str() {
        "not " => assert_ne!(world.tuple.3, 0.0),
        _ => assert_eq!(world.tuple.3, 0.0),
    }
}

#[then(expr = "p = {tuple4}")]
fn point_tuple_val_is(world: &mut TupleWorld, tuple: Tuple4) {
    assert_eq!(world.point.0, tuple.0)
}

#[then(expr = "v = {tuple4}")]
fn vector_tuple_val_is(world: &mut TupleWorld, tuple: Tuple4) {
    assert_eq!(world.vector.0, tuple.0)
}

#[then(expr = "p + v = point\\({float}, {float}, {float}\\)")]
fn point_vector_sum_is(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    assert_eq!(world.point + world.vector, Point::new(x, y, z))
}

#[then(expr = "v + v2 = vector\\({float}, {float}, {float}\\)")]
fn vector_vector_sm_is(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    assert_eq!(world.vector + world.vector_other, Vector::new(x, y, z))
}

#[then(expr = "p - p2 = vector\\({float}, {float}, {float}\\)")]
fn point_point_sub_is(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    assert_eq!(world.point - world.point_other, Vector::new(x, y, z))
}

#[then(expr = "p - v = point\\({float}, {float}, {float}\\)")]
fn point_vector_sub_is(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    assert_eq!(world.point - world.vector, Point::new(x, y, z))
}

#[then(expr = "v - v2 = vector\\({float}, {float}, {float}\\)")]
fn vector_vector_sub_is(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    assert_eq!(world.vector - world.vector_other, Vector::new(x, y, z))
}

#[then(expr = "v * {float} = vector\\({float}, {float}, {float}\\)")]
fn vector_scalar_mul_is(world: &mut TupleWorld, k: f64, x: f64, y: f64, z: f64) {
    assert_eq!(world.vector * k, Vector::new(x, y, z))
}

#[then(expr = "v \\/ {float} = vector\\({float}, {float}, {float}\\)")]
fn vector_scalar_div_is(world: &mut TupleWorld, k: f64, x: f64, y: f64, z: f64) {
    assert_eq!(world.vector / k, Vector::new(x, y, z))
}

#[then(expr = "magnitude\\(v\\) = {float}")]
fn vector_magnitude_is_one(world: &mut TupleWorld, m: f64) {
    assert_eq!(world.vector.magnitude(), m)
}

#[then(expr = "magnitude\\(v\\) = √{float}")]
fn vector_magnitude_is_sqrt_14(world: &mut TupleWorld, m_sq: f64) {
    assert_eq!(world.vector.magnitude(), f64::sqrt(m_sq))
}

#[then(expr = "normalize\\(v\\) = vector\\({float}, {float}, {float}\\)")]
fn vector_normal_is(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    assert!(world.vector.normalize().approx_eq(&Vector::new(x, y, z)))
}

#[then(expr = "dot\\(v, v2\\) = {float}")]
fn vector_dot_product_is(world: &mut TupleWorld, expected: f64) {
    assert_eq!(world.vector.dot(&world.vector_other), expected)
}

#[then(expr = "cross\\(v, v2\\) = vector\\({float}, {float}, {float}\\)")]
fn vector_cross_product_is(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    assert_eq!(world.vector.cross(&world.vector_other), Vector::new(x,y,z))
}

#[then(expr = "cross\\(v2, v\\) = vector\\({float}, {float}, {float}\\)")]
fn vector_cross_product_reversed_is(world: &mut TupleWorld, x: f64, y: f64, z: f64) {
    assert_eq!(world.vector_other.cross(&world.vector), Vector::new(x,y,z))
}



fn main() {
    futures::executor::block_on(TupleWorld::run("tests/features/tuples"));
}