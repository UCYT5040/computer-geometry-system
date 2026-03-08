#[cfg(target_os = "none")]
use alloc::{string::{String, ToString}, vec::Vec};

pub struct StringList {
    items: Vec<String>,
    position: usize
}

impl StringList {
    /// Creates a new string list
    pub fn new() -> Self {
        StringList { items: Vec::new(), position: 0 }
    }

    /// Adds a new item to the string list
    pub fn add(&mut self, item: String) {
        self.items.push(item);
    }

    /// Puts to the list cursor in a specific position.
    /// If the position is over the upper bound of the list,
    /// the cursor will be placed on the last list item.
    pub fn select(&mut self, position: usize) -> Result<(), String> {
        if position > self.items.len() - 1 {
            self.position = self.items.len() - 1;
            return Err("Position out of list bounds".to_string());
        }

        self.position = position;

        Ok(())
    }

    /// Selects the next list item
    pub fn next(&mut self) {
        self.select(self.position + 1);
    }

    // Selects the previous list item
    pub fn previous(&mut self) {
        if self.position < 1 { return; }

        self.select(self.position - 1);
    }
}