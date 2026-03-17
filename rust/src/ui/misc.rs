#[cfg(target_os = "none")]
use alloc::{collections::btree_set::BTreeSet, format, string::String, vec, vec::Vec};
use mathcore_nostd::{Expr, MathCore};

#[cfg(not(target_os = "none"))]
use std::collections::BTreeSet;

use crate::{editor::ROW_HEIGHT, nadk::{display::{COLOR_BLACK, COLOR_WHITE, ScreenPoint, ScreenRect, draw_string, pull_rect, push_rect, push_rect_uniform, push_rect_uniform_bordered}, keyboard::{InputManager, Key, wait_until_pressed_multiple}, time}, ui::list::{SCREEN_HEIGHT, SCREEN_WIDTH, StringList}};

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
    let mut text_arr: [String; 4] = [format!("Input variable for {}:", var), String::new(), String::new(), String::new()];
    loop {
        input_man.scan();
        if let Some(last_pressed) = input_man.get_last_pressed() {
            match last_pressed.get_matching_char(false, false) {
                Some(ch) => {
                    text_arr[1].push(ch);
                },
                None => {
                    match last_pressed {
                        Key::Backspace => { res.pop(); },
                        Key::Ok => break,
                        _ => {}
                    }
                }
            }
        }
        time::wait_milliseconds(20);
    }
    let resm = math.evaluate(&res);
    match resm {
        Ok(expr) => {
            return expr;
        }
        Err(e) => {
            // TODO: Error popup
            push_rect_uniform(ScreenRect::new(15, 200, SCREEN_WIDTH - 15, 15), COLOR_BLACK);
            draw_string(format!("{}", e).as_str(), ScreenPoint::new(15, 200), false, COLOR_WHITE, COLOR_BLACK);
            return input_number_for(var, input_man, math);
        }
    }
}

pub fn show_result(res: String) {
    show_text_box(&[res]);
    time::wait_milliseconds(500);
    wait_until_pressed_multiple(vec![Key::Ok, Key::Back]);
}

pub fn show_text_box(lines: &[String]) {
    push_rect_uniform_bordered(ScreenRect::new(50, 50, SCREEN_WIDTH - 100, SCREEN_HEIGHT - 100), COLOR_BLACK, COLOR_WHITE);
    for (i, line) in lines.iter().enumerate() {
        draw_string(&line, ScreenPoint::new(55, 55 + (i * ROW_HEIGHT) as u16), false, COLOR_WHITE, COLOR_BLACK);
    }
}