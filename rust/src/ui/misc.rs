#[cfg(target_os = "none")]
use alloc::{collections::btree_set::BTreeSet, format, string::String};
use mathcore_nostd::{Expr, MathCore};

#[cfg(not(target_os = "none"))]
use std::collections::BTreeSet;

use crate::{nadk::{display::{COLOR_BLACK, COLOR_WHITE, ScreenPoint, ScreenRect, draw_string, push_rect_uniform}, keyboard::{InputManager, Key}, time}, ui::list::{SCREEN_WIDTH, StringList}};

pub fn select_var(vars: &BTreeSet<String>, input_man: &mut InputManager) -> Option<String> {
    if vars.is_empty() { return None; }
    draw_string("Select desired output", ScreenPoint::new(15, 40), false, COLOR_WHITE, COLOR_BLACK);
    let mut list = StringList::new_with_max_row_count(15, 55);
    list.clear_screen();
    for var in vars {
        list.add(var);
    }
    list.render();
    loop {
        input_man.scan();
        if input_man.is_just_pressed(Key::Down) {
            list.next();
            list.render();
        } else if input_man.is_just_pressed(Key::Up) {
            list.previous();
            list.render();
        } else if input_man.is_just_pressed(Key::Ok) {
            list.clear_screen();
            return list.get_selected().and_then(|i| Some(i.name));
        }
        time::wait_milliseconds(50);
    }
}

pub fn input_number_for(var: &str, input_man: &mut InputManager, math: &MathCore) -> Expr {
    push_rect_uniform(ScreenRect::new(15, 40, SCREEN_WIDTH - 15, 15 * 2), COLOR_BLACK);
    draw_string(format!("Input variable for {}:", var).as_str(), ScreenPoint::new(15, 40), false, COLOR_WHITE, COLOR_BLACK);
    let mut res = String::new();
    loop {
        input_man.scan();
        if let Some(last_pressed) = input_man.get_last_pressed() {
            match last_pressed.get_matching_char(false, false) {
                Some(ch) => {
                    res.push(ch);
                },
                None => {
                    match last_pressed {
                        Key::Backspace => { res.pop(); },
                        Key::Ok => break,
                        _ => {}
                    }
                }
            }
            push_rect_uniform(ScreenRect::new(15, 55, SCREEN_WIDTH - 15, 15), COLOR_BLACK);
            draw_string(res.as_str(), ScreenPoint::new(15, 55), false, COLOR_WHITE, COLOR_BLACK);
        }
        time::wait_milliseconds(20);
    }
    let resm = math.evaluate(&res);
    match resm {
        Ok(expr) => {
            return expr;
        }
        Err(e) => {
            push_rect_uniform(ScreenRect::new(15, 200, SCREEN_WIDTH - 15, 15), COLOR_BLACK);
            draw_string(format!("{}", e).as_str(), ScreenPoint::new(15, 200), false, COLOR_WHITE, COLOR_BLACK);
            return input_number_for(var, input_man, math);
        }
    }
}
