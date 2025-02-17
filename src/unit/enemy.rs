use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops::Range;
use crate::unit::Position;

#[derive(Default)]
pub struct Enemy {
    position: Position<f64>,
    speed: f64,
}

impl Enemy {
    pub fn with_speed(speed: f64) -> Self {
        Self {
            position: Position::default(),
            speed,
        }
    }

    pub fn position(&self) -> &Position<f64> {
        &self.position
    }

    pub fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<f64>, y_range: Range<f64>) {
        self.position.x = rng.gen_range(x_range);
        self.position.y = rng.gen_range(y_range);
    }

    pub fn move_to(&mut self, target: &Position<f64>) {
        let dx = target.x - self.position.x;
        let dy = target.y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let ratio = self.speed / distance;
        self.position.x += dx * ratio;
        self.position.y += dy * ratio;
    }
}
