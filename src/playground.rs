use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::draw::{Position, draw_rectangle};


const PLAYGROUND_COLOR: Color = [0.5, 1.0, 0.5, 1.0];
const BORDER_COLOR: Color = [1.0, 1.0, 0.0, 1.0];
const BORDER_WIDTH: u32 = 2;


pub struct Playground {
    width: u32,
    height: u32,
    color: Color,
    border_color: Color,
    border_width: u32,
}


impl Playground {

    pub fn new(width: u32, height: u32) -> Playground {
        Playground::with_more_details(
            width, 
            height, 
            PLAYGROUND_COLOR, 
            BORDER_COLOR, 
            BORDER_WIDTH)
    }

    pub fn with_more_details(width: u32, height: u32, color: Color,
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

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        draw_rectangle(
            Position (0, 0), 
            self.width, 
            self.height, 
            self.border_color, 
            context, 
            graphics
        );
        draw_rectangle(
            Position (self.border_width, self.border_width), 
            self.width - self.border_width, 
            self.height - self.border_width, 
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

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }

    pub fn set_border_width(&mut self, width: u32) {
        self.border_width = width;
    }

}