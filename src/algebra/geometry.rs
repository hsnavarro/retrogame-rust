use crate::algebra::{Vec2f, cross_product, distance, dot_product};
use crate::shapes;

pub fn is_point_inside_rect(rect: &shapes::Rect, point: Vec2f) -> bool {
    let rect_points = rect.get_points_clockwise();

    for i in 0..rect_points.len() {
        let j = (i + 1) % rect_points.len();
    
        if cross_product(rect_points[j] - rect_points[i], point - rect_points[i]) < 0.0 {
            return false;
        }
    }

    true
}

pub fn closest_to_point_in_rect_border(rect: &shapes::Rect, point: Vec2f) -> Vec2f {
    
    let mut min_distance = f64::MAX;
    let mut closest_point = Vec2f::new();

    let rect_points = rect.get_points_clockwise();

    let mut update_closest_point = |rect_point: Vec2f| {
        let distance = distance(rect_point, point);

        if distance < min_distance {
            min_distance = distance;
            closest_point = rect_point;
        }
    };

    // rect points
    for rect_point in rect_points.iter() { update_closest_point(*rect_point); }

    // rect sides
    for i in 0..rect_points.len() {
        let j = (i + 1) % rect_points.len();

        let rect_side = rect_points[j] - rect_points[i];

        let projected_vector = (point - rect_points[i]).projection(rect_side);

        if projected_vector.magnitude() > rect_side.magnitude() || 
            dot_product(projected_vector, rect_side) < 0.0 { continue; }

        let projected_point = rect_points[i] + projected_vector;
        update_closest_point(projected_point);
    }

    closest_point
}

#[cfg(test)]
mod tests {
    use super::*;  
   
    use crate::algebra::Vec2f; 
    use crate::shapes::Rect;
    
    use sdl2::pixels::Color;

    mod is_point_inside_rect_tests {
        use super::*; 
        
        #[test]
        fn point_outside_rect() {
            let rect = Rect::create_rect(2.0, 10.0, 10.0, 20.0, Color::BLACK);
            let point = Vec2f { x: 40.0, y: 40.0 };  

            assert!(is_point_inside_rect(&rect, point) == false);
        }
    
        #[test]
        fn point_inside_rect() {
            let rect = Rect::create_rect(2.0, 10.0, 10.0, 20.0, Color::BLACK);
            let point = Vec2f { x: 10.0, y: 20.0 };  

            assert!(is_point_inside_rect(&rect, point) == true);
        }
   
        #[test]
        fn point_in_rect_border() {
            let rect = Rect::create_rect(2.0, 10.0, 10.0, 20.0, Color::BLACK);
            let point = Vec2f { x: 2.0, y: 15.0 };  

            assert!(is_point_inside_rect(&rect, point) == true);
        }
    }
    
    mod closest_to_point_in_rect_border_tests {
        use super::*; 
        
        #[test]
        fn closest_is_corner() {
            let rect = Rect::create_rect(2.0, 10.0, 20.0, 10.0, Color::BLACK);
            let point = Vec2f { x: 40.0, y: 40.0 };  

            let ans = Vec2f { x: 22.0, y: 20.0 };

            assert_eq!(closest_to_point_in_rect_border(&rect, point), ans);
        }
    
        #[test]
        fn closest_is_side() {
            let rect = Rect::create_rect(2.0, 10.0, 20.0, 10.0, Color::BLACK);
            let point = Vec2f { x: 6.0, y: 4.0 };  

            let ans = Vec2f { x: 6.0, y: 10.0 };
            
            assert_eq!(closest_to_point_in_rect_border(&rect, point), ans);
        }
   
        #[test]
        fn point_inside_rect() {
            let rect = Rect::create_rect(2.0, 10.0, 20.0, 10.0, Color::BLACK);
            let point = Vec2f { x: 10.0, y: 13.0 };  

            let ans = Vec2f { x: 10.0, y: 10.0 };
            
            assert_eq!(closest_to_point_in_rect_border(&rect, point), ans);
        }
        
        #[test]
        fn point_in_rect_border() {
            let rect = Rect::create_rect(2.0, 10.0, 20.0, 10.0, Color::BLACK);
            
            let point = Vec2f { x: 12.0, y: 10.0 };  

            let ans = Vec2f { x: 12.0, y: 10.0 };
            
            assert_eq!(closest_to_point_in_rect_border(&rect, point), ans);
        }
    }
}