// src/unit/stats.rs
#[derive(Default, Clone, Debug)]
pub struct Stats {
    pub level: u32,
    pub experience: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub max_health: u32,
    pub current_health: u32,
    pub max_mana: u32,
    pub current_mana: u32,
}

impl Stats {
    pub fn new(level: u32) -> Self {
        Self {
            level,
            experience: 0,
            strength: 10 + level,
            dexterity: 10 + level,
            intelligence: 10 + level,
            max_health: 100 + (level * 10),
            current_health: 100 + (level * 10),
            max_mana: 50 + (level * 5),
            current_mana: 50 + (level * 5),
        }
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        let exp_needed = self.level * 100;  // Simple leveling formula
        
        if self.experience >= exp_needed {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.strength += 2;
        self.dexterity += 2;
        self.intelligence += 2;
        self.max_health += 10;
        self.current_health = self.max_health;
        self.max_mana += 5;
        self.current_mana = self.max_mana;
        self.experience = 0;  // Reset experience for next level
    }
}