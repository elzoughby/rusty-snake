use std::collections::LinkedList;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use crate::draw::{Block, Shape, Position};


const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];


pub struct Snake {
    head: Block,
    body: LinkedList<Block>,
    direction: Direction,
    color: Color,
}

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


impl Snake {

    pub fn new(position: Position) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block::new(
            position.shifted_by(3, 0),
            Shape::Square,
        ));

        body.push_back(Block::new(
            position.shifted_by(2, 0),
            Shape::Square,
        ));

        Snake {
            head: Block::new( 
                position.shifted_by(4, 0),
                Shape::Circle,
            ),
            body: body,
            direction: Direction::Right,
            color: SNAKE_COLOR,
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        self.head.draw(SNAKE_COLOR, context, graphics);
        for block in self.body.iter() {
            block.draw(SNAKE_COLOR, context, graphics);
        }
    }

    pub fn step(&mut self, dir: Option<Direction>) {
        if let Some(direction) = dir {
            self.direction = direction;
        }

        let last_pos = self.head.get_position();
        let new_block = Block::new(
            Position (last_pos.0, last_pos.1), 
            Shape::Square);
        let new_position = match self.direction {
            Direction::Up => last_pos.shifted_by(0, -1),
            Direction::Down => last_pos.shifted_by(0, 1),
            Direction::Left => last_pos.shifted_by(-1, 0),
            Direction::Right => last_pos.shifted_by(1, 0),
        };
        self.head.set_position(new_position);
        self.body.push_front(new_block);
        self.body.pop_back();
    }

    pub fn bite_itself(&self) -> bool {
        for block in self.body.iter() {
            if self.head.get_position() == block.get_position() {
                return true;
            }
        }
        false
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

}