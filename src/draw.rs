use std::path::{Path, PathBuf};
use piston_window::*;
use piston_window::types::Color;
use find_folder::Search;


const BLOCK_SIZE: f64 = 12.0;
const WHITE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];


pub struct Block {
    position: Position,
    shape: Shape,
}

#[allow(dead_code)]
pub enum Shape {
    Square,
    Circle,
    Triangle,
    Image(String),
}


#[derive(Clone, PartialEq)]
pub struct Position (pub u32, pub u32);

#[derive(PartialEq)]
pub struct Coord (pub f64, pub f64);

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl Direction {

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

}


impl Position {

    pub fn new(columns: u32, rows: u32) -> Position {
        Position (columns, rows)
    }

    pub fn to_coord(&self) -> Coord {
        let x = f64::from(self.0) * BLOCK_SIZE;
        let y = f64::from(self.1) * BLOCK_SIZE;
        Coord (x, y)
    }

    pub fn shifted_by(&self, columns: i32, rows: i32,) -> Position {
        Position::new(
            (self.0 as i32 + columns) as u32, 
            (self.1 as i32 + rows) as u32)
    }

}


impl Coord {

    #[allow(dead_code)]
    pub fn new(x:f64, y:f64) -> Coord {
        Coord (x, y)
    }

    #[allow(dead_code)]
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

    pub fn draw(&self, color: Color, factory: &mut GfxFactory, 
                    context: &Context, graphics: &mut G2d) {
        let Coord (x, y) = self.position.to_coord();
        match &self.shape {
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
            Shape::Image(path) => {
                let image   = Image::new().rect([x, y, BLOCK_SIZE, BLOCK_SIZE]);
                let texture = Texture::from_path(factory, find_resource(path),
                        Flip::None, &TextureSettings::new()).unwrap();
                image.draw(&texture, &DrawState::default(), context.transform, graphics);
            },
        }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    #[allow(dead_code)]
    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    #[allow(dead_code)]
    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }

}



pub fn draw_rectangle(position: &Position, width: u32, height: u32, 
            color: Color, context: &Context, graphics: &mut G2d) {
    let Coord (x, y) = position.to_coord();
    rectangle(
        color, 
        [x, y, (f64::from(width) * BLOCK_SIZE), 
            (f64::from(height) * BLOCK_SIZE)], 
        context.transform,
        graphics
    );
}

pub fn draw_text(text: &str, position: &Position, color: Color, size: u32,
            factory: GfxFactory, context: &Context, graphics: &mut G2d) {
    let Coord (x, y) = position.to_coord();
    let font = find_resource("ExoExtraBold.ttf");
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    piston_window::text(
        color,
        size,
        text,
        &mut glyphs,
        context.transform.trans(x, y),
        graphics).unwrap();
}

pub fn draw_eyes(head: &Block, direction: &Direction, 
            context: &Context, graphics: &mut G2d) {
    let Coord (x, y) = head.position.to_coord();
    let fifth = BLOCK_SIZE/5.0;
    let (eye1_x, eye1_y, eye2_x, eye2_y) = match direction {
        Direction::Up =>
            (x+fifth, y+fifth, x+fifth*3.0, y+fifth),
        Direction::Down =>
            (x+fifth, y+fifth*3.0, x+fifth*3.0, y+fifth*3.0),
        Direction::Left =>
            (x+fifth, y+fifth, x+fifth, y+fifth*3.0),
        Direction::Right =>
            (x+fifth*3.0, y+fifth, x+fifth*3.0, y+fifth*3.0),
    };
    ellipse(WHITE_COLOR, 
            [eye1_x, eye1_y, fifth, fifth],
            context.transform,
            graphics);
    ellipse(WHITE_COLOR, 
            [eye2_x, eye2_y, fifth, fifth],
            context.transform,
            graphics);
}



fn find_resource<P: AsRef<Path>>(asset: P) -> PathBuf {
    let mut exe_folder = std::env::current_exe()
            .expect("Couldn't capture the executable path");
    exe_folder.pop(); // Remove the executable name
    let resource_path = Search::KidsThenParents(2, 2)
            .of(exe_folder).for_folder("resources")
            .expect("Couldn't find the resources folder");
    resource_path.join(asset)
}

