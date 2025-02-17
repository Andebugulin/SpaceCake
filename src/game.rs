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

    pub fn enemies(&self) -> &Vec<Enemy> {
        &self.enemies
    }

    pub fn enemies_mut(&mut self) -> &mut Vec<Enemy> {
        &mut self.enemies
    }

    pub fn walls(&self) -> &Vec<Wall> {
        &self.walls
    }

    pub fn collectible(&self) -> &Collectible {
        &self.collectible
    }

    pub fn update(&mut self, delta_time: Duration) {
        if !self.player.is_alive() {
            return;
        }

        let delta_seconds = delta_time.as_secs_f64();
        

        // Update enemy positions - multiply by delta_time for smooth movement
        for enemy in &mut self.enemies {
            let target_pos = self.player.position().clone();
            enemy.move_to(&target_pos);
            
            // Check collision with player
            let dx = enemy.position().x - self.player.position().x;
            let dy = enemy.position().y - self.player.position().y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance < 30.0 {  // Collision radius increased for better gameplay
                self.player.take_damage(10);
                println!("Player hit! Health: {}", self.player.health()); // Debug info
            }
        }

        // Check collectible collision
        let dx = (self.collectible.position().x as f64) - self.player.position().x;
        let dy = (self.collectible.position().y as f64) - self.player.position().y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance < 30.0 {  // Collection radius increased for better gameplay
            // Respawn collectible in random position
            let mut rng = thread_rng();
            self.collectible.set_rand_position(&mut rng, 0..800, 0..600); // Match window size
            // Could add points/healing here
            println!("Collectible gathered!"); // Debug info
        }
    }

    pub fn move_player(&mut self, dx: f64, dy: f64) {
        println!("move_player called: dx = {}, dy = {}", dx, dy); // Debug print
    
        if!self.player.is_alive() {
            return;
        }
    
        let speed = self.player.speed() * 5.0; // Increased speed for better gameplay
        println!("Player speed: {}", speed); // Debug print
    
        let new_x = self.player.position().x + dx * speed;
        let new_y = self.player.position().y + dy * speed;
    
        // Debug: Print new position
        println!("New player position: ({}, {})", new_x, new_y);
    
        // Keep player in bounds
        let new_x = new_x.clamp(0.0, 800.0);
        let new_y = new_y.clamp(0.0, 600.0);
    
        // Debug: Print clamped position
        println!("Clamped player position: ({}, {})", new_x, new_y);
    
        // Check wall collisions
        let can_move = !self.walls.iter().any(|wall| {
            let wall_x = wall.position().x as f64;
            let wall_y = wall.position().y as f64;
            let distance = ((new_x - wall_x).powi(2) + (new_y - wall_y).powi(2)).sqrt();
            if distance < 35.0 {
                println!("Wall collision detected at: ({}, {})", wall_x, wall_y); // Debug print
                true
            } else {
                false
            }
        });
    
        if can_move {
            println!("Calling move_by: dx = {}, dy = {}", dx * speed, dy * speed); // Debug print
            self.player.move_by(dx * speed, dy * speed);
        } else {
            println!("Wall collision detected, cannot move"); // Debug print
        }
    
        // Debug: Print final player position
        println!(
            "Final player position: ({}, {})",
            self.player.position().x,
            self.player.position().y
        );
    }

    pub fn initialize(&mut self) {
        let mut rng = thread_rng();
    
        // Position player in center
        self.player.position_mut().x = 400.0;
        self.player.position_mut().y = 300.0;
    
        // Spread enemies around the edges
        for enemy in &mut self.enemies {
            let mut enemy_rng = thread_rng();
            if rng.gen_bool(0.5) {
                enemy.set_rand_position(&mut enemy_rng, 5.0..10.0, 5.0..100.0);
            } else {
                enemy.set_rand_position(&mut enemy_rng, 750.0..800.0, 0.0..600.0);
            }
        }
    
        // Position walls in strategic locations
        for (i, wall) in self.walls.iter_mut().enumerate() {
            let x = ((i as f64 + 1.0) * 200.0) as u16;
            let mut wall_rng = thread_rng();
            wall.set_rand_position(&mut wall_rng, x - 50..x + 50, 250..350);
        }
    
        // Position collectible randomly but away from player
        self.collectible.set_rand_position(&mut rng, 600..750, 100..500);
    }
}