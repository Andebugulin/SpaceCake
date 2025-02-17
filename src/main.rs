use space_cake::game::Game;
use std::time::{Duration, Instant};

fn main() {
    let mut game = Game::new();
    game.initialize();

    let frame_duration = Duration::from_millis(16); // ~60 FPS
    let mut last_update = Instant::now();

    loop {
        let now = Instant::now();
        let delta = now - last_update;
        
        if delta >= frame_duration {
            // Here you would handle input and move the player
            // For example:
            // game.move_player(1.0, 0.0); // Move right
            
            game.update(delta);
            last_update = now;
            
            // Here you would render the game state
            // For now, let's just print the player's health
            println!("Player health: {}", game.player().health());
            
            if !game.player().is_alive() {
                println!("Game Over!");
                break;
            }
        }
    }
}