use crate::algebra::Vec2f;

pub struct PhysicsProperties {
    pub direction: Vec2f,
    pub velocity_magnitude: f64
}

impl PhysicsProperties {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PhysicsProperties::default() 
    }

    pub fn velocity(&self) -> Vec2f {
        self.direction * self.velocity_magnitude
    }
}

impl Default for PhysicsProperties {
    fn default() -> Self {  
        Self {
            direction: Vec2f::new(), 
            velocity_magnitude: 0.0 
        } 
    }
}