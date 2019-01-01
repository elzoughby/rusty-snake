use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::draw::Block;


const FROG_COLOR: Color = [0.5, 1.0, 0.5, 1.0];
const BONUS_COLOR: Color = [0.0, 1.0, 1.0, 1.0];


pub struct Food {
    block: Block,
    color: Color,
    calories: u32,
    disappear_after: Option<f64>,
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
