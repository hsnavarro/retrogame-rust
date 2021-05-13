use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct EventPump {
    event_pump: sdl2::EventPump,
    pub close_game: bool, 
    pub go_left: bool,
    pub go_right: bool
}

impl EventPump {
    pub fn new(event_pump: sdl2::EventPump) -> Self {
        Self {
            event_pump,
            close_game: false,
            go_left: false,
            go_right: false
        }        
    }

    pub fn handle_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } 
                | Event::Quit { .. } => { 
                    self.close_game = true; 
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    self.go_left = true; 
                }, 
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    self.go_left = false; 
                }, 
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    self.go_right = true; 
                }, 
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    self.go_right = false; 
                }, 
                _ => {}
           } 
        }
    }
} 