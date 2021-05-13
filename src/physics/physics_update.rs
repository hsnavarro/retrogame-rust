use crate::algebra::Vec2f;
use crate::algebra::{closest_to_point_in_rect_border, is_point_inside_rect};
use crate::entities;
use crate::game_settings;

use std::vec::Vec;

fn detect_circle_rect_collision(circle_entity: &entities::CircleEntity, 
                                rect_entity: &entities::RectEntity) -> Option<Vec2f> {
    
    let circle_center = circle_entity.shape.center;
    let circle_radius = circle_entity.shape.radius;
    
    let closest_point = closest_to_point_in_rect_border(&rect_entity.shape, circle_center);
    let is_center_inside_rect = is_point_inside_rect(&rect_entity.shape, circle_center);
                                    
    let circle_center_penetration = (circle_center - closest_point).magnitude();
    let mut collision_normal = (circle_center - closest_point).norm();
    let mut circle_penetration = circle_radius - circle_center_penetration;
    
    if is_center_inside_rect { 
        collision_normal *= -1.0; 
        circle_penetration = circle_radius + circle_radius;
    } 

    if circle_penetration <= 0.0 { return None; }

    Some(collision_normal * circle_penetration)
}

fn treat_circle_rect_collision(circle_entity: &mut entities::CircleEntity, 
                               rect_entity:&entities::RectEntity) -> bool {
    
    match detect_circle_rect_collision(circle_entity, rect_entity) {
        Some(penetration_vector) => {
            circle_entity.shape.move_shape(penetration_vector);
            
            let collision_normal = penetration_vector.norm();            
            let old_direction = circle_entity.physics_properties.direction;            

            let perpendicular = old_direction.perpendicular(collision_normal);
            let parallel = old_direction.projection(collision_normal);

            let new_direction = perpendicular - parallel; 

            circle_entity.physics_properties.direction = new_direction;
            
            true
        }
        None => false
    }
}

enum ScreenCollisionType { HORIZONTAL, VERTICAL }

fn detect_screen_circle_collision(entity: &entities::CircleEntity) -> Option<ScreenCollisionType> {
    let entity_center = &entity.shape.center;
    let entity_radius = entity.shape.radius;

    let width_limit = game_settings::SCREEN_WIDTH as f64 - entity_radius;
    let height_limit = game_settings::SCREEN_HEIGHT as f64 - entity_radius;

    if entity_center.x < entity_radius || entity_center.x >  width_limit {
        return Some(ScreenCollisionType::HORIZONTAL);
    }
    
    if entity_center.y < entity_radius || entity_center.y > height_limit {
        return Some(ScreenCollisionType::VERTICAL);
    }

    None
}

fn treat_screen_circle_collision(entity: &mut entities::CircleEntity) {
    let collision_detection = detect_screen_circle_collision(entity);
    
    let entity_direction = &mut entity.physics_properties.direction;

    match collision_detection {
        Some(ScreenCollisionType::HORIZONTAL) => {
            *entity_direction = Vec2f { x: -entity_direction.x, y: entity_direction.y };
        }, 
        Some(ScreenCollisionType::VERTICAL) => {
            *entity_direction = Vec2f { x: entity_direction.x, y: -entity_direction.y };
        }, 
        None => {} 
    }
}

fn block_rect(entity: &mut entities::RectEntity) {
    let width_limit = game_settings::SCREEN_WIDTH as f64 - entity.shape.width();
    let height_limit = game_settings::SCREEN_HEIGHT as f64 - entity.shape.height();

    let clamp = |x: &mut f64, min_value: f64, max_value: f64| {
        if *x < min_value { *x = min_value; }
        if *x > max_value { *x = max_value; }
    };

    let Vec2f { x: mut new_x, y: mut new_y } = entity.shape.position();
    
    clamp(&mut new_x, 0.0, width_limit);
    clamp(&mut new_y, 0.0, height_limit);

    entity.shape.set_position(Vec2f { x: new_x, y: new_y });
}

fn delete_rect_entities(indexes_to_delete: &Vec<usize>, 
                        rect_entities: &mut Vec<entities::RectEntity>) {
    
    let num_of_deletions = indexes_to_delete.len();
    
    if num_of_deletions == 0 { return; }

    let num_of_rects = rect_entities.len();

    assert!(num_of_deletions <= num_of_rects);
    
    let mut last_index = num_of_rects - 1;
    
    for i in indexes_to_delete.iter() {
        rect_entities.swap(*i, last_index);
        last_index -= 1;
    }

    rect_entities.truncate(num_of_rects - num_of_deletions);
}

fn update_simulation_frame(delta_time: f64, 
                           rect_entities: &mut Vec<entities::RectEntity>, 
                           circle_entities: &mut Vec<entities::CircleEntity>) {
    
    for rect_entity in rect_entities.iter_mut() {
        rect_entity.move_rect(delta_time);
        block_rect(rect_entity);
    }
    
    for circle_entity in circle_entities.iter_mut() {
        circle_entity.move_circle(delta_time);
        treat_screen_circle_collision(circle_entity);
    }

    let mut indexes_to_delete = Vec::new();

    for circle_entity in circle_entities.iter_mut() {
        for (i, rect_entity) in rect_entities.iter_mut().enumerate() {
            if treat_circle_rect_collision(circle_entity, rect_entity) && i != 0 {
                indexes_to_delete.push(i);
            }
        }
    }

    delete_rect_entities(&indexes_to_delete, rect_entities);
}

pub fn update_game_frame(frame_time: f64, 
                         rect_entities: &mut Vec<entities::RectEntity>, 
                         circle_entities: &mut Vec<entities::CircleEntity>) {
    
    let mut delta_time_left = frame_time; 
    
    let minimum = |x: f64, y: f64| if x < y { x } else { y };

    while delta_time_left > 0.0 {
        let delta_time = minimum(delta_time_left, game_settings::FIXED_DELTA_TIME);
    
        update_simulation_frame(delta_time, rect_entities, circle_entities);

        delta_time_left -= delta_time;
    }
}

