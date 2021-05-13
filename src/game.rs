use crate::algebra::Vec2f;

use crate::entities;

use crate::physics;

use crate::game_settings;

use crate::systems::EventPump;
use crate::systems::TimeSystem;
use crate::systems::RenderSystem;

use rand::Rng;

use sdl2::pixels::Color;

use std::vec::Vec;

const RECT_HEIGHT: f64 = 40.0;

const RIGHT: Vec2f = Vec2f { x: 1.0, y: 0.0 };
const LEFT: Vec2f = Vec2f { x: -1.0, y: 0.0 };
const STAY: Vec2f = Vec2f { x: 0.0, y: 0.0 };

pub struct Game {
    event_pump: EventPump,
    time_system: TimeSystem,
    render_system: RenderSystem,
    rect_entities: Vec<entities::RectEntity>,
    circle_entities: Vec<entities::CircleEntity>,
}

impl Game {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();

        let event_pump = EventPump::new(sdl_context.event_pump().unwrap());
        let time_system = TimeSystem::new(sdl_context.timer().unwrap());
        let render_system = RenderSystem::new(sdl_context.video().unwrap());
        
        let player = entities::RectEntity::create_rect_entity(
            game_settings::SCREEN_WIDTH as f64 * 0.5, 
            game_settings::SCREEN_HEIGHT as f64 * 0.9, 
            game_settings::PLAYER_HEIGHT, 
            game_settings::PLAYER_WIDTH, 
            game_settings::PLAYER_VELOCITY, 
            Color::BLACK);

        let mut rect_entities = vec![player];
        
        rect_entities.append(&mut Game::rects_generator(6u8, 7u8));
        
        let mut game_ball = entities::CircleEntity::create_circle_entity(
            game_settings::SCREEN_WIDTH as f64 * 0.5,
            game_settings::SCREEN_HEIGHT as f64 * 0.5, 
            game_settings::BALL_RADIUS, 
            game_settings::BALL_VELOCITY, 
            Color::CYAN
        );

        let ball_initial_direction = || -> Vec2f {
            let pi = std::f64::consts::PI;
            let random_angle = rand::thread_rng().gen_range(0.0..(2.0 * pi)); 

            Vec2f { x: random_angle.sin(), y: random_angle.cos() }
        }();
        
        game_ball.physics_properties.direction = ball_initial_direction;
        
        let circle_entities = vec![game_ball];
        
        Self {
            event_pump,
            time_system,
            render_system,
            rect_entities,
            circle_entities
        }
    }

    fn rects_generator(num_of_rect_lines: u8, max_num_of_rect_per_line: u8) -> Vec<entities::RectEntity> {
        let mut rect_entities = Vec::new();
        let mut random_generator = rand::thread_rng();

        let color_palette = vec![Color::BLUE, Color::GREEN, Color::RED, Color::YELLOW];

        let mut rect_x;
        let mut rect_y = 0.0;

        for _ in 0..num_of_rect_lines {
            
            let num_of_rects = random_generator.gen_range(3..=max_num_of_rect_per_line);           

            /* TODO: fix floating point errors causing visual glitches when rendering 
            let num_of_rects = || -> u8 {
                let mut num;
                loop {
                    num = random_generator.gen_range(3..=max_num_of_rect_per_line);

                    if game_settings::SCREEN_WIDTH % num as u32 == 0 {
                        break;
                    }
                }
                num
            }();
            */

            let rect_width = game_settings::SCREEN_WIDTH as f64 / num_of_rects as f64;

            rect_x = 0.0;

            for _ in 0..num_of_rects {
                let color_index = random_generator.gen_range(0..color_palette.len());
                let rect_color = color_palette[color_index];
                
                let rect_entity = entities::RectEntity::create_rect_entity(
                    rect_x, 
                    rect_y, 
                    RECT_HEIGHT, 
                    rect_width, 
                    0.0, 
                    rect_color
                );

                rect_entities.push(rect_entity);

                rect_x += rect_width;
            }

            rect_y += RECT_HEIGHT;
        }
        
        return rect_entities;
    }
    
    fn process_player_input(&mut self) {
        let player = self.rect_entities.first_mut().unwrap();
        
        let mut go_left = self.event_pump.go_left;
        let mut go_right = self.event_pump.go_right;
        
        if go_left && go_right {
            go_left = false;
            go_right = false;
        }        

        let find_direction = || -> Vec2f { 
            if go_left {
                LEFT
            } else if go_right {
                RIGHT
            } else {
                STAY
            }
        };

        player.physics_properties.direction = find_direction();
    }

    pub fn run(&mut self) {
        loop {
            self.render_system.render(&self.rect_entities, &self.circle_entities);
            
            self.event_pump.handle_input();
            if self.event_pump.close_game { break; }

            self.process_player_input();            

            self.time_system.update_frame();

            physics::update_game_frame(self.time_system.game_frame_duration, 
                                       &mut self.rect_entities, 
                                       &mut self.circle_entities);

            //println!("{:.10}", self.time_system.game_frame_duration);
        } 
    }
}