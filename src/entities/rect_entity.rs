use crate::physics::*;
use crate::shapes::*;

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
    pub fn create_rect_entity(x: f64, y: f64, height: f64, width: f64, max_velocity: f64) -> Self {
        Self {
            shape: Rect::create_rect(x, y, width, height),
            physics_properties: PhysicsProperties { max_velocity: max_velocity, ..PhysicsProperties::default() }
        }
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