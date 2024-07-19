use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops::Range;
use crate::unit::Position;

#[derive(Default)]
pub struct Wall {
    position: Position<u16>,
}

impl Wall {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            position: Position { x, y },
        }
    }

    pub fn position(&self) -> &Position<u16> {
        &self.position
    }

    pub fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<u16>, y_range: Range<u16>) {
        self.position.x = rng.gen_range(x_range);
        self.position.y = rng.gen_range(y_range);
    }
}
