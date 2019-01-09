use piston_window::{GfxFactory,Context, G2d};
use piston_window::types::Color;
use crate::draw::{Block, Shape, Position};


const FROG_COLOR: Color = [0.17, 0.51, 0.08, 1.0];
const BONUS_COLOR: Color = [0.0, 1.0, 1.0, 1.0];
const BONUS_DISAPPEAR_TIME: f64 = 5.0;
const FROG_IMAGE_PATH: &str = "./assets/Frog.png";
const BONUS_IMAGE_PATH: &str = "./assets/Mouse.png";


pub struct Food {
    block: Block,
    color: Color,
    calories: u32,
    disappear_after: Option<f64>,
}


impl Food {

    pub fn new_frog(position: Position) -> Food {
        let shape = Shape::Image(String::from(FROG_IMAGE_PATH));
        Food {
            block: Block::new(position, shape),
            color: FROG_COLOR,
            calories: 2,
            disappear_after: None,
        }
    }

    pub fn new_bonus(position: Position) -> Food {
        let shape = Shape::Image(String::from(BONUS_IMAGE_PATH));
        Food {
            block: Block::new(position, shape),
            color: BONUS_COLOR,
            calories: 10,
            disappear_after: Some(BONUS_DISAPPEAR_TIME),
        }
    }

    pub fn draw(&self, factory: &mut GfxFactory, context: &Context, graphics: &mut G2d) {
        self.block.draw(self.color, factory, context, graphics);
    }

    pub fn on_position(&self, position: &Position) -> bool {
        position == self.block.get_position()
    }

    pub fn get_position(&self) -> &Position {
        self.block.get_position()
    }

    pub fn get_calories(&self) -> u32 {
        self.calories
    }

    pub fn get_disappear_after(&self) -> Option<f64> {
        self.disappear_after
    }

}
