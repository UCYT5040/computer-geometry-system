use core::ops::Range;

#[cfg(target_os = "none")]
use alloc::{string::String, format, vec::Vec};

use crate::{ui::list::{SCREEN_HEIGHT, SCREEN_WIDTH}, nadk::{display::{COLOR_BLACK, COLOR_WHITE, Color565, ScreenPoint, ScreenRect, draw_string, push_rect_uniform}, keyboard::{InputManager, Key}, time}};

const ROW_LENGTH: usize = 45;
const ROW_HEIGHT: usize = 15;
const EDITOR_START: u16 = 18;

struct BWColor {
    color: Color565
}

#[derive(Default)]
struct TextCursor {
    row: usize,
    pos: usize,
    color: BWColor
}

struct TextContent {
    rows: Vec<String>
}

pub struct TextEditor {
    content: TextContent,
    shift_pressed: bool,
    alpha_pressed: bool,
    time: u64,
    cursor: TextCursor
}

impl TextEditor {
    pub fn new() -> Self {
        return TextEditor { content: TextContent::new(), shift_pressed: false, alpha_pressed: false, time: 0, cursor: TextCursor::default() }
    }

    pub fn start(&mut self, input_man: &mut InputManager) -> String {
        self.clear_screen();
        self.render_button_states();
        self.render_top_bar();
        self.render_bottom_bar();
        self.render_cursor(COLOR_WHITE);
        loop {
            input_man.scan();
            if input_man.is_just_pressed(Key::Shift) { self.shift_pressed = !self.shift_pressed; self.render_button_states(); }
            if input_man.is_just_pressed(Key::Alpha) { self.alpha_pressed = !self.alpha_pressed; self.render_button_states(); }
            if input_man.is_just_pressed(Key::Back) || input_man.is_just_pressed(Key::Exe) { break; }
            if let Some(key) = input_man.get_last_pressed() {
                self.handle_keypress(key);
            }
            time::wait_milliseconds(20);
            self.time += 20;
            if self.time.is_multiple_of(600) {
                let cursor_color = self.cursor.color.invert();
                self.render_cursor(cursor_color);
            }
        }
        return self.content.get();
    }

    fn render_cursor(&self, color: Color565) {
        let cursor_x = ((self.cursor.pos % ROW_LENGTH) * 7).saturating_sub(1);
        let cursor_y = self.content.get_row_depth(self.cursor.row, self.cursor.pos) * ROW_HEIGHT + EDITOR_START as usize;
        push_rect_uniform(ScreenRect::new(cursor_x as u16, cursor_y as u16, 1, 12), color);
    }

    fn handle_keypress(&mut self, key: Key) {
        if let Some(c) = key.get_matching_char(self.shift_pressed, self.alpha_pressed) {
            self.content.insert(self.cursor.row, self.cursor.pos, c);
            self.cursor.pos += 1;
        } else {
            match key {
                Key::Backspace => {
                    if self.cursor.pos > 0 {
                        self.content.remove(self.cursor.row, self.cursor.pos - 1);
                        self.cursor.pos -= 1;
                    } else {
                        let old_row = self.content.remove_row(self.cursor.row);
                        self.cursor.row = self.cursor.row.saturating_sub(1);
                        self.cursor.pos = self.content.row_len(self.cursor.row);
                        self.content.row_append(self.cursor.row, &old_row);
                    }
                },
                Key::Left => {
                    if self.cursor.pos > 0 {
                        self.cursor.pos = self.cursor.pos.saturating_sub(1);
                    } else {
                        self.cursor.row = self.cursor.row.saturating_sub(1);
                        self.cursor.pos = self.content.row_len(self.cursor.row);
                    }
                },
                Key::Right => {
                    if self.cursor.pos < self.content.row_len(self.cursor.row) {
                        self.cursor.pos += 1;
                    } else if self.content.row_exists(self.cursor.row + 1) {
                        self.cursor.row += 1;
                        self.cursor.pos = 0;
                    }
                },
                Key::Up => {
                    self.cursor.row = self.cursor.row.saturating_sub(1);
                    self.cursor.pos = self.cursor.pos.min(self.content.row_len(self.cursor.row));
                },
                Key::Down => {
                    let new_row = self.cursor.row + 1;
                    if self.content.row_exists(new_row) {
                        self.cursor.row += 1;
                        self.cursor.pos = self.cursor.pos.min(self.content.row_len(self.cursor.row));
                    }
                }
                Key::Ans => {
                    let row_len = self.content.row_len(self.cursor.row);
                    let new_content = if self.cursor.pos < row_len {
                        self.content.drain_from_row(self.cursor.row, self.cursor.pos..row_len)
                    } else {
                        String::new()
                    };
                    self.content.insert_row(self.cursor.row + 1, new_content);
                    self.cursor.row += 1;
                    self.cursor.pos = 0;
                }
                _ => {}
            }
        }
        self.render_content();
        self.render_top_bar();
        self.render_bottom_bar();
        self.render_cursor(COLOR_WHITE);
        self.time = 0;
    }

    fn render_content(&self) {
        push_rect_uniform(ScreenRect::new(0, EDITOR_START, SCREEN_WIDTH, SCREEN_HEIGHT - EDITOR_START), COLOR_BLACK);

        let mut row = 0;
        let mut current_row = String::new();

        for c in self.content.get().chars() {
            if c != '\n' { current_row.push(c); }
            if current_row.len() >= ROW_LENGTH || c == '\n' {
                draw_string(&current_row, ScreenPoint::new(0, (row * ROW_HEIGHT) as u16 + EDITOR_START), false, COLOR_WHITE, COLOR_BLACK);
                row += 1;
                current_row.clear();
            }
        }
        if !current_row.is_empty() {
            draw_string(&current_row, ScreenPoint::new(0, (row * ROW_HEIGHT) as u16 + EDITOR_START), false, COLOR_WHITE, COLOR_BLACK);
        }
    }

    fn render_top_bar(&self) {
        push_rect_uniform(ScreenRect::new(0, 14, SCREEN_WIDTH, 1), COLOR_WHITE);
        let chars_str = format!("{} {}", self.content.len(), if self.content.len() == 1 { "char" } else { "chars" });
        let chars_str_x = SCREEN_WIDTH.saturating_sub((chars_str.len() * 7 - 2).try_into().unwrap()) - 7;
        push_rect_uniform(ScreenRect::new(chars_str_x.saturating_sub(20), 0, SCREEN_WIDTH - chars_str_x + 20, 14), COLOR_BLACK);
        draw_string(chars_str.as_str(), ScreenPoint::new(chars_str_x, 0), false, COLOR_WHITE, COLOR_BLACK);
    }

    fn render_bottom_bar(&self) {
        push_rect_uniform(ScreenRect::new(0, SCREEN_HEIGHT - 15, SCREEN_WIDTH, 15), COLOR_BLACK);
        push_rect_uniform(ScreenRect::new(0, SCREEN_HEIGHT - 15, SCREEN_WIDTH, 1), COLOR_WHITE);
        draw_string("   Ans: New Line      Exe: Execute Program", ScreenPoint::new(5, SCREEN_HEIGHT - 14), false, COLOR_WHITE, COLOR_BLACK);
    }

    fn render_button_states(&self) {
        let shift_fg; let shift_bg;
        let alpha_fg; let alpha_bg;

        if self.shift_pressed { shift_fg = COLOR_BLACK; shift_bg = COLOR_WHITE; } else { shift_fg = COLOR_WHITE; shift_bg = COLOR_BLACK; }
        if self.alpha_pressed { alpha_fg = COLOR_BLACK; alpha_bg = COLOR_WHITE; } else { alpha_fg = COLOR_WHITE; alpha_bg = COLOR_BLACK; }

        draw_string(" Shift ", ScreenPoint::new(0, 0), false, shift_fg, shift_bg);
        draw_string(" Alpha ", ScreenPoint::new(50, 0), false, alpha_fg, alpha_bg);
    }

    fn clear_screen(&self) {
        push_rect_uniform(ScreenRect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT), COLOR_BLACK);
    }
}

impl TextContent {
    fn new() -> Self {
        TextContent { rows: Vec::new() }
    }

    fn len(&self) -> usize {
        let mut length = 0;
        for row in &self.rows {
            length += row.len();
        }
        length
    }

    fn row_len(&self, row: usize) -> usize {
        self.rows.get(row).and_then(|r| Some(r.len())).unwrap_or(0)
    }

    fn row_exists(&self, row: usize) -> bool {
        row < self.rows.len()
    }

    fn get(&self) -> String {
        return self.rows.join("\n");
    }

    fn insert(&mut self, row: usize, pos: usize, ch: char) {
        if row >= self.rows.len() {
            self.rows.resize(row + 1, String::new());
        }
        self.rows[row].insert(pos, ch);
    }

    fn insert_row(&mut self, row: usize, content: String) {
        self.rows.insert(row, content);
    }

    fn row_append(&mut self, row: usize, content: &str) {
        if row < self.rows.len() {
            self.rows[row].push_str(content);
        }
    }

    fn remove(&mut self, row: usize, pos: usize) {
        if row < self.rows.len() {
            self.rows[row].remove(pos);
        }
    }

    fn remove_row(&mut self, row: usize) -> String {
        self.rows.remove(row)
    }

    fn get_row_depth(&self, row: usize, pos: usize) -> usize {
        let depth: usize = self.rows[..row].iter()
            .map(|s| if s.is_empty() { 1 } else { (s.len() + ROW_LENGTH - 1) / ROW_LENGTH })
            .sum();
        depth + pos / ROW_LENGTH
    }

    fn drain_from_row(&mut self, row: usize, range: Range<usize>) -> String {
        if row < self.rows.len() {
            self.rows[row].drain(range).collect()
        } else {
            String::new()
        }
    }
}

impl Default for BWColor {
    fn default() -> Self {
        BWColor { color: COLOR_WHITE }
    }
}

impl BWColor {
    fn invert(&mut self) -> Color565 {
        if self.color == COLOR_WHITE {
            self.color = COLOR_BLACK;
        } else if self.color == COLOR_BLACK {
            self.color = COLOR_WHITE;
        }
        self.color
    }
}
