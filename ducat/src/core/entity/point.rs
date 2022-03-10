use num::{Num, NumCast};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point::<T> { x, y }
    }
}

impl<T: Num> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Point<T>) -> Point<T> {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Num + Copy> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Num> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Num + Copy> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: NumCast> Point<T> {
    pub fn to_f64(&self) -> Point<f64> {
        Point::new(self.x.to_f64().unwrap(), self.y.to_f64().unwrap())
    }

    pub fn to_i32(&self) -> Point<i32> {
        Point::new(self.x.to_i32().unwrap(), self.y.to_i32().unwrap())
    }
}

pub fn distance<T: NumCast>(p: Point<T>, q: Point<T>) -> f64 {
    distance_sq(p, q).sqrt()
}

pub fn distance_sq<T: NumCast>(p: Point<T>, q: Point<T>) -> f64 {
    let p = p.to_f64();
    let q = q.to_f64();
    (p.x - q.x).powf(2.0) + (p.y - q.y).powf(2.0)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Rotation {
    sin_theta: f64,
    cos_theta: f64,
}

impl Rotation {
    #[allow(dead_code)]
    pub fn new(theta: f64) -> Rotation {
        let (sin_theta, cos_theta) = theta.sin_cos();
        Rotation {
            sin_theta,
            cos_theta,
        }
    }
}

impl Point<f64> {
    #[allow(dead_code)]
    pub(crate) fn rotate(&self, rotation: Rotation) -> Point<f64> {
        let x = self.x * rotation.cos_theta + self.y * rotation.sin_theta;
        let y = self.y * rotation.cos_theta - self.x * rotation.sin_theta;
        Point::new(x, y)
    }

    #[allow(dead_code)]
    pub(crate) fn invert_rotation(&self, rotation: Rotation) -> Point<f64> {
        let x = self.x * rotation.cos_theta - self.y * rotation.sin_theta;
        let y = self.y * rotation.cos_theta + self.x * rotation.sin_theta;
        Point::new(x, y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Line {
    a: f64,
    b: f64,
    c: f64,
}

impl Line {
    #[allow(dead_code)]
    pub fn from_points(p: Point<f64>, q: Point<f64>) -> Line {
        let a = p.y - q.y;
        let b = q.x - p.x;
        let c = p.x * q.y - q.x * p.y;
        Line { a, b, c }
    }

    #[allow(dead_code)]
    pub fn distance_from_point(&self, point: Point<f64>) -> f64 {
        let Line { a, b, c } = self;
        (a * point.x + b * point.y + c).abs() / (a.powf(2.0) + b.powf(2.)).sqrt()
    }
}
