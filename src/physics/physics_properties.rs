use crate::algebra::Vec2f;

pub struct PhysicsProperties {
    pub velocity: Vec2f,
    pub fixed_x_axis: bool,
    pub fixed_y_axis: bool,
    pub max_velocity: f64
}

impl PhysicsProperties {
    #[allow(dead_code)]
    fn new() -> Self {
        PhysicsProperties::default() 
    }
}

impl Default for PhysicsProperties {
    fn default() -> Self {  
        Self {
            velocity: Vec2f::new(), 
            fixed_x_axis: true, 
            fixed_y_axis: true, 
            max_velocity: 0.0 
        } 
    }
}