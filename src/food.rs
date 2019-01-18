use piston_window::{GfxFactory,Context, G2d};
use crate::draw::{Block, Shape, Position};


const BONUS_DISAPPEAR_TIME: f64 = 5.0;
const FROG_IMAGE: &str = "Frog.png";
const BONUS_IMAGE: &str = "Mouse.png";


pub struct Food {
    block: Block,
    calories: u32,
    disappear_after: Option<f64>,
}


impl Food {

    pub fn new(shape: Shape, calories: u32, 
            disappear_after: Option<f64>) -> Food {
        let block = Block::new(Position::new(0, 0), shape);
        Food {
            block,
            calories,
            disappear_after,
        }
    }

    pub fn default_food() -> Food {
        let shape = Shape::Image(String::from(FROG_IMAGE));
        Food::new(shape, 2, None)
    }

    pub fn default_bonus() -> Food {
        let shape = Shape::Image(String::from(BONUS_IMAGE));
        Food::new(shape, 10, Some(BONUS_DISAPPEAR_TIME))
    }

    pub fn draw(&self, factory: &mut GfxFactory,
            context: &Context, graphics: &mut G2d) {
        self.block.draw(factory, context, graphics);
    }

    pub fn on_position(&self, position: &Position) -> bool {
        position == self.block.get_position()
    }

    pub fn get_position(&self) -> &Position {
        self.block.get_position()
    }

    pub fn set_position(&mut self, position: Position) {
        self.block.set_position(position);
    }

    pub fn get_calories(&self) -> u32 {
        self.calories
    }

    pub fn get_disappear_after(&self) -> Option<f64> {
        self.disappear_after
    }

}
