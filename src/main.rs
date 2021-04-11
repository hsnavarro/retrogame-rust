use sdl2::pixels::Color;

mod algebra;
mod entities;
mod game_settings;
mod physics;
mod shapes;
mod systems;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = systems::VideoSystem::new(sdl_context.video().unwrap());
    let mut time_system = systems::TimeSystem::new(sdl_context.timer().unwrap());
    let mut event_pump = systems::EventPump::new(sdl_context.event_pump().unwrap());

    let mut canvas = video_subsystem.create_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    //let mut ball = entities::CircleEntity::new();
    //let mut rectangle = entities::RectEntity::new();

    loop {
        event_pump.handle_input();
        if event_pump.close_game { break; }

        canvas.present();

        time_system.update_frame();
        println!("{:.10}", time_system.game_frame_duration);
    }
}
