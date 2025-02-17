use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode;
use glam::Vec2;
use std::time::Duration;

use space_cake::game::Game;
use space_cake::graphics::GameGraphics;

#[derive(Default)]
struct KeyState {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

struct MainState {
    game: Game,
    graphics: GameGraphics,
    keys: KeyState,
}

impl MainState {
    pub fn new() -> Self {
        let mut game = Game::new();
        game.initialize();
        let graphics = GameGraphics::new();

        Self {
            game,
            graphics,
            keys: KeyState::default(),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        println!("Update called"); // Debug print

        // Get time since last frame
        let delta = ctx.time.delta();
        println!("Delta time: {:?}", delta); // Debug print

        // Handle keyboard input
        let kb = &ctx.keyboard;

        // Update key states
        self.keys.left = kb.is_key_pressed(KeyCode::Left) || kb.is_key_pressed(KeyCode::A);
        self.keys.right = kb.is_key_pressed(KeyCode::Right) || kb.is_key_pressed(KeyCode::D);
        self.keys.up = kb.is_key_pressed(KeyCode::Up) || kb.is_key_pressed(KeyCode::W);
        self.keys.down = kb.is_key_pressed(KeyCode::Down) || kb.is_key_pressed(KeyCode::S);

        // Calculate movement based on key states
        let mut movement = Vec2::ZERO;

        if self.keys.left {
            movement.x -= 1.0;
        }
        if self.keys.right {
            movement.x += 1.0;
        }
        if self.keys.up {
            movement.y -= 1.0;
        }
        if self.keys.down {
            movement.y += 1.0;
        }

        // Normalize movement vector if moving diagonally
        if movement.length_squared() > 0.0 {
            movement = movement.normalize();
        }

        // Apply movement to the player
        self.game.move_player(movement.x as f64, movement.y as f64);

        // Debug: Print player position
        println!(
            "Player position: ({}, {})",
            self.game.player().position().x,
            self.game.player().position().y
        );

        // Update the game state
        self.game.update(delta);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Draw called"); // Debug print
        self.graphics.draw(&self.game, ctx)
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("space_cake", "you")
        .window_setup(ggez::conf::WindowSetup::default().title("Space Cake RPG"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = MainState::new();
    event::run(ctx, event_loop, state)
}