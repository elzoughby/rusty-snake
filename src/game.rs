use rand::{thread_rng, Rng};
use piston_window::*;
use piston_window::types::Color;
use crate::snake::{Snake, Direction};
use crate::draw::*;


const FROG_COLOR: Color = [0.5, 1.0, 0.5, 1.0];
const BONUS_COLOR: Color = [0.0, 1.0, 1.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];
const RESTART_DELAY: f64 = 1.0;


pub struct Playground {
    width: u32,
    height: u32,
    color: Color,
    border_color: Color,
    border_width: u32,
}

pub struct Food {
    block: Block,
    color: Color,
    calories: u32,
    disappear_after: Option<f64>,
}

pub struct Game {
    playground: Playground,
    score: u32,
    snake: Snake,
    frog: Option<Food>,
    bonus: Option<Food>,
    eatings: u32,
    move_delay: f64,
    state: State,
}

pub enum State {
    Running,
    HitWalls,
    BiteItself,
    GameOver,
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

}


impl Food {

    pub fn new_frog(block: Block, color: Color) -> Food {
        Food {
            block,
            color,
            calories: 10,
            disappear_after: None,
        }
    }

    pub fn new_bonus(block: Block, color: Color) -> Food {
        Food {
            block,
            color,
            calories: 50,
            disappear_after: Some(30.0),
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        self.block.draw(self.color, context, graphics);
    }

}


impl Game {

    pub fn new(playground: Playground) -> Game {
        let ref_playground = &playground;
        let width =  ref_playground.width;
        let height = ref_playground.height;
        Game {
            playground: playground,
            score: 0,
            snake: Snake::new(Position (3, 3)),
            frog: Some(Food::new_frog(Block::new(Position (width - 4, height - 4), Shape::Triangle), FROG_COLOR)),
            bonus: None,
            eatings: 0,
            move_delay: 0.1,
            state: State::Running,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if let State::Running = self.state {
            let dir = match key {
                Key::Up => Some(Direction::Up),
                Key::Down => Some(Direction::Down),
                Key::Left => Some(Direction::Left),
                Key::Right => Some(Direction::Right),
                _ => None,
            };
            if let Some(direction) = dir {
                if direction != self.snake.get_direction().opposite() {
                    self.update_snake();
                }
            }
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        if let State::HitWalls = self.state {
            self.playground.border_color = [8.0, 0.0, 0.0, 1.0];
            self.state = State::GameOver;
        } else if let State::BiteItself = self.state {
            self.snake.set_color([8.0, 0.0, 0.0, 1.0]);
            self.state = State::GameOver;
        }
        self.playground.draw(context, graphics);
        if let Some(food) = &self.frog {
            food.draw(context, graphics);
        }
        if let Some(food) = &self.bonus {
            food.draw(context, graphics);
        }
        self.snake.draw(context, graphics);
        if let State::GameOver = self.state {
            draw_rectangle(
                Position (0, 0), 
                self.playground.width, 
                self.playground.height, 
                GAMEOVER_COLOR, 
                context, 
                graphics);
        }
    }

    fn update_snake(&mut self) {

    }

}