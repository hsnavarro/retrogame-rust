use crate::algebra::*;

pub struct Circle {
    pub center: Vec2f,
    pub radius: f64
}

impl Circle {
    #[allow(dead_code)] 
    pub fn new() -> Self {
        Circle::default()
    }

    pub fn create_circle(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Vec2f { x: x, y: y },
            radius: radius
        }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Vec2f::new(),
            radius: 1.0
        }
    }
}