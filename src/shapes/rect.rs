use crate::algebra::*;

use sdl2::pixels::Color;

pub struct Rect {
    top_left: Vec2f, 
    top_right: Vec2f, 
    bottom_left: Vec2f, 
    bottom_right: Vec2f, 
    width: f64,
    height: f64,
    pub color: Color
}

impl Rect {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Rect::default()
    }

    pub fn position(&self) -> Vec2f {
        self.top_left
    }

    pub fn width(&self) -> f64 {
        self.width
    }
    
    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_position(&mut self, new_position: Vec2f) {
        let position_delta = new_position - self.top_left;
        self.move_shape(position_delta);
    }

    pub fn move_shape(&mut self, position_delta: Vec2f) {
        self.top_left += position_delta;
        self.top_right += position_delta;
        self.bottom_left += position_delta;
        self.bottom_right += position_delta;
    }
    
    pub fn get_points_clockwise(&self) -> Vec<Vec2f> {
        vec![self.bottom_left, self.top_left, self.top_right, self.bottom_right]
    }
    
    pub fn create_rect(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            top_left: Vec2f { x: x, y: y },
            top_right: Vec2f { x: x + width, y: y },
            bottom_left: Vec2f { x: x, y: y + height },
            bottom_right: Vec2f { x: x + width, y: y + height },
            width,
            height,
            color 
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect::create_rect(0.0, 0.0, 1.0, 1.0, Color::BLACK)
    }
}