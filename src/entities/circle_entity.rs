use crate::physics::*;
use crate::shapes::*;

use sdl2::pixels::Color;

pub struct CircleEntity {
    pub shape: Circle,
    pub physics_properties: PhysicsProperties
}

impl CircleEntity {
    #[allow(dead_code)]
    fn new() -> Self {
        CircleEntity::default()
    } 
   
    pub fn create_circle_entity(x: f64, y: f64, radius: f64, velocity_magnitude: f64, color: Color) -> Self {
        Self {
            shape: Circle::create_circle(x, y, radius, color),
            physics_properties: PhysicsProperties { velocity_magnitude: velocity_magnitude, ..PhysicsProperties::default() }
        }
    }

    pub fn move_circle(&mut self, delta_time: f64) {
        self.shape.move_shape(self.physics_properties.velocity() * delta_time);
    }
}

impl Default for CircleEntity {
    fn default() -> Self {
        Self {
            shape: Circle::default(),
            physics_properties: PhysicsProperties::default()
        }
    }
}
