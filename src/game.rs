use crate::{unit::Collectible, unit::Enemy, unit::Player, unit::Wall};
use std::time::Duration;
use rand::prelude::*;

#[allow(unused)]
pub struct Game {
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
    collectible: Collectible,
    player: Player,
}

#[allow(clippy::new_without_default)]
impl Game {
    pub fn new() -> Self {
        Self {
            enemies: vec![Enemy::default(), Enemy::default()],
            walls: vec![Wall::default(), Wall::default(), Wall::default()],
            collectible: Collectible::default(),
            player: Player::default(),
        }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn update(&mut self, _delta_time: Duration) {
        if !self.player.is_alive() {
            return;
        }

        // Update enemy positions
        for enemy in &mut self.enemies {
            enemy.move_to(self.player.position());
            
            // Check collision with player
            let dx = enemy.position().x - self.player.position().x;
            let dy = enemy.position().y - self.player.position().y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance < 1.0 {  // Collision radius
                self.player.take_damage(10);
            }
        }

        // Check collectible collision
        let dx = (self.collectible.position().x as f64) - self.player.position().x;
        let dy = (self.collectible.position().y as f64) - self.player.position().y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance < 1.5 {  // Collection radius
            // Respawn collectible in random position
            let mut rng = thread_rng();
            self.collectible.set_rand_position(&mut rng, 0..100, 0..100);
        }
    }

    pub fn move_player(&mut self, dx: f64, dy: f64) {
        if !self.player.is_alive() {
            return;
        }

        let speed = self.player.speed();
        let new_x = self.player.position().x + dx * speed;
        let new_y = self.player.position().y + dy * speed;

        // Check wall collisions
        let can_move = !self.walls.iter().any(|wall| {
            let wall_x = wall.position().x as f64;
            let wall_y = wall.position().y as f64;
            let distance = ((new_x - wall_x).powi(2) + (new_y - wall_y).powi(2)).sqrt();
            distance < 1.0  // Collision radius
        });

        if can_move {
            self.player.move_by(dx * speed, dy * speed);
        }
    }

    pub fn initialize(&mut self) {
        let mut rng = thread_rng();
        
        // Set random positions for all entities
        self.player.set_rand_position(&mut rng, 0.0..100.0, 0.0..100.0);
        
        for enemy in &mut self.enemies {
            enemy.set_rand_position(&mut rng, 0.0..100.0, 0.0..100.0);
        }
        
        for wall in &mut self.walls {
            wall.set_rand_position(&mut rng, 0..100, 0..100);
        }
        
        self.collectible.set_rand_position(&mut rng, 0..100, 0..100);
    }
}