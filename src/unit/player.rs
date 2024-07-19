#[derive(Default)]
pub struct Player {
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
}

pub struct PlayerBuilder {
    speed: f64,
    health: u8,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
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
            speed: self.speed,
            health: self.health,
        }
    }
}