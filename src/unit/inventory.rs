// src/unit/inventory.rs
#[derive(Debug, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub item_type: ItemType,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Weapon { damage: u32 },
    Armor { defense: u32 },
    Consumable { health_restore: u32, mana_restore: u32 },
}

#[derive(Default)]
pub struct Inventory {
    items: Vec<Item>,
    capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn add_item(&mut self, item: Item) -> bool {
        if self.items.len() < self.capacity {
            self.items.push(item);
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
}