use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2f {
    pub x: f64, 
    pub y: f64
}

impl Vec2f {
    pub fn new() -> Self {
        Vec2f::default()
    }

    pub fn norm(self) -> Vec2f {
        let magnitude = self.magnitude();
        let _x = self.x; 
        let _y = self.y; 
        assert!(magnitude != 0.0);

        self / magnitude
    }

    pub fn magnitude(self) -> f64 {
        Vec2f::square_magnitude(self).sqrt()
    }
    
    pub fn square_magnitude(self) -> f64 {
        dot_product(self, self)
    }

    pub fn projection(self, rhs: Vec2f) -> Self {
        assert!(rhs.square_magnitude() != 0.0);
        
        (dot_product(self, rhs) * rhs) / rhs.square_magnitude() 
    }

    pub fn perpendicular(self, rhs: Vec2f) -> Self {
        self - self.projection(rhs)
    }
}

pub fn distance(x: Vec2f, y: Vec2f) -> f64 {
    (x - y).square_magnitude()
}

pub fn cross_product(lhs: Vec2f, rhs: Vec2f) -> f64 {
    lhs.x * rhs.y - lhs.y * rhs.x
}

pub fn dot_product(lhs: Vec2f, rhs: Vec2f) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y
}

impl Add for Vec2f {
    type Output = Self;

    fn add(self, rhs: Vec2f) -> Self::Output {
        Vec2f { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vec2f {
    fn add_assign(&mut self, rhs: Vec2f) {
        *self = Vec2f { x: self.x + rhs.x, y: self.y + rhs.y };
    }
}

impl Div<f64> for Vec2f {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        assert!(rhs != 0.0); 
        
        (1.0 / rhs) * self
    }
}

impl Div<Vec2f> for f64 {
    type Output = Vec2f;

    fn div(self, rhs: Vec2f) -> Self::Output {
        assert!(self != 0.0); 
        
        (1.0 / self) * rhs
    }
}

impl Mul<f64> for Vec2f {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2f { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vec2f> for f64 {
    type Output = Vec2f;

    fn mul(self, rhs: Vec2f) -> Self::Output {
        Vec2f { x: self * rhs.x, y: self * rhs.y }
    }
}

impl MulAssign<f64> for Vec2f {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec2f { x: self.x * rhs, y: self.y * rhs };
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Vec2f) -> Self::Output {
        Vec2f { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Default for Vec2f {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}