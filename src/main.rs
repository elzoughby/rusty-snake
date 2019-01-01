extern crate rand;
extern crate piston_window;

mod game;
mod playground;
mod snake;
mod food;
mod draw;

use piston_window::*;


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rusty Snake", (360, 340))
        .exit_on_esc(true).build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([0.5, 1.0, 0.5, 1.0], graphics);
            rectangle([1.0, 1.0, 0.0, 1.0], // yellow
                      [50.0, 50.0, 50.0, 50.0],
                      context.transform,
                      graphics);
            ellipse([0.0, 1.0, 1.0, 1.0], 
                    [55.0, 55.0, 40.0, 40.0],
                    context.transform,
                    graphics);
        });
    }
}

