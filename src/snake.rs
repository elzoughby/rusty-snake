use std::collections::LinkedList;
use piston_window::types::Color;
use piston_window::{GfxFactory, Context, G2d};
use crate::playground::Playground;
use crate::draw::{Block, Shape, Position, Direction, draw_eyes};


const SNAKE_COLOR: Color = [0.19, 0.19, 0.18, 1.0];
const SNAKE_BODY_SHAPE: Shape = Shape::Square(SNAKE_COLOR);
const SNAKE_HEAD_SHAPE: Shape = Shape::Circle(SNAKE_COLOR);
const SNAKE_INIT_POSITION: Position = Position (5, 3);
const SNAKE_INIT_DIRECTION: Direction = Direction::Right;
const SNAKE_INIT_LENGTH: u8 = 2;


pub struct Snake {
    head: Block,
    body_shape: Shape,
    body: LinkedList<Block>,
    direction: Direction,
    init_position: Position,
    init_direction: Direction,
    prev_tail: Position,
    eatings: u32,
}


impl Default for Snake {

    fn default() -> Snake {
        let head = Block::new(SNAKE_INIT_POSITION, SNAKE_HEAD_SHAPE);
        Snake::new(head, SNAKE_BODY_SHAPE, SNAKE_INIT_DIRECTION)
    }

}


impl Snake {

    pub fn new(head: Block, body_shape: Shape, 
            init_direction: Direction) -> Snake {
        let init_position = head.get_position().clone();
        let body: LinkedList<Block> = Snake::build_snake_body(
            &body_shape,
            &init_position,
            &init_direction);
        let tail_pos = body.back().unwrap().get_position().clone();
        
        Snake {
            head,
            body_shape,
            body,
            direction: init_direction.clone(),
            init_direction,
            init_position,
            prev_tail: tail_pos,
            eatings: 0,
        }
    }

    fn build_snake_body(body_shape: &Shape, init_position: &Position,
            init_direction: &Direction) -> LinkedList<Block> {
        let mut body: LinkedList<Block> = LinkedList::new();
        let mut pos = init_position.clone();
        for _ in 0..SNAKE_INIT_LENGTH {
            pos = match init_direction {
                Direction::Up => pos.shifted_by(0, 1),
                Direction::Down => pos.shifted_by(0, -1),
                Direction::Left => pos.shifted_by(1, 0),
                Direction::Right => pos.shifted_by(-1, 0),
            };
            let body_block = Block::new(pos.clone(), body_shape.clone());
            body.push_back(body_block);
        }
        body
    }

    pub fn draw(&self, factory: &mut GfxFactory, context: &Context, 
                graphics: &mut G2d) {
        for block in self.body.iter() {
            block.draw(factory, context, graphics);
        }
        self.head.draw(factory, context, graphics);
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
        let body_shape = self.body_shape.clone();
        let new_block = Block::new(tail_pos, body_shape);
        self.body.push_back(new_block);
        self.eatings += 1;
    }

    pub fn reset(&mut self) {
        self.head.set_position(self.init_position.clone());
        self.body = Snake::build_snake_body(
            &self.body_shape,
            &self.init_position,
            &self.init_direction);
        self.direction = self.init_direction.clone();
        self.prev_tail = self.body.back().unwrap()
                .get_position().clone();
        self.eatings = 0;
    }

    pub fn worth_bonus(&self) -> bool {
        self.eatings != 0 && self.eatings%5 == 0
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

    pub fn get_eatings(&self) -> u32 {
        self.eatings
    }

}