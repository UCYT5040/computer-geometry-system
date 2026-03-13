#[cfg(target_os = "none")]
use alloc::{string::String, format};

use crate::{list::{SCREEN_HEIGHT, SCREEN_WIDTH}, nadk::{display::{COLOR_BLACK, COLOR_RED, COLOR_WHITE, ScreenPoint, ScreenRect, draw_string, push_rect_uniform}, keyboard::{InputManager, Key}, time}};

const ROW_LENGTH: usize = 45;
const ROW_HEIGHT: usize = 15;
const EDITOR_START: u16 = 18;

pub struct TextEditor {
    content: String,
    shift_pressed: bool,
    alpha_pressed: bool
}

impl TextEditor {
    pub fn new() -> Self {
        return TextEditor { content: String::new(), shift_pressed: false, alpha_pressed: false }
    }

    pub fn start(&mut self, input_man: &mut InputManager) -> String {
        self.clear_screen();
        self.render_button_states();
        self.render_top_bar();
        self.render_cursor();
        loop {
            input_man.scan();
            if input_man.is_just_pressed(Key::Shift) { self.shift_pressed = !self.shift_pressed; self.render_button_states(); }
            if input_man.is_just_pressed(Key::Alpha) { self.alpha_pressed = !self.alpha_pressed; self.render_button_states(); }
            if input_man.is_just_pressed(Key::Backspace) { self.content.pop(); self.render_content(); }
            if input_man.is_just_pressed(Key::Back) { break; }
            if let Some(key) = input_man.get_last_pressed() {
                self.handle_keypress(key);
            }
            time::wait_milliseconds(20);
        }
        return self.content.clone();
    }

    fn render_cursor(&self) {
        let cursor_x = (self.content.len() % ROW_LENGTH) * 7 + 1;
        let cursor_y = ((self.content.len()) / ROW_LENGTH) * ROW_HEIGHT + EDITOR_START as usize;
        push_rect_uniform(ScreenRect::new(cursor_x as u16, cursor_y as u16, 2, 13), COLOR_WHITE);
    }

    fn handle_keypress(&mut self, key: Key) {
        if let Some(c) = key.get_matching_char(self.shift_pressed, self.alpha_pressed) {
            self.content.push(c);
            self.render_content();
            self.render_top_bar();
            self.render_cursor();
        }
    }

    fn render_content(&self) {
        push_rect_uniform(ScreenRect::new(0, EDITOR_START, SCREEN_WIDTH, SCREEN_HEIGHT - EDITOR_START), COLOR_BLACK);

        let mut row = 0;
        let mut current_row = String::new();

        for c in self.content.chars() {
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
        let chars_str = format!("{} chars", self.content.len());
        let chars_str_x = SCREEN_WIDTH.saturating_sub((chars_str.len() * 8 - 2).try_into().unwrap());
        push_rect_uniform(ScreenRect::new(chars_str_x, 0, SCREEN_WIDTH - chars_str_x, 14), COLOR_BLACK);
        draw_string(chars_str.as_str(), ScreenPoint::new(chars_str_x, 0), false, COLOR_WHITE, COLOR_BLACK);
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
