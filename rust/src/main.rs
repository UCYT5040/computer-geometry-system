#![cfg_attr(target_os = "none", no_std)]
//#![no_std]
#![no_main]

#[macro_use]
mod nadk;

mod list;
use list::StringList;

mod tree;

#[cfg(target_os = "none")]
use alloc::format;

use crate::nadk::display::{
    draw_string, push_rect_uniform, Color565, ScreenPoint, COLOR_BLACK, COLOR_WHITE, SCREEN_RECT,
};
use crate::nadk::keyboard::{InputManager, Key};
use crate::nadk::time;
use crate::nadk::utils::wait_ok_released;

// The app name must be a C string and the app name size must include the end line NULL character
configure_app!(b"CGS\0", 4, "../target/icon.nwi", 745);

// Setup the heap allocator if you need one
setup_allocator!();

#[unsafe(no_mangle)]
fn main() {
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

    let mut menu_list = StringList::new_with_max_row_count(15, 40);
    menu_list.add("Circle");
    menu_list.add("Polygon");
    menu_list.add("Rectangle");
    menu_list.add("Square");
    menu_list.add("Triangle");
    menu_list.add("Ellipse");
    menu_list.render();

    let mut input_man = InputManager::new();

    loop {
        input_man.scan();
        if input_man.is_just_pressed(Key::Down) {
            menu_list.next();
            menu_list.render();
        } else if input_man.is_just_pressed(Key::Up) {
            menu_list.previous();
            menu_list.render();
        } else if input_man.is_just_pressed(Key::Ok) {
            menu_list.remove_current();
            menu_list.render();
        } else if input_man.is_just_pressed(Key::Home) {
            break;
        }
        time::wait_milliseconds(50);
    }
}
