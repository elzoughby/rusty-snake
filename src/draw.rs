use piston_window::*;
use piston_window::types::Color;


const BLOCK_SIZE: f64 = 10.0;


#[derive(Clone)]
pub struct Block {
    position: Position,
    shape: Shape,
}

#[derive(Clone)]
pub enum Shape {
    Square,
    Circle,
    Triangle,
}

#[derive(Clone, PartialEq)]
pub struct Position (pub u32, pub u32);

#[derive(Clone, PartialEq)]
pub struct Coord (pub f64, pub f64);


impl Position {

    pub fn new(columns: u32, rows: u32) -> Position {
        Position (columns, rows)
    }

    pub fn to_coord(&self) -> Coord {
        let x = (self.0 as f64) * BLOCK_SIZE;
        let y = (self.1 as f64) * BLOCK_SIZE;
        Coord (x, y)
    }

    pub fn shifted_by(&self, columns: i32, rows: i32,) -> Position {
        Position ((self.0 as i32 + columns) as u32, (self.1 as i32 + rows) as u32)
    }

}


impl Coord {

    pub fn new(x:f64, y:f64) -> Coord {
        Coord (x, y)
    }

    pub fn to_position(&self) -> Position {
        let column = (self.0 / BLOCK_SIZE) as u32;
        let row = (self.1 / BLOCK_SIZE) as u32;
        Position (column, row)
    }

    pub fn as_array(&self) -> [f64; 2] {
        let Coord (x, y) = *self;
        [x, y]
    }

}


impl Block {

    pub fn new(position: Position, shape: Shape) -> Block {
        Block {
            position,
            shape,
        }
    }

    pub fn draw(&self, color: Color, context: &Context, graphics: &mut G2d) {
        let Coord (x, y) = self.position.to_coord();
        match self.shape {
            Shape::Square => rectangle(
                color, 
                [x, y, BLOCK_SIZE, BLOCK_SIZE], 
                context.transform,
                graphics),
            Shape::Circle => ellipse(
                color, 
                [x, y, BLOCK_SIZE, BLOCK_SIZE],
                context.transform,
                graphics),
            Shape::Triangle => polygon(
                color, 
                &[
                    self.position.shifted_by(1, 0).to_coord().as_array(), 
                    self.position.shifted_by(0, 1).to_coord().as_array(), 
                    self.position.shifted_by(1, 1).to_coord().as_array()
                ], 
                context.transform,
                graphics),
        }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }

}



pub fn draw_rectangle(position: Position, width: u32, height: u32, 
            color: Color, context: &Context, graphics: &mut G2d) {
    let Coord (x, y) = position.to_coord();
    rectangle(
        color, 
        [x, y, (width as f64 * BLOCK_SIZE), (height as f64 * BLOCK_SIZE)], 
        context.transform,
        graphics
    );
}

