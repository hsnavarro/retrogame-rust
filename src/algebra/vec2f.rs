pub struct Vec2f {
    pub x: f64, 
    pub y: f64
}

impl Vec2f {
    pub fn new() -> Self {
        Vec2f::default()
    }
}

impl Default for Vec2f {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}