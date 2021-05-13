pub struct TimeSystem {
    pub fixed_delta_time: f64,
    pub game_frame_duration: f64,

    frame_duration: f64,
    last_time: f64,
    scale: f64,
    timer_subsystem: sdl2::TimerSubsystem,
}

impl TimeSystem {
    pub fn new(timer_subsystem: sdl2::TimerSubsystem) -> Self {
        let last_time = Self::get_current_time(&timer_subsystem);

        Self {
            fixed_delta_time: 1.0 / 120.0,
            game_frame_duration: 0.0,
            frame_duration: 0.0,
            last_time: last_time,
            scale: 1.0,
            timer_subsystem
        }
    }

    pub fn update_frame(&mut self) {
        let current_time = Self::get_current_time(&self.timer_subsystem);
        self.frame_duration = current_time - self.last_time;
        self.last_time = current_time;
        self.game_frame_duration = self.frame_duration * self.scale;
    }

    fn get_current_time(timer_subsystem: &sdl2::TimerSubsystem) -> f64 {
        let counter = timer_subsystem.performance_counter() as f64;
        let frequency = timer_subsystem.performance_frequency() as f64;

        counter / frequency
    }
}
