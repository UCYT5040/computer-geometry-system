#![allow(unused)]

#[cfg(target_os = "none")]
use alloc::{string::String, vec::Vec};

use indextree::NodeId;

use crate::{
    nadk::display::{
        COLOR_BLACK, COLOR_WHITE, ScreenPoint, ScreenRect, draw_string, push_rect_uniform,
    },
    tree::EquationTree,
};

const ROW_HEIGHT: u16 = 15;
pub const SCREEN_WIDTH: u16 = 320;
pub const SCREEN_HEIGHT: u16 = 240;

#[derive(Clone)]
pub struct ListItem {
    pub name: String,
    pub id: Option<NodeId>,
}

pub struct StringList {
    items: Vec<ListItem>,
    position: u16,
    rows: u16,
    x: u16,
    y: u16,
}

impl StringList {
    /// Creates a new string list
    pub fn new(x: u16, y: u16, rows: u16) -> Self {
        StringList {
            items: Vec::new(),
            position: 0,
            x,
            y,
            rows,
        }
    }

    /// Creates a new string list with the maximum number of rows
    pub fn new_with_max_row_count(x: u16, y: u16) -> Self {
        let max_rows = (SCREEN_HEIGHT - y) / ROW_HEIGHT;
        StringList {
            items: Vec::new(),
            position: 0,
            x,
            y,
            rows: max_rows,
        }
    }

    /// Adds a new item to the string list
    pub fn add(&mut self, item: impl Into<String>) {
        self.items.push(ListItem {
            name: item.into(),
            id: None,
        });
    }

    pub fn add_tree_item(&mut self, tree: &EquationTree, item: NodeId) {
        if let Some(data) = tree.get_data(item) {
            self.items.push(ListItem {
                name: data.name.clone(),
                id: Some(item),
            });
        }
    }

    pub fn add_tree_items(&mut self, tree: &EquationTree, items: Vec<NodeId>) {
        for item in items {
            self.add_tree_item(tree, item);
        }
    }

    pub fn remove(&mut self, position: u16) -> ListItem {
        if position <= self.position {
            self.position = self.position.saturating_sub(1);
        }
        self.items.remove(position as usize)
    }

    pub fn remove_all(&mut self, item: impl Into<String>) {
        let item = item.into();
        self.items.retain_mut(|value| *value.name != item);
        self.position = 0;
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn remove_current(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if self.position as usize >= self.items.len() {
            self.items.remove(self.items.len() - 1);
            self.position = self.items.len().saturating_sub(1) as u16;
            return;
        }
        self.items.remove(self.position as usize);
        self.position = self.position.saturating_sub(1);
    }

    /// Puts to the list cursor in a specific position.
    /// If the position is over the upper bound of the list,
    /// the cursor will be placed on the last list item.
    pub fn select(&mut self, position: u16) -> Result<(), &'static str> {
        if position as usize >= self.items.len() {
            self.position = self.items.len().saturating_sub(1) as u16;
            return Err("Position out of list bounds");
        }

        self.position = position;

        Ok(())
    }

    pub fn get_selected(&self) -> Option<ListItem> {
        if self.items.is_empty() {
            return None;
        }
        if self.position as usize >= self.items.len() {
            return self.items.last().cloned();
        }
        self.items.get(self.position as usize).cloned()
    }

    /// Selects the next list item
    pub fn next(&mut self) {
        let _ = self.select(self.position + 1);
    }

    /// Selects the previous list item
    pub fn previous(&mut self) {
        if self.position < 1 {
            return;
        }

        let _ = self.select(self.position - 1);
    }

    fn render_cursor(&self) {
        draw_string(
            ">",
            ScreenPoint::new(self.x, self.y + (self.position % self.rows) * ROW_HEIGHT),
            false,
            COLOR_WHITE,
            COLOR_BLACK,
        );
    }

    pub fn clear_screen(&self) {
        push_rect_uniform(
            ScreenRect::new(
                self.x,
                self.y,
                SCREEN_WIDTH - self.x,
                self.rows * ROW_HEIGHT,
            ),
            COLOR_BLACK,
        );
    }

    /// Renders the list
    pub fn render(&self) {
        self.clear_screen();

        if self.items.is_empty() {
            draw_string(
                "List is empty",
                ScreenPoint::new(self.x, self.y),
                false,
                COLOR_WHITE,
                COLOR_BLACK,
            );
            return;
        }

        self.render_cursor();

        let page = self.position / self.rows;

        for (i, item) in self
            .items
            .iter()
            .skip((page * self.rows).into())
            .take(self.rows as usize)
            .enumerate()
        {
            draw_string(
                &item.name,
                ScreenPoint::new(self.x + 10, self.y + i as u16 * ROW_HEIGHT),
                false,
                COLOR_WHITE,
                COLOR_BLACK,
            );
        }
    }
}
