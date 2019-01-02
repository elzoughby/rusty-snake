use rand::{thread_rng, Rng};
use piston_window::*;
use piston_window::types::Color;
use crate::food::Food;
use crate::playground::Playground;
use crate::snake::{Snake, Direction};
use crate::draw::{Position, draw_rectangle};


const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];
const RESTART_DELAY: f64 = 2.0;
const MOVE_DELAY: f64 = 0.2;


pub struct Game {
    playground: Playground,
    score: u32,
    snake: Snake,
    frog: Option<Food>,
    bonus: Option<Food>,
    move_delay: f64,
    state: State,
    waiting_time: f64,
}

pub enum State {
    Running,
    HitWalls,
    BiteItself,
    GameOver,
}


impl Game {

    pub fn new(playground: Playground) -> Game {
        let ref_playground = &playground;
        let width =  ref_playground.get_width();
        let height = ref_playground.get_height();
        Game {
            playground: playground,
            score: 0,
            snake: Snake::new(Position (3, 3)),
            frog: Some(Food::new_frog(Position (width - 4, height - 4))),
            bonus: None,
            move_delay: MOVE_DELAY,
            state: State::Running,
            waiting_time: 0.0,
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
                if *direction != self.snake.get_direction().opposite() {
                    self.update_snake(dir);
                }
            }
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
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
                self.playground.get_width(), 
                self.playground.get_height(), 
                GAMEOVER_COLOR, 
                context, 
                graphics);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        self.update_state();
        if let State::HitWalls = self.state {
            self.playground.set_border_color([8.0, 0.0, 0.0, 1.0]);
            self.state = State::GameOver;
        } else if let State::BiteItself = self.state {
            self.snake.set_color([8.0, 0.0, 0.0, 1.0]);
            self.state = State::GameOver;
        }
        if let State::GameOver = self.state {
            if self.waiting_time > RESTART_DELAY {
                self.restart();
            }
            return;
        }
        if self.frog.is_none() {
            let position = self.get_random_position();
            self.frog = Some(Food::new_frog(position));
        }
        if self.waiting_time > self.move_delay {
            self.update_snake(None);
        }
    }

    fn try_eating(&mut self) {
        if let Some(food) = &self.frog {
            if food.get_position() == self.snake.get_head_position() {
                self.snake.eat();
                self.score += food.get_calories();
                self.frog = None;
            }
        } 
        if let Some(food) = &self.bonus {
            if food.get_position() == self.snake.get_head_position() {
                self.snake.eat();
                self.score += food.get_calories();
                self.bonus = None;
            }
        }
    }

    fn update_state(&mut self) {
        if let State::GameOver = self.state {
            return;
        } 
        
        self.state = if self.snake.bite_itself() {
                State::BiteItself
            } else if self.snake.hit_walls_of(&self.playground) {
                State::HitWalls
            } else {
                State::Running
            };
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if let State::Running = self.state {
            self.snake.step(dir);
            self.try_eating();
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(Position (3, 3));
        self.state = State::Running;
        self.score = 0;
        self.waiting_time = 0.0;
        self.frog = Some(Food::new_frog(self.get_random_position()));
    }

    fn get_random_position(&self) -> Position {
        let mut rng = thread_rng();
        let border = self.playground.get_border_width();
        let width = self.playground.get_width();
        let height = self.playground.get_height();
        //let frog = self.frog.clone();
        //let bonus = self.bonus.clone();
        //let has_frog = !frog.is_none();
        //let has_bonus = !bonus.is_none();
        let mut column = rng.gen_range(border, width - border - 1);
        let mut row = rng.gen_range(border, height - border - 1);
        let mut new_pos = Position (column, row);

        while self.snake.on_position(&new_pos) 
                //|| (has_frog && frog.unwrap().get_position()  == &new_pos) 
                //||(has_bonus && bonus.unwrap().get_position() == &new_pos) 
        {
            column = rng.gen_range(border, width - border - 1);
            row = rng.gen_range(border, height - border - 1);
            new_pos = Position (column, row);
        }
        new_pos
    }

}