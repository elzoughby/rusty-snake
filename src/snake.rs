use std::collections::LinkedList;
use piston_window::types::Color;
use piston_window::{GfxFactory, Context, G2d};
use crate::playground::Playground;
use crate::draw::{Block, Shape, Position, Direction, draw_eyes};


const SNAKE_COLOR: Color = [0.19, 0.19, 0.18, 1.0];
const SNAKE_BODY_SHAPE: Shape = Shape::Square;
const SNAKE_HEAD_SHAPE: Shape = Shape::Circle;


pub struct Snake {
    head: Block,
    body: LinkedList<Block>,
    prev_tail: Position,
    direction: Direction,
    color: Color,
    eatings: u32,
}


impl Snake {

    pub fn new(position: &Position) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block::new(
            position.shifted_by(1, 0),
            SNAKE_BODY_SHAPE,
        ));

        body.push_back(Block::new(
            position.shifted_by(0, 0),
            SNAKE_BODY_SHAPE,
        ));

        Snake {
            head: Block::new( 
                position.shifted_by(2, 0),
                SNAKE_HEAD_SHAPE,
            ),
            body: body,
            prev_tail: position.shifted_by(1, 0),
            direction: Direction::Right,
            color: SNAKE_COLOR,
            eatings: 0,
        }
    }

    pub fn draw(&self, factory: &mut GfxFactory, context: &Context, 
                graphics: &mut G2d) {
        for block in self.body.iter() {
            block.draw(SNAKE_COLOR, factory, context, graphics);
        }
        self.head.draw(SNAKE_COLOR, factory, context, graphics);
        draw_eyes(&self.head, &self.direction, context, graphics);
    }

    pub fn step(&mut self, dir: Option<Direction>) {
        if let Some(direction) = dir {
            self.direction = direction;
        }

        let last_head_pos = self.head.get_position().clone();
        let new_head_pos = match self.direction {
            Direction::Up => last_head_pos.shifted_by(0, -1),
            Direction::Down => last_head_pos.shifted_by(0, 1),
            Direction::Left => last_head_pos.shifted_by(-1, 0),
            Direction::Right => last_head_pos.shifted_by(1, 0),
        };
        self.head.set_position(new_head_pos);
        let mut new_block = self.body.pop_back().unwrap();
        self.prev_tail = new_block.get_position().clone();
        new_block.set_position(last_head_pos);
        self.body.push_front(new_block);
    }

    pub fn eat(&mut self) {
        let tail_pos = self.prev_tail.clone();
        let new_block = Block::new(tail_pos, SNAKE_BODY_SHAPE);
        self.body.push_back(new_block);
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

        let hit_top = row == (wall_width - 1);
        let hit_left = column == (wall_width - 1);
        let hit_bottom = row == (height - wall_width);
        let hit_right = column == (width - wall_width);

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

    #[allow(dead_code)]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn get_eatings(&self) -> u32 {
        self.eatings
    }

}