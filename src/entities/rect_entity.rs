use crate::physics::*;
use crate::shapes::*;

use sdl2::pixels::Color;

pub struct RectEntity {
    pub shape: Rect, 
    pub physics_properties: PhysicsProperties
}

impl RectEntity {
    #[allow(dead_code)] 
    pub fn new() -> Self {
        RectEntity::default()
    }
    
    #[allow(dead_code)] 
    pub fn create_rect_entity(x: f64, y: f64, height: f64, width: f64, velocity_magnitude: f64, color: Color) -> Self {
        Self {
            shape: Rect::create_rect(x, y, width, height, color),
            physics_properties: PhysicsProperties { velocity_magnitude: velocity_magnitude, ..PhysicsProperties::default() }
        }
    }

    pub fn move_rect(&mut self, delta_time: f64) {
        self.shape.move_shape(self.physics_properties.velocity() * delta_time);
    }
}

impl Default for RectEntity {
    fn default() -> Self {
        Self {
            shape: Rect::default(),
            physics_properties: PhysicsProperties::default()
        }
    }
}