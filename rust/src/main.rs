#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[macro_use]
mod nadk;

mod list;
use list::StringList;

#[cfg(target_os = "none")]
use alloc::format;

use crate::nadk::display::{COLOR_BLACK, COLOR_WHITE, Color565, SCREEN_RECT, ScreenPoint, draw_string, push_rect_uniform};
use crate::nadk::keyboard::{Key, KeyboardState, wait_until_pressed};
use crate::nadk::time;
use crate::nadk::utils::wait_ok_released;

// The app name must be a C string and the app name size must include the end line NULL character
configure_app!(b"CGS\0", 4, "../target/icon.nwi", 745);

// Setup the heap allocator if you need one
setup_allocator!();

#[unsafe(no_mangle)]
fn main() {
    // You must call setup_allocator!() before
    init_heap!();
    wait_ok_released();

    push_rect_uniform(SCREEN_RECT, Color565::from_rgb888(0, 0, 0));
    draw_string(
        format!("Computer Geometry System v{}", env!("CARGO_PKG_VERSION")).as_str(),
        ScreenPoint::new(15, 15),
        false,
        COLOR_WHITE,
        COLOR_BLACK,
    );

    loop {
        let scan = KeyboardState::scan();
        if scan.key_down(Key::Down) {
            draw_string("Pressed Down", ScreenPoint::new(15, 30), false, COLOR_WHITE, COLOR_BLACK);
        } else if scan.key_down(Key::Up) {
            draw_string("Pressed Up  ", ScreenPoint::new(15, 30), false, COLOR_WHITE, COLOR_BLACK);
        }
        time::wait_milliseconds(50);
    }
}
