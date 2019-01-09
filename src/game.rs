use rand::{thread_rng, Rng};
use piston_window::*;
use piston_window::types::Color;
use crate::food::Food;
use crate::playground::Playground;
use crate::snake::{Snake};
use crate::draw::{Position, Direction, draw_rectangle, draw_text};


const GAMEOVER_COLOR: Color = [0.0, 0.0, 0.0, 0.90];
const WHITE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
const BLACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const SCORE_FONT_SIZE: u32 = 12;
const GAMEOVER_FONT_SIZE: u32 = 20;
const MOVE_DELAY: f64 = 0.24;


pub struct Game {
    playground: Playground,
    score: u32,
    snake: Snake,
    frog: Option<Food>,
    bonus: Option<Food>,
    move_delay: f64,
    state: State,
    waiting_time: f64,
    bonus_time: f64,
    worth_bonus: bool,
}

pub enum State {
    Running,
    GameOver,
}


impl Game {

    pub fn new(width: u32, height: u32) -> Game {
        Game {
            playground: Playground::new(width, height),
            score: 0,
            snake: Snake::new(&Position (3, 3)),
            frog: None,
            bonus: None,
            move_delay: MOVE_DELAY,
            state: State::Running,
            waiting_time: 0.0,
            bonus_time: 0.0,
            worth_bonus: false,
        }
    }

    pub fn draw(&self, factory: &mut GfxFactory, 
            context: &Context, graphics: &mut G2d) {
        self.playground.draw(factory, context, graphics);
        if let Some(food) = &self.frog {
            food.draw(factory, context, graphics);
        }
        if let Some(food) = &self.bonus {
            food.draw(factory, context, graphics);
        }
        self.snake.draw(factory, context, graphics);
        draw_rectangle(
            &Position (0, self.playground.get_height()),
            self.playground.get_width(),
            2,
            [0.80, 0.30, 0.30, 1.0], //border color
            context,
            graphics);
        draw_text(
            &format!("Score: {}", &self.score), 
            &Position (2, self.playground.get_height()+1), 
            BLACK_COLOR, 
            SCORE_FONT_SIZE, 
            factory.clone(), 
            context, 
            graphics);

        if let State::GameOver = self.state {
            draw_rectangle(
                &Position (0, 0), 
                self.playground.get_width(), 
                self.playground.get_height()+2, 
                GAMEOVER_COLOR, 
                context, 
                graphics);
            draw_text(
                "Game Over", 
                &Position (13, 12), 
                WHITE_COLOR, 
                GAMEOVER_FONT_SIZE, 
                factory.clone(), 
                context, 
                graphics);
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
            if let Some(direction) = &dir {
                let snake_direction = self.snake.get_direction();
                if direction != snake_direction &&
                        *direction != snake_direction.opposite() {
                    self.update_snake(dir);
                }
            }
        } else if let Key::Return = key {
            self.restart();
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        self.bonus_time += delta_time;
        if let State::GameOver = self.state {
            return;
        } else if self.snake.bite_itself() ||
                self.snake.hit_walls_of(&self.playground) {
            self.state = State::GameOver;
        }
        if self.frog.is_none() {
            let position = self.get_random_position();
            self.frog = Some(Food::new_frog(position));
        }
        if self.bonus.is_none() {
            let eatings = self.snake.get_eatings();
            if eatings%5 == 0 && self.worth_bonus {
                let position = self.get_random_position();
                self.bonus = Some(Food::new_bonus(position));
                self.bonus_time = 0.0;
            }
        } else {
            let bonus = self.bonus.as_ref().unwrap();
            if self.bonus_time > bonus.get_disappear_after().unwrap() {
                self.bonus = None;
                self.worth_bonus = false;
            }
        }
        if self.waiting_time > self.move_delay {
            self.update_snake(None);
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        self.snake.step(dir);
        self.try_eating();
        self.waiting_time = 0.0;
    }

    fn try_eating(&mut self) {
        if let Some(food) = &self.frog {
            if food.get_position() == self.snake.get_head_position() {
                self.snake.eat();
                self.score += food.get_calories();
                self.frog = None;
                self.worth_bonus = true;
            }
        } 
        if let Some(food) = &self.bonus {
            if food.get_position() == self.snake.get_head_position() {
                self.snake.eat();
                self.score += food.get_calories();
                self.bonus = None;
                // increase game speed
                if self.move_delay > 0.0 {
                    self.move_delay -= 0.03;
                }
            }
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(&Position (3, 3));
        self.state = State::Running;
        self.move_delay = MOVE_DELAY;
        self.waiting_time = 0.0;
        self.worth_bonus = false;
        self.frog = None;
        self.bonus = None;
        self.score = 0;
    }

    fn get_random_position(&self) -> Position {
        let mut rng = thread_rng();
        let border = self.playground.get_border_width();
        let width = self.playground.get_width();
        let height = self.playground.get_height();
        let has_frog = self.frog.is_some();
        let has_bonus = self.bonus.is_some();
        let mut column = rng.gen_range(border, width - border - 1);
        let mut row = rng.gen_range(border, height - border - 1);
        let mut new_pos = Position (column, row);

        while self.snake.on_position(&new_pos) 
                || (has_frog && self.frog.as_ref().unwrap().on_position(&new_pos)) 
                || (has_bonus && self.bonus.as_ref().unwrap().on_position(&new_pos)) 
        {
            column = rng.gen_range(border, width - border - 1);
            row = rng.gen_range(border, height - border - 1);
            new_pos = Position (column, row);
        }
        new_pos
    }

    pub fn get_size(&self) -> [f64; 2] {
        let width = self.playground.get_width();
        let height = self.playground.get_height() + 2;
        Position::new(width, height)
            .to_coord()
            .as_array()
    }

}