use crate::game_settings;

pub struct VideoSystem {
    window_height: u32,
    window_width: u32,
    video_subsystem: sdl2::VideoSubsystem
}

impl VideoSystem {
    pub fn new(video_subsystem: sdl2::VideoSubsystem) -> Self {
        Self {
            window_height: game_settings::SCREEN_HEIGHT,
            window_width: game_settings::SCREEN_WIDTH,
            video_subsystem
        }
    }
  
    pub fn create_window(&self) -> sdl2::video::Window {
        self.video_subsystem
        .window("block-game-rust", self.window_width, self.window_height)
        .position_centered()
        .build()
        .unwrap() 
    }

    pub fn create_canvas(&self) -> sdl2::render::Canvas<sdl2::video::Window> {
        self.create_window().into_canvas().build().unwrap()
    }
}