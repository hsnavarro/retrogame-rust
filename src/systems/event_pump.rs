use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct EventPump {
    event_pump: sdl2::EventPump,
    pub close_game: bool
}

impl EventPump {
    pub fn new(event_pump: sdl2::EventPump) -> Self {
        Self {
            event_pump,
            close_game: false
        }        
    }

    pub fn handle_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } 
                | Event::Quit { .. } => { self.close_game = true; },
                _ => {}
           } 
        }
    }
} 