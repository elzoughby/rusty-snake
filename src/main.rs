mod game;
mod playground;
mod snake;
mod food;
mod draw;

use piston_window::*;
use piston_window::types::Color;
use crate::game::Game;


const WHITE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];


fn main() {
    let columns = 35;
    let rows = 25;
    let mut game = Game::new(columns, rows);
    let size = game.get_size();
    let mut window: PistonWindow = 
        WindowSettings::new("Rusty Snake", size)
        .exit_on_esc(true).build()
        .unwrap_or_else(|e| {
            panic!("Failed to build PistonWindow: {}", e)
        });
    while let Some(event) = window.next() {
        let mut factory = window.factory.clone();
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |context, graphics| {
            clear(WHITE_COLOR, graphics);
            game.draw(&mut factory, &context, graphics);
        });
        event.update(|arg| {
            game.update(arg.dt)
        });
    }
}
