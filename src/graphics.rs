use ggez::{Context, GameResult, graphics::{self, Canvas, DrawParam, Color, Mesh}};
use glam::Vec2;

use crate::game::Game;

pub struct GameGraphics {
    player_size: f32,
    enemy_size: f32,
    wall_size: f32,
    collectible_size: f32,
}

impl GameGraphics {
    pub fn new() -> Self {
        Self {
            player_size: 30.0,
            enemy_size: 25.0,
            wall_size: 40.0,
            collectible_size: 15.0,
        }
    }

    pub fn draw(&self, game: &Game, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Draw player
        let player_pos = game.player().position();
        let player_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.player_size,
            0.1,
            Color::GREEN,
        )?;
        canvas.draw(&player_mesh, DrawParam::default().dest(Vec2::new(player_pos.x as f32, player_pos.y as f32)));

        // Draw enemies
        let enemy_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.enemy_size,
            0.1,
            Color::RED,
        )?;
        
        for enemy in game.enemies() {
            let enemy_pos = enemy.position();
            canvas.draw(&enemy_mesh, DrawParam::default().dest(Vec2::new(enemy_pos.x as f32, enemy_pos.y as f32)));
        }

        // Draw walls
        let wall_mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, self.wall_size, self.wall_size),
            Color::new(0.5, 0.5, 0.5, 1.0),
        )?;

        // Draw collectible
        let collectible_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.collectible_size,
            0.1,
            Color::YELLOW,
        )?;
        
        let collectible_pos = game.collectible().position();
        canvas.draw(&collectible_mesh, DrawParam::default().dest(Vec2::new(collectible_pos.x as f32, collectible_pos.y as f32)));

        // Draw UI
        let health_text = graphics::Text::new(format!("Health: {}", game.player().health()));
        canvas.draw(&health_text, DrawParam::default().dest(Vec2::new(10.0, 10.0)).color(Color::WHITE));

        let level_text = graphics::Text::new(format!("Level: {}", game.player().stats().level));
        canvas.draw(&level_text, DrawParam::default().dest(Vec2::new(10.0, 30.0)).color(Color::WHITE));

        canvas.finish(ctx)?;
        Ok(())
    }
}