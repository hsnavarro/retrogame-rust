use crate::algebra::Vec2f;
use crate::entities;
use crate::game_settings;
use crate::shapes;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::VideoSubsystem;
use sdl2::video::Window;

use std::vec::Vec;

pub struct RenderSystem {
    canvas: Canvas<Window>
}

impl RenderSystem {
    pub fn new(video_subsystem: VideoSubsystem) -> Self {
        Self {
            canvas: RenderSystem::create_canvas(game_settings::SCREEN_WIDTH, game_settings::SCREEN_HEIGHT, &video_subsystem)
        }
    }
  
    fn create_canvas(window_width: u32, window_height: u32, video_subsystem: &VideoSubsystem) -> Canvas<Window> {
        video_subsystem
        .window("block-game-rust", window_width, window_height)
        .position_centered()
        .build()
        .unwrap() 
        .into_canvas()
        .build()
        .unwrap()
    }

    fn draw_filled_rect_with_border(&mut self, rect_entity: &entities::RectEntity) {
        let rect_color = rect_entity.shape.color;
        let Vec2f { x: rect_x, y: rect_y } = rect_entity.shape.position();
       
        let rect_x = rect_x.round() as i32; 
        let rect_y = rect_y.round() as i32;
        let rect_width = rect_entity.shape.width().round() as u32;
        let rect_height = rect_entity.shape.height().round() as u32;

        let rect = Rect::new(rect_x, rect_y, rect_width, rect_height);
        
        self.canvas.set_draw_color(rect_color);
        self.canvas.fill_rect(rect).unwrap(); 

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.draw_rect(rect).unwrap(); 
    }

    fn draw_filled_circle_with_border(&mut self, circle_entity: &entities::CircleEntity) {
        let entities::CircleEntity { 
            shape: shapes::Circle { 
                center: Vec2f { x: x_center, y: y_center }, 
                radius: circle_radius, 
                color: circle_color, 
                .. 
            }, 
            .. 
        } = circle_entity;

        let x_center_i16 = x_center.round() as i16;
        let y_center_i16 = y_center.round() as i16;
        let circle_radius_i16 = circle_radius.round() as i16;

        self.canvas.filled_circle(x_center_i16, y_center_i16, circle_radius_i16, *circle_color).unwrap();
        self.canvas.circle(x_center_i16, y_center_i16, circle_radius_i16, Color::BLACK).unwrap();
    }

    fn draw_rect_entities(&mut self, rect_entities: &Vec<entities::RectEntity>) {
        for rect_entity in rect_entities.iter() {
            self.draw_filled_rect_with_border(rect_entity);
        }
    }
    
    fn draw_circle_entities(&mut self, circle_entities: &Vec<entities::CircleEntity>) {
        for circle_entity in circle_entities.iter() {
            self.draw_filled_circle_with_border(circle_entity);
        }
    }

    pub fn render(&mut self, rect_entities: &Vec<entities::RectEntity>, circle_entities: &Vec<entities::CircleEntity>) {
        self.canvas.set_draw_color(Color::GREY);
        self.canvas.clear();
        
        self.draw_rect_entities(rect_entities);
        self.draw_circle_entities(circle_entities);

        self.canvas.present();
    }
}