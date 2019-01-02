use std::collections::LinkedList;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use crate::playground::Playground;
use crate::draw::{Block, Shape, Position};


const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];


pub struct Snake {
    head: Block,
    body: LinkedList<Block>,
    prev_tail: Option<Block>,
    direction: Direction,
    color: Color,
    eatings: u32,
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
            prev_tail: None,
            direction: Direction::Right,
            color: SNAKE_COLOR,
            eatings: 0,
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
        self.prev_tail = self.body.pop_back();
    }

    pub fn eat(&mut self) {
        let tail = self.prev_tail.clone().unwrap();
        self.body.push_back(tail);
        self.prev_tail = None;
        self.eatings += 1;
    }

    pub fn bite_itself(&self) -> bool {
        for block in self.body.iter() {
            if self.head.get_position() == block.get_position() {
                return true;
            }
        }
        false
    }

    pub fn hit_walls_of(&self, playground: &Playground) -> bool {
        let width = playground.get_width();
        let height = playground.get_height();
        let wall_width = playground.get_border_width();
        let Position (column, row) = *self.get_head_position();

        let hit_top = (row == (wall_width - 1));
        let hit_left = (column == (wall_width - 1));
        let hit_bottom = (row == (height - wall_width));
        let hit_right = (column == (width - wall_width));

        hit_top || hit_bottom || hit_left || hit_right
    }

    pub fn on_position(&self, position: &Position) -> bool {
        for block in self.body.iter() {
            if position == block.get_position() {
                return true;
            }
        }
        self.get_head_position() == position
    }

    pub fn get_head_position(&self) -> &Position {
        self.head.get_position()
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