use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::draw::{Block, Shape, Position};


const FROG_COLOR: Color = [0.17, 0.51, 0.08, 1.0];
const BONUS_COLOR: Color = [0.0, 1.0, 1.0, 1.0];
const FROG_SHAPE: Shape = Shape::Square;
const BONUS_SHAPE: Shape = Shape::Circle;

#[derive(Clone)]
pub struct Food {
    block: Block,
    color: Color,
    calories: u32,
    disappear_after: Option<f64>,
}


impl Food {

    pub fn new_frog(position: Position) -> Food {
        Food {
            block: Block::new(position, FROG_SHAPE),
            color: FROG_COLOR,
            calories: 2,
            disappear_after: None,
        }
    }

    pub fn new_bonus(position: Position) -> Food {
        Food {
            block: Block::new(position, BONUS_SHAPE),
            color: BONUS_COLOR,
            calories: 10,
            disappear_after: Some(10.0),
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        self.block.draw(self.color, context, graphics);
    }

    pub fn get_position(&self) -> &Position {
        self.block.get_position()
    }

    pub fn get_calories(&self) -> u32 {
        self.calories
    }

}
