use piston_window::{GfxFactory, Context, G2d};
use piston_window::types::Color;
use crate::draw::{Position, draw_rectangle};


const PLAYGROUND_WIDTH: u32 = 35;
const PLAYGROUND_HEIGHT: u32 = 25;
const PLAYGROUND_COLOR: Color = [0.66, 0.78, 0.22, 1.0];
const BORDER_COLOR: Color = [0.80, 0.30, 0.30, 1.0];
const BORDER_WIDTH: u32 = 1;


pub struct Playground {
    width: u32,
    height: u32,
    color: Color,
    border_color: Color,
    border_width: u32,
}


impl Default for Playground {

    fn default() -> Playground {
        Playground::new(
            PLAYGROUND_WIDTH, 
            PLAYGROUND_HEIGHT, 
            PLAYGROUND_COLOR, 
            BORDER_COLOR,
            BORDER_WIDTH)
    }

}


impl Playground {

    pub fn new(width: u32, height: u32, color: Color,
            border_color: Color, border_width: u32)
            -> Playground {
        Playground {
            width,
            height,
            color,
            border_color,
            border_width,
        }
    }

    pub fn draw(&self, _factory: &mut GfxFactory,
            context: &Context, graphics: &mut G2d) {
        draw_rectangle(
            &Position (0, 0), 
            self.width, 
            self.height, 
            self.border_color, 
            context, 
            graphics
        );
        draw_rectangle(
            &Position (self.border_width, self.border_width), 
            self.width - (self.border_width * 2), 
            self.height - (self.border_width * 2), 
            self.color, 
            context, 
            graphics
        );
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_border_width(&self) -> u32 {
        self.border_width
    }

}