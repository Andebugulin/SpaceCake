use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops::Range;
use crate::unit::Position;

#[derive(Default)]
pub struct Collectible {
    position: Position<u16>,
}

impl Collectible {
    pub fn position(&self) -> &Position<u16> {
        &self.position
    }

    pub fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<u16>, y_range: Range<u16>) {
        self.position.x = rng.gen_range(x_range);
        self.position.y = rng.gen_range(y_range);
    }
}
