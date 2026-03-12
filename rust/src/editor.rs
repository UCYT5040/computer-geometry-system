#[cfg(target_os = "none")]
use alloc::string::String;

use crate::{list::{SCREEN_HEIGHT, SCREEN_WIDTH}, nadk::{display::{COLOR_BLACK, COLOR_WHITE, ScreenPoint, ScreenRect, draw_string, push_rect_uniform}, keyboard::{InputManager, Key}}};

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
        loop {
            input_man.scan();
            if input_man.is_just_pressed(Key::Shift) { self.shift_pressed = !self.shift_pressed; self.render_button_states(); }
            if input_man.is_just_pressed(Key::Alpha) { self.alpha_pressed = !self.alpha_pressed; self.render_button_states(); }
            if input_man.is_just_pressed(Key::Back) { break; }
        }
        return self.content.clone();
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
