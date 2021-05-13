mod algebra;
mod entities;
mod game;
mod game_settings;
mod physics;
mod shapes;
mod systems;

fn main() {
    let mut game = game::Game::new();
    game.run();
}