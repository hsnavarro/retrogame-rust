use crate::algebra::*;

pub struct Rect {
    pub top_left: Vec2f,
    pub top_right: Vec2f,
    pub bottom_left: Vec2f, 
    pub bottom_right: Vec2f,
}

impl Rect {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Rect::default()
    }
    
    pub fn create_rect(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            top_left: Vec2f { x: x, y: y + height },
            top_right: Vec2f { x: x + width, y: y + height},
            bottom_left: Vec2f { x: x, y: y },
            bottom_right: Vec2f { x: x + width, y: y }
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect::create_rect(0.0, 0.0, 1.0, 1.0)
    }
}