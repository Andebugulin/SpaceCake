// src/unit/player.rs
use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops::Range;
use crate::unit::Position;
use crate::unit::Stats;
use crate::unit::Inventory;

#[derive(Default)]
pub struct Player {
    position: Position<f64>,
    speed: f64,
    health: u8,
    stats: Stats,
    inventory: Inventory,
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

    pub fn position_mut(&mut self) -> &mut Position<f64> {
        &mut self.position
    }

    pub fn move_by(&mut self, dx: f64, dy: f64) {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<f64>, y_range: Range<f64>) {
        self.position.x = rng.gen_range(x_range);
        self.position.y = rng.gen_range(y_range);
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn stats_mut(&mut self) -> &mut Stats {
        &mut self.stats
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}

pub struct PlayerBuilder {
    position: Position<f64>,
    speed: f64,
    health: u8,
    stats: Stats,
    inventory: Inventory,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
            position: Position::default(),
            speed: 1.0,
            health: 100,
            stats: Stats::new(1),
            inventory: Inventory::new(20),
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
            stats: self.stats,
            inventory: self.inventory,
        }
    }
}