#[cfg(target_os = "none")]
use alloc::{string::{String, ToString}, vec::Vec};

use crate::nadk::display::{COLOR_BLACK, COLOR_WHITE, ScreenPoint, ScreenRect, draw_string, push_rect_uniform};

const ROW_HEIGHT: u16 = 15;

pub struct StringList {
    items: Vec<String>,
    position: u16,
    rows: u16,
    x: u16,
    y: u16
}

impl StringList {
    /// Creates a new string list
    pub fn new(x: u16, y: u16, rows: u16) -> Self {
        StringList { items: Vec::new(), position: 0, x, y, rows }
    }

    /// Adds a new item to the string list
    pub fn add(&mut self, item: impl Into<String>) {
        self.items.push(item.into());
    }

    /// Puts to the list cursor in a specific position.
    /// If the position is over the upper bound of the list,
    /// the cursor will be placed on the last list item.
    pub fn select(&mut self, position: u16) -> Result<(), String> {
        if position > (self.items.len() - 1) as u16 {
            self.position = (self.items.len() - 1) as u16;
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

    fn render_cursor(&self) {
        draw_string(">", ScreenPoint::new(self.x, self.y + self.position * ROW_HEIGHT), false, COLOR_WHITE, COLOR_BLACK);
    }

    pub fn render(&self) {
        push_rect_uniform(ScreenRect::new(self.x, self.y, 10, self.rows * ROW_HEIGHT), COLOR_BLACK);
        self.render_cursor();

        for (i, item_str) in self.items.iter().enumerate() {
            draw_string(item_str, ScreenPoint::new(self.x + 10, self.y + i as u16 * ROW_HEIGHT), false, COLOR_WHITE, COLOR_BLACK);
        }
    }
}