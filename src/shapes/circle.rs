use crate::algebra::*;

use sdl2::pixels::Color;

pub struct Circle {
    pub center: Vec2f,
    pub radius: f64,
    pub color: Color
}

impl Circle {
    #[allow(dead_code)] 
    pub fn new() -> Self {
        Circle::default()
    }

    pub fn create_circle(x: f64, y: f64, radius: f64, color: Color) -> Self {
        Self {
            center: Vec2f { x: x, y: y },
            radius,
            color
        }
    }    
    
    #[allow(dead_code)] 
    pub fn set_position(&mut self, new_position: Vec2f) {
        self.center = new_position;
    }

    pub fn move_shape(&mut self, position_delta: Vec2f) {
        self.center += position_delta;
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Vec2f::new(),
            radius: 1.0,
            color: Color::BLACK
        }
    }
}