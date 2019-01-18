use rand::{thread_rng, Rng};
use piston_window::*;
use piston_window::types::Color;
use crate::food::Food;
use crate::playground::Playground;
use crate::snake::{Snake};
use crate::draw::{Position, Direction, draw_rectangle, draw_text};


const GAMEOVER_COLOR: Color = [0.0, 0.0, 0.0, 0.90];
const WHITE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
const SCORE_FONT_SIZE: u32 = 12;
const GAMEOVER_FONT_SIZE: u32 = 20;
const MOVE_DELAY: f64 = 0.3; //300ms


pub struct Game {
    playground: Playground,
    snake: Snake,
    food: Food,
    bonus: Food,
    score: u32,
    status: Status,
    move_delay: f64,
    waiting_time: f64,
    bonus_time: f64,
    show_bonus: bool,
    missed_bonus: bool,
}

pub enum Status {
    Running,
    GameOver,
}


impl Default for Game {

    fn default() -> Game {
        let playground = Playground::default();
        let snake = Snake::default();
        let food = Food::default_food();
        let bonus = Food::default_bonus();
        Game::new(playground, snake, food, bonus, MOVE_DELAY)
    }

}


impl Game {

    pub fn new(playground: Playground, snake: Snake,
            food: Food, bonus: Food, move_delay: f64) 
            -> Game {
        let mut game = Game {
            playground,
            snake,
            food,
            bonus,
            move_delay,
            score: 0,
            status: Status::Running,
            waiting_time: 0.0,
            bonus_time: 0.0,
            show_bonus: false,
            missed_bonus: false,
        };
        // randomize food position
        game.food.set_position(game.get_random_position());
        game
    }

    pub fn draw(&self, factory: &mut GfxFactory, 
            context: &Context, graphics: &mut G2d) {
        self.playground.draw(factory, context, graphics);
        self.food.draw(factory, context, graphics);
        if self.show_bonus {
            self.bonus.draw(factory, context, graphics);
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
            WHITE_COLOR, 
            SCORE_FONT_SIZE, 
            factory.clone(), 
            context, 
            graphics);

        if let Status::GameOver = self.status {
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
        if let Status::Running = self.status {
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
        if let Status::GameOver = self.status {
            return;
        } else if self.snake.bite_itself() ||
                self.snake.hit_walls_of(&self.playground) {
            self.status = Status::GameOver;
        }

        if self.show_bonus {
            if self.bonus_time > self.bonus.get_disappear_after().unwrap() {
                self.show_bonus = false;
                self.missed_bonus = true;
            }
        } else if self.snake.worth_bonus() && !self.missed_bonus {
            let new_pos = self.get_random_position();
            self.bonus.set_position(new_pos);
            self.show_bonus = true;
            self.bonus_time = 0.0;
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
        if self.food.get_position() == self.snake.get_head_position() {
            self.snake.eat();
            self.score += self.food.get_calories();
            self.food.set_position(self.get_random_position());
            self.missed_bonus = false;
        } else if self.show_bonus && 
                self.bonus.on_position(self.snake.get_head_position()) {
            self.snake.eat();
            self.score += self.bonus.get_calories();
            self.show_bonus = false;
            // increase game speed
            if self.move_delay > 0.0 {
                self.move_delay -= 0.02;
            }
        }
    }

    fn restart(&mut self) {
        self.snake.reset();
        self.status = Status::Running;
        self.move_delay = MOVE_DELAY;
        self.waiting_time = 0.0;
        self.score = 0;
        self.show_bonus = false;
        self.food.set_position(self.get_random_position());
        self.bonus.set_position(self.get_random_position());
    }

    fn get_random_position(&self) -> Position {
        let mut rng = thread_rng();
        let border = self.playground.get_border_width();
        let width = self.playground.get_width();
        let height = self.playground.get_height();
        let mut column = rng.gen_range(border, width - border - 1);
        let mut row = rng.gen_range(border, height - border - 1);
        let mut new_pos = Position (column, row);

        while self.snake.on_position(&new_pos) 
                || self.food.on_position(&new_pos)
                || self.bonus.on_position(&new_pos)
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