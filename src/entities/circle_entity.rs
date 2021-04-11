use crate::physics::*;
use crate::shapes::*;

pub struct CircleEntity {
    pub shape: Circle,
    pub physics_properties: PhysicsProperties
}

impl CircleEntity {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CircleEntity::default()
    } 
   
    #[allow(dead_code)]
    pub fn create_circle_entity(&self, x: f64, y: f64, radius: f64, max_velocity: f64) -> Self {
        Self {
            shape: Circle::create_circle(x, y, radius),
            physics_properties: PhysicsProperties { max_velocity: max_velocity, ..PhysicsProperties::default() }
        }
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
