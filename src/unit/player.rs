use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops::Range;
use crate::unit::Position;

#[derive(Default)]
pub struct Player {
    position: Position<f64>,
    speed: f64,
    health: u8,
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: u8) {
        if damage >= self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }

    pub fn position(&self) -> &Position<f64> {
        &self.position
    }

    pub fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<f64>, y_range: Range<f64>) {
        self.position.x = rng.gen_range(x_range);
        self.position.y = rng.gen_range(y_range);
    }
}

pub struct PlayerBuilder {
    position: Position<f64>,
    speed: f64,
    health: u8,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
            position: Position::default(),
            speed: 0.0,
            health: 100,
        }
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    pub fn health(mut self, health: u8) -> Self {
        self.health = health;
        self
    }

    pub fn build(self) -> Player {
        Player {
            position: self.position,
            speed: self.speed,
            health: self.health,
        }
    }
}
